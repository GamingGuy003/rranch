use cmds::cmds::{
    cancel_all_jobs, cancel_queued_job, checkout_pkg, clear_completed_jobs, client_status,
    diff_pkgs, managed_pkg_builds, managed_pkgs, rebuild_dependers, status, submit_build,
    submit_pkg, submit_solution, view_log, view_sys_log, view_tree, create_template,
};
use conn::conn::connect;
use console::Style;
use dbs::dbs::run_dbs;
use log::{debug, error};
use std::process::exit;
use structs::config::{Client, Config, Master};

use crate::args::args::ArgParser;
mod args;
mod cmds;
mod coms;
mod conn;
mod dbs;
mod structs;

fn main() -> std::io::Result<()> {
    //init env logger
    let conf = Config::new_from_cfg("rranch.toml");
    //set loglevel
    let mut loglevel = "";
    match conf
        .client
        .as_ref()
        .unwrap_or(&Client {
            name: None,
            r#type: None,
            loglevel: None,
        })
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
    let yellow = Style::new().yellow();
    let socket = connect(
        conf.master
            .as_ref()
            .unwrap_or(&Master {
                addr: None,
                port: None,
                authkey: None,
            })
            .addr
            .clone()
            .unwrap_or("localhost".to_owned())
            .as_str(),
        conf.master
            .as_ref()
            .unwrap_or(&Master {
                addr: None,
                port: None,
                authkey: None,
            })
            .port
            .clone()
            .unwrap_or(27015)
            .to_string()
            .as_str(),
        format!(
            "{}",
            conf.client
                .as_ref()
                .unwrap_or(&Client {
                    name: None,
                    r#type: None,
                    loglevel: None
                })
                .name
                .clone()
                .unwrap_or("a-rranch-client".to_owned())
                .clone()
        )
        .as_str(),
        conf.master
            .as_ref()
            .unwrap_or(&Master {
                addr: None,
                port: None,
                authkey: None,
            })
            .authkey
            .clone()
            .unwrap_or("default".to_owned())
            .as_str(),
        conf.client
            .as_ref()
            .unwrap_or(&Client {
                name: None,
                r#type: None,
                loglevel: None,
            })
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
    //work out which function to execute
    for func in funcs {
        let fmatch = (func.0.as_str(), func.1);
        match fmatch {
            ("--debugshell", _) => run_dbs(&socket),
            ("--checkout", name) => checkout_pkg(&socket, &name.unwrap_or("".to_owned())),
            ("--template", _) => create_template(),
            ("--submit", filename) => submit_pkg(&socket, &filename.unwrap_or("".to_owned())),
            ("--releasebuild", name) => {
                submit_build(&socket, &name.unwrap_or("".to_owned()), false)
            }
            ("--crossbuild", name) => submit_build(&socket, &name.unwrap_or("".to_owned()), true),
            ("--viewlog", job_id) => view_log(&socket, &job_id.unwrap_or("".to_owned())),
            ("--status", _) => status(&socket),
            ("--clientstatus", _) => client_status(&socket),
            ("--clearjobs", _) => clear_completed_jobs(&socket),
            ("--cancelalljobs", _) => cancel_all_jobs(&socket),
            ("--canceljob", job_id) => cancel_queued_job(&socket, &job_id.unwrap_or("".to_owned())),
            ("--managedpkgs", _) => managed_pkgs(&socket),
            ("--managedpkgbuilds", _) => managed_pkg_builds(&socket),
            ("--differencepkgs", _) => diff_pkgs(&socket),
            ("--viewsyslog", _) => view_sys_log(&socket),
            ("--viewtree", name) => view_tree(&socket, &name.unwrap_or("".to_owned())),
            ("--rebuilddependers", name) => {
                rebuild_dependers(&socket, &name.unwrap_or("".to_owned()))
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
    Ok(())
}
