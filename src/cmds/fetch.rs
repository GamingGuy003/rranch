use std::{
    net::TcpStream,
    process::exit,
};

use console::Style;
use log::{debug, error, info, trace};

use crate::{
    coms::coms::write_and_read, structs::pkgbuild::PKGBuildJson, util::util::print_vec_cols,
};

pub fn fetch_dependencies_for(socket: &TcpStream, pkg_name: &str) -> i32 {
    info!("Trying to show dependencies of {}...", pkg_name);
    let cpkg_resp = match write_and_read(&socket, format!("CHECKOUT_PACKAGE {}", pkg_name)) {
        Ok(msg) => msg,
        Err(err) => {
            error!("{}", err);
            return -1;
        }
    };

    if cpkg_resp == "INV_PKG_NAME" {
        error!("Invalid package name!");
        exit(-1)
    } else if cpkg_resp == "INV_PKG" {
        error!("The packagebuild is invalid!");
        return -1;
    }

    let json: PKGBuildJson = match serde_json::from_str(&cpkg_resp) {
        Ok(json) => {
            debug!("Successfully received and deserialized json from server!");
            json
        }
        Err(err) => {
            error!("Failed deserializing json received from server: {}", err);
            return -1;
        }
    };

    let resp = match write_and_read(socket, "MANAGED_PACKAGES".to_owned()) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Error communicating with server: {}", err);
            return -1;
        }
    };

    let pkgs: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or(Vec::new());

    let resp = match write_and_read(socket, "MANAGED_PKGBUILDS".to_owned()) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Error communicating with server: {}", err);
            return -1;
        }
    };

    let pkgb: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or(Vec::new());

    let deps = json.get_dependencies();
    let bdeps = json.get_build_dependencies();
    let cdeps = json.get_cross_dependencies();

    let bold = Style::new().bold();
    //dep in neither
    let red = Style::new().red();
    //dep only in pkb
    let yellow = Style::new().yellow();
    //dep in pkg && pkb
    let green = Style::new().green();

    let mut diffdeps: Vec<String> = Vec::new();
    let mut diffbdeps: Vec<String> = Vec::new();
    let mut diffcdeps: Vec<String> = Vec::new();
    for dep in deps.clone() {
        if pkgb.contains(&dep) {
            if pkgs.contains(&dep) {
                diffdeps.push(format!("{}", green.apply_to(dep)));
            } else {
                diffdeps.push(format!("{}", yellow.apply_to(dep)));
            }
        } else {
            diffdeps.push(format!("{}", red.apply_to(dep)));
        }
    }

    for dep in bdeps.clone() {
        if pkgb.contains(&dep) {
            if pkgs.contains(&dep) {
                diffbdeps.push(format!("{}", green.apply_to(dep)));
            } else {
                diffbdeps.push(format!("{}", yellow.apply_to(dep)));
            }
        } else {
            diffbdeps.push(format!("{}", red.apply_to(dep)));
        }
    }

    for dep in cdeps.clone() {
        if pkgb.contains(&dep) {
            if pkgs.contains(&dep) {
                diffcdeps.push(format!("{}", green.apply_to(dep)));
            } else {
                diffcdeps.push(format!("{}", yellow.apply_to(dep)));
            }
        } else {
            diffcdeps.push(format!("{}", red.apply_to(dep)));
        }
    }

    let maxdeps = Some(
        (deps
            .iter()
            .max_by_key(|s| s.chars().count())
            .unwrap_or(&"".to_owned())
            .chars()
            .count()
            + 5) as i32,
    );
    println!(
        "{}",
        bold.apply_to(format!("Dependencies for {}:", pkg_name))
    );
    if diffdeps.len() > 0 {
        print_vec_cols(diffdeps, maxdeps, 8);
    } else {
        println!("No runtimedependencies.");
    }

    let maxbdeps = Some(
        (bdeps
            .iter()
            .max_by_key(|s| s.chars().count())
            .unwrap_or(&"".to_owned())
            .chars()
            .count()
            + 5) as i32,
    );
    println!(
        "{}",
        bold.apply_to(format!("Builddependencies for {}:", pkg_name))
    );
    if diffbdeps.len() > 0 {
        print_vec_cols(diffbdeps, maxbdeps, 8);
    } else {
        println!("No builddependencies.");
    }

    let maxcdeps = Some(
        (cdeps
            .iter()
            .max_by_key(|s| s.chars().count())
            .unwrap_or(&"".to_owned())
            .chars()
            .count()
            + 5) as i32,
    );
    println!(
        "{}",
        bold.apply_to(format!("Crossdependencies for {}:", pkg_name))
    );
    if diffcdeps.len() > 0 {
        print_vec_cols(diffcdeps, maxcdeps, 8);
    } else {
        println!("No crossdependencies.");
    }
    0
}

pub fn fetch_dependers_on(socket: &TcpStream, pkg_name: &str) -> i32 {
    let bold = Style::new().bold();

    let resp = match write_and_read(socket, format!("GET_DEPENDERS {}", pkg_name)) {
        Ok(resp) => resp,
        Err(err) => {
            error!(
                "Error while fetching dependency tree for {}:{}",
                pkg_name, err
            );
            return -1;
        }
    };

    if resp == "INV_PKG_NAME" {
        error!("Invalid package name!");
        return -1;
    }

    println!("{}", bold.apply_to(format!("\nDependers on {}:", pkg_name)));
    print_vec_cols(
        serde_json::from_str::<Vec<String>>(&resp).unwrap_or(Vec::new()),
        None,
        0,
    );
    0
}

pub fn fetch_sys_log(socket: &TcpStream) -> i32 {
    let bold = Style::new().bold();

    let log = match write_and_read(socket, "VIEW_SYS_EVENTS".to_owned()) {
        Ok(log) => {
            debug!("Successfully received sys log from master");
            log
        }
        Err(err) => {
            error!("Error while retrieving sys log: {}", err);
            return -1;
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
    0
}

pub fn fetch_difference_pkgb_pkgs(socket: &TcpStream) -> i32 {
    let red = Style::new().red();
    let green = Style::new().green();
    let bold = Style::new().bold();

    let resp = match write_and_read(socket, "MANAGED_PACKAGES".to_owned()) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Error communicating with server: {}", err);
            return -1;
        }
    };
    let pkgs: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or(Vec::new());

    let resp = match write_and_read(socket, "MANAGED_PKGBUILDS".to_owned()) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Error communicating with server: {}", err);
            return -1;
        }
    };
    let mut pkgb: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or(Vec::new());
    pkgb.sort();

    let mut diff = Vec::new();
    let max;
    if !pkgb.is_empty() {
        max = Some(
            (pkgb
                .iter()
                .max_by_key(|value| value.chars().count())
                .unwrap_or(&"default_with_some_length".to_owned())
                .chars()
                .count()
                + 13) as i32,
        );
    } else {
        max = Some(
            (pkgs
                .iter()
                .max_by_key(|value| value.chars().count())
                .unwrap_or(&"default_with_some_length".to_owned())
                .chars()
                .count()
                + 13) as i32,
        );
    }

    for pbuild in pkgb {
        if pkgs.contains(&pbuild) {
            diff.push(format!("{}", green.apply_to(pbuild)));
        } else {
            diff.push(format!("{}", red.apply_to(pbuild)));
        }
    }

    println!("{}", bold.apply_to("Package / Packageuild diff:"));
    if diff.len() > 0 {
        print_vec_cols(diff, max, 0);
    } else {
        println!("No managed packagebuilds on server");
    }
    0
}

pub fn fetch_managed_packagebuilds(socket: &TcpStream) -> i32 {
    let bold = Style::new().bold();

    let resp = match write_and_read(socket, "MANAGED_PKGBUILDS".to_owned()) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Error communicating with server: {}", err);
            return -1;
        }
    };
    let mut pkgb: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or(Vec::new());

    println!("{}", bold.apply_to("Managed packageuilds:"));
    if pkgb.len() > 0 {
        pkgb.sort();
        print_vec_cols(pkgb, None, 0);
    } else {
        println!("No managed packagebuilds on server");
    }
    0
}

pub fn fetch_managed_packages(socket: &TcpStream) -> i32 {
    let bold = Style::new().bold();

    let resp = match write_and_read(socket, "MANAGED_PACKAGES".to_owned()) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Error communicating with server: {}", err);
            return -1;
        }
    };
    let mut pkgs: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or(Vec::new());
    debug!("Successfully fetched managed pkgs from server: {:?}", pkgs);

    println!("{}", bold.apply_to("Managed packages:"));
    if pkgs.len() > 0 {
        pkgs.sort();
        print_vec_cols(pkgs, None, 0);
    } else {
        println!("No managed packages on server");
    }
    0
}

pub fn fetch_log_of(socket: &TcpStream, job_id: &str) -> i32 {
    let bold = Style::new().bold();

    let log = match write_and_read(socket, format!("VIEW_LOG {}", job_id)) {
        Ok(log) => {
            debug!("Successfully received log msg for {}", job_id);
            log
        }
        Err(err) => {
            error!("Error while retrieving log msg: {}", err);
            return -1;
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
    0
}

pub fn fetch_client_status(socket: &TcpStream) -> i32 {
    let bold = Style::new().bold();

    let conn = match write_and_read(socket, "CONNECTED_CONTROLLERS".to_owned()) {
        Ok(resp) => {
            debug!("Retrieved connected controllers");
            resp
        }
        Err(err) => {
            error!("Error while requesting connected controllers: {}", err);
            return -1;
        }
    };
    let bots = match write_and_read(socket, "CONNECTED_BUILDBOTS".to_owned()) {
        Ok(resp) => {
            debug!("Retrieved connected buildbots");
            resp
        }
        Err(err) => {
            error!("Error while requesting connected buildbots: {}", err);
            return -1;
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
    0
}

pub fn fetch_packagebuild_for(socket: &TcpStream, pkg_name: &str) -> i32 {
    info!("Trying to checkout {}...", pkg_name);
    let cpkg_resp = match write_and_read(&socket, format!("CHECKOUT_PACKAGE {}", pkg_name)) {
        Ok(msg) => msg,
        Err(err) => {
            error!("{}", err);
            return -1;
        }
    };

    if cpkg_resp == "INV_PKG_NAME" {
        error!("Invalid package name!");
        return -1;
    } else if cpkg_resp == "INV_PKG" {
        error!("The packagebuild is invalid!");
        return -1;
    }

    let json: PKGBuildJson = match serde_json::from_str(&cpkg_resp) {
        Ok(json) => {
            debug!("Successfully received and deserialized json from server!");
            json
        }
        Err(err) => {
            error!("Failed deserializing json received from server: {}", err);
            return -1;
        }
    };

    json.create_workdir();
    0
}
