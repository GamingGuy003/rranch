use cmds::{
    fetch::{
        fetch_client_status, fetch_dependencies_for, fetch_dependers_on,
        fetch_difference_pkgb_pkgs, fetch_log_of, fetch_managed_packagebuilds,
        fetch_managed_packages, fetch_packagebuild_for, fetch_sys_log,
    },
    job::{
        request_build, request_cancel_all_jobs, request_cancel_queued_job,
        request_clear_completed_jobs, request_rebuild_dependers, request_status,
    },
    other::{create_template, edit, latest_log, watch_jobs},
    submit::{submit_packagebuild, submit_solution},
};
use conn::conn::connect;
use dbs::dbs::run_dbs;
use log::{debug, error, trace};
use std::process::exit;
use structs::config::{Client, Config, Master};

use crate::args::args::ArgParser;
mod args;
mod cmds;
mod coms;
mod conn;
mod dbs;
mod structs;
mod util;

fn main() -> std::io::Result<()> {
    //init env logger
    let conf = Config::new_from_cfg("rranch.toml");
    //set loglevel
    let mut loglevel = "";
    match conf
        .client
        .as_ref()
        .unwrap_or(&Client::empty())
        .loglevel
        .clone()
        .unwrap_or("".to_owned())
        .to_lowercase()
        .as_str()
    {
        "trace" => loglevel = "trace",
        "debug" => loglevel = "debug",
        "info" => loglevel = "info",
        "none" => {}
        _ => {}
    }
    std::env::set_var("rranch_log", loglevel);
    pretty_env_logger::init_custom_env("rranch_log");
    let mut argparser = ArgParser::new();
    //try to fetch arguments from cli and parse them
    match argparser.args() {
        Ok(_) => debug!("Got args: {:?}", argparser.funcs()),
        Err(err) => {
            error!("Could not get arguments: {}", err);
            argparser.help();
            exit(-1);
        }
    };

    //get arg array and connect
    let funcs = argparser.funcs();
    let socket = connect(
        conf.master
            .as_ref()
            .unwrap_or(&Master::empty())
            .addr
            .clone()
            .unwrap_or("localhost".to_owned())
            .as_str(),
        conf.master
            .as_ref()
            .unwrap_or(&Master::empty())
            .port
            .clone()
            .unwrap_or(27015)
            .to_string()
            .as_str(),
        format!(
            "{}",
            conf.client
                .as_ref()
                .unwrap_or(&Client::empty())
                .name
                .clone()
                .unwrap_or("a-rranch-client".to_owned())
                .clone()
        )
        .as_str(),
        conf.master
            .as_ref()
            .unwrap_or(&Master::empty())
            .authkey
            .clone()
            .unwrap_or("default".to_owned())
            .as_str(),
        conf.client
            .as_ref()
            .unwrap_or(&Client::empty())
            .r#type
            .clone()
            .unwrap_or("CONTROLLER".to_owned())
            .as_str(),
    );
    if funcs.len() == 0 {
        error!("No arguments have been provided!");
        argparser.help();
        exit(0)
    }

    let editor = conf
        .client
        .unwrap_or(Client::empty())
        .editor
        .unwrap_or("".to_owned());
    //work out which function to execute
    for func in funcs {
        let fmatch = (func.0.as_str(), func.1);
        match fmatch {
            ("--debugshell", _) => run_dbs(&socket),
            ("--checkout", name) => fetch_packagebuild_for(&socket, &name.unwrap_or("".to_owned())),
            ("--edit", name) => edit(&socket, &name.unwrap_or("".to_owned()), editor.as_str()),
            ("--template", _) => create_template(),
            ("--submit", filename) => {
                submit_packagebuild(&socket, &filename.unwrap_or("".to_owned()))
            }
            ("--releasebuild", name) => {
                request_build(&socket, &name.unwrap_or("".to_owned()), false)
            }
            ("--crossbuild", name) => request_build(&socket, &name.unwrap_or("".to_owned()), true),
            ("--viewlog", job_id) => fetch_log_of(&socket, &job_id.unwrap_or("".to_owned())),
            ("--viewlastlog", _) => latest_log(&socket),
            ("--status", _) => request_status(&socket, false),
            ("--watchjobs", interval) => watch_jobs(&socket, &interval.unwrap_or("".to_owned())),
            ("--clientstatus", _) => fetch_client_status(&socket),
            ("--clearjobs", _) => request_clear_completed_jobs(&socket),
            ("--cancelalljobs", _) => request_cancel_all_jobs(&socket),
            ("--canceljob", job_id) => {
                request_cancel_queued_job(&socket, &job_id.unwrap_or("".to_owned()))
            }
            ("--managedpkgs", _) => fetch_managed_packages(&socket),
            ("--managedpkgbuilds", _) => fetch_managed_packagebuilds(&socket),
            ("--differencepkgs", _) => fetch_difference_pkgb_pkgs(&socket),
            ("--viewsyslog", _) => fetch_sys_log(&socket),
            ("--viewdependers", name) => {
                fetch_dependers_on(&socket, &name.unwrap_or("".to_owned()))
            }
            ("--viewdependencies", name) => {
                fetch_dependencies_for(&socket, &name.unwrap_or("".to_owned()))
            }
            ("--rebuilddependers", name) => {
                request_rebuild_dependers(&socket, &name.unwrap_or("".to_owned()))
            }
            ("--releasebuildsol", filename) => {
                submit_solution(&socket, &filename.unwrap_or("".to_owned()), false)
            }
            ("--crossbuildsol", filename) => {
                submit_solution(&socket, &filename.unwrap_or("".to_owned()), true)
            }
            _ => debug!(
                "No arg found; This is likely a bug or this argument has not been implemented yet."
            ),
        }
    }

    match socket.shutdown(std::net::Shutdown::Both) {
        Ok(_) => {}
        Err(err) => trace!("Failed to close socket: {}", err),
    }
    Ok(())
}
