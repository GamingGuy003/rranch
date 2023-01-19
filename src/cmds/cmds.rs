use std::{
    net::{Shutdown, TcpStream},
    process::exit,
    time::Duration,
};

use console::Style;
use log::{debug, error, info, trace, warn};
use termion::{clear, cursor};

use crate::{
    coms::coms::write_and_read,
    structs::{job::Job, pkgbuild::PKGBuildJson},
};

pub fn checkout_pkg(socket: &TcpStream, pkg_name: &str) {
    info!("Trying to checkout {}...", pkg_name);
    let cpkg_resp = match write_and_read(&socket, format!("CHECKOUT_PACKAGE {}", pkg_name)) {
        Ok(msg) => msg,
        Err(err) => {
            error!("{}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };

    if cpkg_resp == "INV_PKG_NAME" {
        error!("Invalid package name!");
        exit(-1)
    } else if cpkg_resp == "INV_PKG" {
        error!("The packagebuild is invalid!");
        socket
            .shutdown(Shutdown::Both)
            .unwrap_or(trace!("Failed to close socket"));
        exit(-1)
    }

    let json: PKGBuildJson = match serde_json::from_str(&cpkg_resp) {
        Ok(json) => {
            debug!("Successfully received and deserialized json from server!");
            json
        }
        Err(err) => {
            error!("Failed deserializing json received from server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };

    json.create_workdir();
    info!("Successfully checked out package {}", pkg_name);
}

pub fn submit_pkg(socket: &TcpStream, filename: &str) {
    let pkgbuild = PKGBuildJson::from_pkgbuild(filename);
    let json = serde_json::to_string(&pkgbuild).unwrap_or("".to_owned());
    if json.len() == 0 {
        error!("Failed to serialize struct! Check pkgbuild content...");
        socket
            .shutdown(Shutdown::Both)
            .unwrap_or(trace!("Failed to close socket"));
        exit(-1)
    }
    let resp = match write_and_read(socket, format!("SUBMIT_PACKAGE {}", json)) {
        Ok(resp) => {
            debug!("Successfully sent submit message");
            resp
        }
        Err(err) => {
            error!("Failed to send json to server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };
    match resp.as_str() {
        "INV_PKG_BUILD" => {
            error!("Package submission rejected by server. The package build you attempted to submit is invalid.");
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
        "CMD_OK" => {
            info!("Package submission accepted by server.");
        }
        msg => {
            error!("Received unknown message from server: {}", msg);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    }
}

pub fn submit_build(socket: &TcpStream, pkg_name: &str, cb: bool) {
    let cmd;
    if cb {
        cmd = "CROSS_BUILD";
    } else {
        cmd = "RELEASE_BUILD";
    }

    let binding = match write_and_read(socket, format!("{} {}", cmd, pkg_name)) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Encountered error while communicating with server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };
    let resp = binding.as_str();

    match resp {
        "BUILD_REQ_SUBMIT_IMMEDIATELY" => info!("The package build was immediately handled by a ready build bot."),
        "BUILD_REQ_QUEUED" => info!("No buildbot is currently available to handle the build request. Build request added to queue."),
        "INV_PKG_NAME" => {
            error!("Invalid package name!");
            socket.shutdown(Shutdown::Both).unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        },
        "PKG_BUILD_DAMAGED" => {
            error!("The packagebuild you attempted to queue is damaged.");
            socket.shutdown(Shutdown::Both).unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        },
        msg => {
            error!("Received invalid response from server: {}", msg);
            socket.shutdown(Shutdown::Both).unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    }
}

pub fn status(socket: &TcpStream) {
    let bold = Style::new().bold();
    let ital = Style::new().italic();

    //running jobs
    let resp = match write_and_read(socket, "RUNNING_JOBS_STATUS".to_owned()) {
        Ok(resp) => {
            debug!("Successfully fetched running jobs from server");
            resp
        }
        Err(err) => {
            error!("Encountered error while communicating with server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };

    let running = serde_json::from_str::<Vec<Job>>(&resp).unwrap_or(Vec::new());
    trace!("Successfully received and parsed running jobs");

    //completed jobs
    let resp = match write_and_read(socket, "COMPLETED_JOBS_STATUS".to_owned()) {
        Ok(resp) => {
            debug!("Successfully fetched completed jobs from server");
            resp
        }
        Err(err) => {
            error!("Encountered error while communicating with server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };

    let completed = serde_json::from_str::<Vec<Job>>(&resp).unwrap_or(Vec::new());
    trace!("Successfully received and parsed completed jobs");

    //queued jobs
    let resp = match write_and_read(socket, "QUEUED_JOBS_STATUS".to_owned()) {
        Ok(resp) => {
            debug!("Successfully fetched queued jobs from server");
            resp
        }
        Err(err) => {
            error!("Encountered error while communicating with server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };

    let queued = serde_json::from_str::<Vec<Job>>(&resp).unwrap_or(Vec::new());
    trace!("Successfully received and parsed queued jobs");

    println!("{}", bold.apply_to("RUNNING JOBS"));
    if running.len() == 0 {
        println!("No jobs.");
    } else {
        println!(
            "{}",
            ital.apply_to(format!(
                "{:<20} {:<15} {:<40} {:10}",
                "NAME", "STATUS", "ID", "REQUESTED BY"
            ))
        );
        for job in running {
            println!("{}", job.to_string());
        }
    }

    println!("{}", bold.apply_to("COMPLETED JOBS"));
    if completed.len() == 0 {
        println!("No jobs.");
    } else {
        println!(
            "{}",
            ital.apply_to(format!(
                "{:<20} {:<15} {:<40} {:10}",
                "NAME", "STATUS", "ID", "REQUESTED BY"
            ))
        );
        for job in completed {
            println!("{}", job.to_string());
        }
    }

    println!("{}", bold.apply_to("QUEUED JOBS"));
    if queued.len() == 0 {
        println!("No jobs.");
    } else {
        println!(
            "{}",
            ital.apply_to(format!(
                "{:<20} {:<15} {:<40} {:10}",
                "NAME", "STATUS", "ID", "REQUESTED BY"
            ))
        );
        for job in queued {
            println!("{}", job.to_string());
        }
    }
}

pub fn client_status(socket: &TcpStream) {
    let bold = Style::new().bold();

    let conn = match write_and_read(socket, "CONNECTED_CONTROLLERS".to_owned()) {
        Ok(resp) => {
            debug!("Retrieved connected controllers");
            resp
        }
        Err(err) => {
            error!("Error while requesting connected controllers: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };
    let bots = match write_and_read(socket, "CONNECTED_BUILDBOTS".to_owned()) {
        Ok(resp) => {
            debug!("Retrieved connected buildbots");
            resp
        }
        Err(err) => {
            error!("Error while requesting connected buildbots: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };

    debug!("Trying to deserialize...");
    let connlist: Vec<String> = serde_json::from_str(conn.as_str()).unwrap_or(Vec::new());
    debug!("Connlist was: {:?}", connlist);
    let botslist: Vec<String> = serde_json::from_str(bots.as_str()).unwrap_or(Vec::new());
    debug!("Botslist was: {:?}", botslist);

    println!("{}", bold.apply_to("CONNECTED CLIENTS"));
    if connlist.len() > 0 {
        for c in connlist {
            println!("{}", c);
        }
    } else {
        println!("No clients connected.");
    }

    println!("{}", bold.apply_to("CONNECTED BUILDBOTS"));
    if botslist.len() > 0 {
        for b in botslist {
            println!("{}", b);
        }
    } else {
        println!("No buildbots connected.");
    }
}

pub fn view_log(socket: &TcpStream, job_id: &str) {
    let bold = Style::new().bold();

    let log = match write_and_read(socket, format!("VIEW_LOG {}", job_id)) {
        Ok(log) => {
            debug!("Successfully received log msg for {}", job_id);
            log
        }
        Err(err) => {
            error!("Error while retrieving log msg: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };

    let log_as_lines: Vec<String> = serde_json::from_str(log.as_str()).unwrap_or(Vec::new());
    println!("{}", bold.apply_to(format!("Buildlog for {}:\n", job_id)));
    if log_as_lines.len() > 0 {
        for line in log_as_lines {
            println!("{}", line);
        }
    } else {
        println!("Log was empty.");
    }
}

pub fn cancel_queued_job(socket: &TcpStream, job_id: &str) {
    let resp = match write_and_read(socket, format!("CANCEL_QUEUED_JOB {}", job_id)) {
        Ok(resp) => {
            debug!(
                "Successfully fetched response for CANCEL_QUEUD_JOB: {}",
                resp
            );
            resp
        }
        Err(err) => {
            error!("Error communicating with server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };

    match resp.as_str() {
        "INV_JOB_ID" => {
            error!("No such job queued");
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
        "JOB_CANCELED" => info!("Successfully canceled job {}", job_id),
        msg => {
            error!("Received unknown response: {}", msg);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    }
}

pub fn clear_completed_jobs(socket: &TcpStream) {
    let resp = match write_and_read(socket, "CLEAR_COMPLETED_JOBS".to_owned()) {
        Ok(resp) => {
            debug!(
                "Successfully fetched response for CLEAR_COMPLETED_JOBS: {}",
                resp
            );
            resp
        }
        Err(err) => {
            error!("Error communicating with server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };

    if resp.as_str() != "JOBS_CLEARED" {
        error!("Failed to clear completed jobs: {}", resp);
        socket
            .shutdown(Shutdown::Both)
            .unwrap_or(trace!("Failed to close socket"));
        exit(-1)
    } else {
        info!("Successfully cleared jobs");
    }
}

pub fn cancel_all_jobs(socket: &TcpStream) {
    let resp = match write_and_read(socket, "CANCEL_ALL_QUEUED_JOBS".to_owned()) {
        Ok(resp) => {
            debug!(
                "Successfully fetched response for CANCEL_ALL_QUEUD_JOBS: {}",
                resp
            );
            resp
        }
        Err(err) => {
            error!("Error communicating with server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };

    if resp.as_str() != "JOBS_CANCELED" {
        error!("Failed to cancel all queued jobs: {}", resp);
        socket
            .shutdown(Shutdown::Both)
            .unwrap_or(trace!("Failed to close socket"));
        exit(-1)
    } else {
        info!("Successfully cancelled all queued jobs");
    }
}

pub fn managed_pkgs(socket: &TcpStream) {
    let bold = Style::new().bold();

    let resp = match write_and_read(socket, "MANAGED_PACKAGES".to_owned()) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Error communicating with server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };
    let mut pkgs: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or(Vec::new());
    debug!("Successfully fetched managed pkgs from server: {:?}", pkgs);

    println!("{}", bold.apply_to("Managed packages:"));
    if pkgs.len() > 0 {
        pkgs.sort();
        print_vec_cols(pkgs);
    } else {
        println!("No managed packages on server");
    }
}

pub fn managed_pkg_builds(socket: &TcpStream) {
    let bold = Style::new().bold();

    let resp = match write_and_read(socket, "MANAGED_PKGBUILDS".to_owned()) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Error communicating with server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };
    let mut pkgb: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or(Vec::new());
    debug!(
        "Successfully fetched managed pkgbuilds from server: {:?}",
        pkgb
    );

    println!("{}", bold.apply_to("Managed packageuilds:"));
    if pkgb.len() > 0 {
        pkgb.sort();
        print_vec_cols(pkgb);
    } else {
        println!("No managed packagebuilds on server");
    }
}

pub fn diff_pkgs(socket: &TcpStream) {
    let red = Style::new().red();
    let green = Style::new().green();
    let bold = Style::new().bold();

    let resp = match write_and_read(socket, "MANAGED_PACKAGES".to_owned()) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Error communicating with server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };
    let pkgs: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or(Vec::new());
    debug!("Successfully fetched managed pkgs from server: {:?}", pkgs);

    let resp = match write_and_read(socket, "MANAGED_PKGBUILDS".to_owned()) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Error communicating with server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };
    let mut pkgb: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or(Vec::new());
    debug!(
        "Successfully fetched managed pkgbuilds from server: {:?}",
        pkgb
    );
    pkgb.sort();

    let mut diff = Vec::new();
    for pbuild in pkgb {
        if pkgs.contains(&pbuild) {
            diff.push(format!("{}", green.apply_to(pbuild)));
        } else {
            diff.push(format!("{}", red.apply_to(pbuild)));
        }
    }

    println!("{}", bold.apply_to("Package / Packageuild diff:"));
    if diff.len() > 0 {
        print_vec_cols(diff);
    } else {
        println!("No managed packagebuilds on server");
    }
}

fn print_vec_cols(vec: Vec<String>) {
    let elem_width = vec.iter().max_by_key(|s| s.chars().count()).unwrap_or(&"".to_owned()).chars().count() + 5;
    println!("max len: {}", elem_width);
    let colcount = (termion::terminal_size().unwrap_or((0, 0)).0 / elem_width as u16) as usize;
    for (idx, val) in vec.into_iter().enumerate() {
        if idx % colcount == 0 && idx != 0 {
            println!();
        }
        print!("{:<1$}", val, elem_width);
    }
    println!();
}

pub fn view_sys_log(socket: &TcpStream) {
    let bold = Style::new().bold();

    let log = match write_and_read(socket, "VIEW_SYS_EVENTS".to_owned()) {
        Ok(log) => {
            debug!("Successfully received sys log from master");
            log
        }
        Err(err) => {
            error!("Error while retrieving sys log: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };
    let log_as_lines: Vec<String> = serde_json::from_str(log.as_str()).unwrap_or(Vec::new());
    println!("{}", bold.apply_to("Syslog:"));
    if log_as_lines.len() > 0 {
        for line in log_as_lines {
            println!("{}", line);
        }
    } else {
        println!("Syslog is empty.");
    }
}

pub fn view_dependers(socket: &TcpStream, pkg_name: &str) {
    let bold = Style::new().bold();

    let resp = match write_and_read(socket, format!("GET_DEPENDERS {}", pkg_name)) {
        Ok(resp) => resp,
        Err(err) => {
            error!(
                "Error while fetching dependency tree for {}:{}",
                pkg_name, err
            );
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };

    if resp == "INV_PKG_NAME" {
        error!("Invalid package name!");
        socket
            .shutdown(Shutdown::Both)
            .unwrap_or(trace!("Failed to close socket"));
        exit(-1)
    }

    println!("{}", bold.apply_to(format!("Dependers on {}:", pkg_name)));
    print_vec_cols(serde_json::from_str::<Vec<String>>(&resp).unwrap_or(Vec::new()));
}

pub fn rebuild_dependers(socket: &TcpStream, pkg_name: &str) {
    let resp = match write_and_read(socket, format!("REBUILD_DEPENDERS {}", pkg_name)) {
        Ok(resp) => {
            debug!("Successfully requested dependers rebuild for {}", pkg_name);
            resp
        }
        Err(err) => {
            error!("Error while communicating with server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };

    match resp.as_str() {
        "INV_PKG_NAME" => {
            error!("No such package available");
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
        "CIRCULAR_DEPENDENCY" => {
            error!("Circular dependency detected. The requested batch build contains a circular dependency and could not be submitted.");
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
        "BATCH_QUEUED" => {
            info!("Successfully queued batch");
        }
        msg => {
            error!("Received unknown response from server: {}", msg);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    }
}

pub fn submit_solution(socket: &TcpStream, filename: &str, cb: bool) {
    let cmd;
    if cb {
        cmd = "SUBMIT_SOLUTION_CB";
    } else {
        cmd = "SUBMIT_SOLUTION_RB";
    }

    let file = match std::fs::read_to_string(filename) {
        Ok(file) => {
            debug!("Successfully read sol file {}", filename);
            file
        }
        Err(err) => {
            error!("Error reading sol file: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };

    let mut ret: Vec<Vec<String>> = Vec::new();
    file.lines().for_each(|line| {
        ret.push(
            line.split(";")
                .into_iter()
                .map(|value| value.to_owned())
                .collect(),
        )
    });
    let resp = match write_and_read(socket, format!("{} {:?}", cmd, ret)) {
        Ok(resp) => {
            debug!("Server responded with: {}", resp);
            resp
        }
        Err(err) => {
            error!("Error while communicating with server: {}", err);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    };

    match resp.as_str() {
        s if s.starts_with("PKG_BUILD_MISSING") => {
            error!(
                "Missing packagebuild on server: {}",
                s.splitn(2, " ").collect::<Vec<&str>>()[1]
            );
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
        "BATCH_QUEUED" => info!("Successfully queued solutionfile!"),
        msg => {
            error!("Received unknown response from server: {}", msg);
            socket
                .shutdown(Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
            exit(-1)
        }
    }
}

pub fn create_template() {
    PKGBuildJson::new_template().create_workdir();
}

pub fn watch_jobs(socket: &TcpStream, interval: &str) {
    let n = interval.parse::<u64>().unwrap_or_else(|_| {
        warn!("Failed converting interval to u64; falling back to 5 secs");
        0
    });
    let mut i: u128 = 0;
    loop {
        i += 1;
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        status(socket);
        info!("Update: {}", i);
        std::thread::sleep(Duration::from_secs(n));
    }
}
