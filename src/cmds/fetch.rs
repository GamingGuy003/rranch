use console::Style;
use log::{debug, error, info};
use std::{io::copy, net::TcpStream};

use crate::{
    sockops::coms::write_and_read, structs::pkgbuild::PKGBuildJson, util::funcs::print_vec_cols,
};

pub fn fetch_dependencies_for(socket: &TcpStream, pkg_name: &str) -> i32 {
    info!("Trying to show dependencies of {}...", pkg_name);
    let dependencies = match write_and_read(socket, format!("CHECKOUT_PACKAGE {}", pkg_name)) {
        Ok(msg) => msg,
        Err(err) => {
            error!("{}", err);
            return -1;
        }
    };

    match dependencies.as_str() {
        "INV_PKG_NAME" => {
            error!("Invalid package name!");
            return -1;
        }
        "INV_PKG" => {
            error!("Invalid packagebuild!");
            return -1;
        }
        _ => {}
    }

    let packagebuild: PKGBuildJson = match serde_json::from_str(&dependencies) {
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

    let pkgs: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or_default();

    let resp = match write_and_read(socket, "MANAGED_PKGBUILDS".to_owned()) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Error communicating with server: {}", err);
            return -1;
        }
    };

    let pkgb: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or_default();

    let deps = packagebuild.get_dependencies();
    let bdeps = packagebuild.get_build_dependencies();
    let cdeps = packagebuild.get_cross_dependencies();

    let bold = Style::new().bold(); //title
    let red = Style::new().red(); //dependencies not built nor with packagebuild
    let yellow = Style::new().yellow(); //dependencies with packagebuild but not built
    let green = Style::new().green(); //dependencies build with packagebuild

    let mut diffdeps: Vec<String> = Vec::new();
    let mut diffbdeps: Vec<String> = Vec::new();
    let mut diffcdeps: Vec<String> = Vec::new();

    //adds colors to dependency vector
    for dep in deps.clone() {
        if pkgb.contains(&dep) {
            if pkgs.contains(&dep) {
                diffdeps.push(format!("{}", green.apply_to(dep))); //packagebuild and binary
            } else {
                diffdeps.push(format!("{}", yellow.apply_to(dep))); //packagebuild no binary
            }
        } else {
            diffdeps.push(format!("{}", red.apply_to(dep))); //no packagebuild and no binary
        }
    }

    //adds color to build dependecy vector
    for dep in bdeps.clone() {
        if pkgb.contains(&dep) {
            if pkgs.contains(&dep) {
                diffbdeps.push(format!("{}", green.apply_to(dep))); //packagebuild and binary
            } else {
                diffbdeps.push(format!("{}", yellow.apply_to(dep))); //packagebuild no binary
            }
        } else {
            diffbdeps.push(format!("{}", red.apply_to(dep))); //no packagebuild and no binary
        }
    }

    //adds color to crossbuild dependency vector
    for dep in cdeps.clone() {
        if pkgb.contains(&dep) {
            if pkgs.contains(&dep) {
                diffcdeps.push(format!("{}", green.apply_to(dep))); //packagebuild and binary
            } else {
                diffcdeps.push(format!("{}", yellow.apply_to(dep))); //packagebuild no binary
            }
        } else {
            diffcdeps.push(format!("{}", red.apply_to(dep))); //no packagebuild no binary
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
    if diffdeps.is_empty() {
        println!("No runtimedependencies.");
    } else {
        print_vec_cols(diffdeps, maxdeps, 8);
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
    if diffbdeps.is_empty() {
        println!("No builddependencies.");
    } else {
        print_vec_cols(diffbdeps, maxbdeps, 8);
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
    if diffcdeps.is_empty() {
        println!("No crossdependencies.");
    } else {
        print_vec_cols(diffcdeps, maxcdeps, 8);
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

    println!("{}", bold.apply_to(format!("Dependers on {}:", pkg_name)));
    print_vec_cols(
        serde_json::from_str::<Vec<String>>(&resp).unwrap_or_default(),
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
    let log_as_lines: Vec<String> = serde_json::from_str(log.as_str()).unwrap_or_default();
    println!("{}", bold.apply_to("Syslog:"));
    if log_as_lines.is_empty() {
        println!("Syslog is empty.");
    } else {
        for line in log_as_lines {
            println!("{}", line);
        }
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
    let pkgs: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or_default();

    let resp = match write_and_read(socket, "MANAGED_PKGBUILDS".to_owned()) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Error communicating with server: {}", err);
            return -1;
        }
    };
    let mut pkgb: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or_default();
    pkgb.sort();

    let mut diff = Vec::new();
    let max = if pkgb.is_empty() {
        Some(
            (pkgb
                .iter()
                .max_by_key(|value| value.chars().count())
                .unwrap_or(&"default_with_some_length".to_owned())
                .chars()
                .count()
                + 13) as i32,
        )
    } else {
        Some(
            (pkgs
                .iter()
                .max_by_key(|value| value.chars().count())
                .unwrap_or(&"default_with_some_length".to_owned())
                .chars()
                .count()
                + 13) as i32,
        )
    };

    for pbuild in pkgb {
        if pkgs.contains(&pbuild) {
            diff.push(format!("{}", green.apply_to(pbuild)));
        } else {
            diff.push(format!("{}", red.apply_to(pbuild)));
        }
    }

    println!("{}", bold.apply_to("Package / Packageuild diff:"));
    if diff.is_empty() {
        println!("No managed packagebuilds on server");
    } else {
        print_vec_cols(diff, max, 0);
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
    let mut pkgb: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or_default();

    println!("{}", bold.apply_to("Managed packageuilds:"));
    if pkgb.is_empty() {
        println!("No managed packagebuilds on server");
    } else {
        pkgb.sort();
        print_vec_cols(pkgb, None, 0);
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
    let mut pkgs: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or_default();
    debug!("Successfully fetched managed pkgs from server: {:?}", pkgs);

    println!("{}", bold.apply_to("Managed packages:"));
    if pkgs.is_empty() {
        println!("No managed packages on server");
    } else {
        pkgs.sort();
        print_vec_cols(pkgs, None, 0);
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

    let log_as_lines: Vec<String> = serde_json::from_str(log.as_str()).unwrap_or_default();
    println!("{}", bold.apply_to(format!("Buildlog for {}:\n", job_id)));
    if log_as_lines.is_empty() {
        println!("Log was empty.");
    } else {
        for line in log_as_lines {
            println!("{}", line);
        }
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
    let connlist: Vec<String> = serde_json::from_str(conn.as_str()).unwrap_or_default();
    debug!("Connlist was: {:?}", connlist);
    let botslist: Vec<String> = serde_json::from_str(bots.as_str()).unwrap_or_default();
    debug!("Botslist was: {:?}", botslist);

    println!("{}", bold.apply_to("CONNECTED CLIENTS"));
    if connlist.is_empty() {
        println!("No clients connected.");
    } else {
        for c in connlist {
            println!("{}", c);
        }
    }

    println!("{}", bold.apply_to("CONNECTED BUILDBOTS"));
    if !botslist.is_empty() {
        println!("No buildbots connected.");
    } else {
        for b in botslist {
            println!("{}", b);
        }
    }
    0
}

pub fn fetch_packagebuild_for(socket: &TcpStream, pkg_name: &str) -> i32 {
    info!("Trying to checkout {}...", pkg_name);
    let cpkg_resp = match write_and_read(socket, format!("CHECKOUT_PACKAGE {}", pkg_name)) {
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

pub fn fetch_package(api_url: &str, pkg_name: &str) -> i32 {
    let url = format!("https://{}?get=package&pkgname={}", api_url, pkg_name);
    debug!("Trying to download {}", url);
    let mut response = match reqwest::blocking::get(url) {
        Ok(response) => {
            if response.content_length().unwrap_or(0) == 0 {
                error!("Invalid packagename or link. Length was 0");
                return -1;
            }
            info!(
                "Downloading file with size {}",
                response.content_length().unwrap_or(0)
            );
            response
        }
        Err(_) => {
            error!(
                "Failed to fetch file from {}. (Does the package exist?)",
                api_url
            );
            return -1;
        }
    };

    let mut out = match std::fs::File::create(format!("{}.tar.xz", pkg_name)) {
        Ok(file) => file,
        Err(err) => {
            error!("Failed creating output file {}.tar.xz: {}", pkg_name, err);
            return -1;
        }
    };
    match copy(&mut response, &mut out) {
        Ok(_) => debug!("Successfully copied content to file."),
        Err(err) => {
            error!("Failed writing content to file: {}", err);
            return -1;
        }
    }
    0
}
