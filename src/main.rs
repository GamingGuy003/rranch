use cmds::{
    fetch::{
        fetch_client_status, fetch_dependencies_for, fetch_dependers_on,
        fetch_difference_pkgb_pkgs, fetch_log_of, fetch_managed_packagebuilds,
        fetch_managed_packages, fetch_package, fetch_packagebuild_for, fetch_sys_log,
    },
    job::{
        request_build, request_cancel_all_jobs, request_cancel_queued_job,
        request_clear_completed_jobs, request_rebuild_dependers, request_status,
    },
    other::{create_template, edit, latest_log, watch_jobs},
    submit::{submit_packagebuild, submit_solution},
};

use log::{debug, error};
use std::process::exit;
use structs::config::Config;
use util::funcs::cleanup;

use crate::{args::argparser::ArgParser, cmds::dbs::run_dbs, sockops::connect::connect};
mod args;
mod cmds;
mod sockops;
mod structs;
mod util;

fn main() -> std::io::Result<()> {
    let mut confpath = format!(
        "{}/.config/rranch.toml",
        dirs::home_dir()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    );

    //try to fetch arguments from cli and parse them
    let mut argparser = ArgParser::new();
    match argparser.args() {
        Ok(_) => debug!("Got args: {:?}", argparser.funcs()),
        Err(err) => {
            error!("Could not get arguments: {}", err);
            argparser.help();
            exit(-1);
        }
    };
    //check if config has been passed
    for func in argparser.funcs() {
        if func.0 == *"--config" {
            confpath = func.1.unwrap_or_default();
            break;
        }
    }

    let conf = Config::new_from_cfg(&confpath);
    //set loglevel

    std::env::set_var("rranch_log", conf.get_client().get_loglevel());
    //init env logger
    pretty_env_logger::init_custom_env("rranch_log");

    //get arg array and connect
    let funcs = argparser.funcs();
    let socketres = connect(
        &conf.get_master().get_addr(),
        &conf.get_master().get_port().to_string(),
        &conf.get_client().get_name(),
        &conf.get_master().get_authkey(),
        &conf.get_client().get_type(),
    );

    let socket = match socketres {
        Ok(socket) => socket,
        Err(err) => {
            error!("Exiting on code {} while creating socket...", err);
            exit(err)
        }
    };

    if funcs.is_empty() {
        error!("No arguments have been provided!");
        argparser.help();
        cleanup(Some(socket), None);
        exit(-1)
    }

    //work out which function to execute
    let mut retc = -1;
    for func in funcs {
        let fmatch = (func.0.as_str(), func.1);
        retc = match fmatch {
            ("--debugshell", _) => run_dbs(&socket),
            ("--checkout", name) => fetch_packagebuild_for(&socket, &name.unwrap_or_default()),
            ("--download", name) => {
                fetch_package(&conf.get_master().get_addr(), &name.unwrap_or_default())
            }
            ("--edit", name) => edit(
                &socket,
                &name.unwrap_or_default(),
                &conf.get_client().get_editor(),
            ),
            ("--template", _) => create_template(),
            ("--submit", filename) => submit_packagebuild(&socket, &filename.unwrap_or_default()),
            ("--releasebuild", name) => request_build(&socket, &name.unwrap_or_default(), false),
            ("--crossbuild", name) => request_build(&socket, &name.unwrap_or_default(), true),
            ("--viewlog", job_id) => fetch_log_of(&socket, &job_id.unwrap_or_default()),
            ("--viewlastlog", _) => latest_log(&socket),
            ("--status", _) => request_status(&socket, false),
            ("--watchjobs", interval) => watch_jobs(&socket, &interval.unwrap_or_default()),
            ("--clientstatus", _) => fetch_client_status(&socket),
            ("--clearjobs", _) => request_clear_completed_jobs(&socket),
            ("--cancelalljobs", _) => request_cancel_all_jobs(&socket),
            ("--canceljob", job_id) => {
                request_cancel_queued_job(&socket, &job_id.unwrap_or_default())
            }
            ("--managedpkgs", _) => fetch_managed_packages(&socket),
            ("--managedpkgbuilds", _) => fetch_managed_packagebuilds(&socket),
            ("--differencepkgs", _) => fetch_difference_pkgb_pkgs(&socket),
            ("--viewsyslog", _) => fetch_sys_log(&socket),
            ("--viewdependers", name) => fetch_dependers_on(&socket, &name.unwrap_or_default()),
            ("--viewdependencies", name) => {
                fetch_dependencies_for(&socket, &name.unwrap_or_default())
            }
            ("--rebuilddependers", name) => {
                request_rebuild_dependers(&socket, &name.unwrap_or_default())
            }
            ("--releasebuildsol", filename) => {
                submit_solution(&socket, &filename.unwrap_or_default(), false)
            }
            ("--crossbuildsol", filename) => {
                submit_solution(&socket, &filename.unwrap_or_default(), true)
            }
            _ => {
                debug!(
                "No arg found; This is likely a bug or this argument has not been implemented yet."
            );
                0
            }
        };
    }

    cleanup(Some(socket), Some(retc));
    Ok(())
}
