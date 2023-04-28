use std::process::exit;

use args::argparser::{Arg, ArgParser};
use log::{debug, error, trace};
use structs::{client::Client, config::Config};

use crate::util::funcs::configure;

mod args;
mod funcs;
mod json;
mod structs;
mod util;

fn main() -> std::io::Result<()> {
    let confpath = format!(
        "{}/.config/rranch.toml",
        dirs::home_dir().unwrap_or_default().to_str().unwrap_or_default()
    );
    let config = Config::new_from_cfg(&confpath, 1)?;
    std::env::set_var("rranch_log", config.get_client().get_loglevel());
    pretty_env_logger::init_custom_env("rranch_log");
    let mut argparser = ArgParser::new(
        Vec::new(),
        Some("The branch client rewritten in Rust with Protocol version 2 (json)"),
        Vec::new(),
    );

    let args = Vec::from([
        Arg::new("c", "checkout", "Fetches pkgbuild", Some("name")),
        Arg::new("s", "submit", "Submits pkgbuild", Some("path")),
        Arg::new("n", "new", "Creates new pkgb", Some("name")),
        Arg::new("rb", "releasebuild", "Releasebuilds pkg", Some("name")),
        Arg::new("cb", "crossbuild", "Crossbuilds pkg", Some("name")),
        Arg::new("jl", "job-log", "Joblog for job", Some("job_id")),
        Arg::new("sl", "sys-log", "Fetches syslog", None),
        Arg::new("depds", "dependers", "Dependers", Some("name")),
        Arg::new("deps", "dependencies", "Dependencies", Some("name")),
        Arg::new("rd", "rebuilddependers", "Rebuild dependers", Some("name")),
        Arg::new("js", "job-status", "Shows jobs", None),
        Arg::new("wj", "watch-jobs", "Periodic jobstatus", Some("interval")),
        Arg::new("llc", "latest-log-complete", "Latest job log", None),
        Arg::new("llr", "latest-log-running", "Latest job log", None),
        Arg::new("cs", "client-status", "Shows active clients", None),
        Arg::new("ci", "client-info", "Shows client info", Some("name")),
        Arg::new("mpkg", "managed-pkgs", "Shows pkg status", None),
        Arg::new("mpkgb", "managed-pkgbs", "Shows pkgb status", None),
        Arg::new("d", "diff", "pkgs / pkgbs diff", None),
        Arg::new("cc", "clear-completed", "Clear completed jobs", None),
        Arg::new("cq", "cancel-queued", "Cancels queued job", Some("id")),
        Arg::new("caq", "cancel-all-queued", "Cancels all queud jobs", None),
        Arg::new("ssr", "submit-solution-release", "Submits release solution", Some("path")),
        Arg::new("ssc", "submit-solution-cross", "Submits cross solution", Some("path")),
        Arg::new("e", "edit", "Opens pkgb with editor", Some("name")),
        Arg::new("el", "edit-local", "Edits local pkgb", Some("path")),
        Arg::new("rm", "remove-pkg", "Removes pkg", Some("name")),
        Arg::new("es", "extrasources", "Shows extrasources", None),
        Arg::new("res", "remove-extrasource", "Removes extrasource", Some("id")),
        Arg::new("ses", "submit-extrasource", "Submits extrasource", Some("path")),
        Arg::new("ex", "export", "Exports all pkgbs", None),
        Arg::new("im", "import", "Imports all pkgbs", Some("path")),
        Arg::new("cf", "configure", "configures client", None),
        Arg::new("fp", "fetch-pkg", "Downloads pkg", Some("name")),
    ]);

    argparser.define_args(args);
    argparser.parse_args();

    let mut client = match Client::new(&config.get_master().get_addr(), config.get_master().get_port() as u16) {
        Ok(client) => client,
        Err(err) => {
            error!("Failed to connect to master: {err}");
            exit(-1)
        }
    };

    match client.auth(
        &config.get_client().get_name(),
        &config.get_client().get_type(),
        &config.get_master().get_authkey(),
        config.get_client().get_protver(),
    ) {
        Ok(response) => debug!("{}", response.logon_message),
        Err(err) => {
            error!("Failed to authenticate: {err}");
            exit(-1)
        }
    };

    for parsed in argparser.get_parsed() {
        debug!("Trying to handle {}", parsed.0);
        let result = match parsed.0.clone().as_str() {
            "--checkout" => client.checkout(parsed.1.unwrap_or_default().as_str()),
            "--submit" => client.submit(parsed.1.unwrap_or_default().as_str()),
            "--new" => client.new_pkgbuild(parsed.1.unwrap_or_default().as_str()),
            "--releasebuild" => client.build(parsed.1.unwrap_or_default().as_str(), true),
            "--crossbuild" => client.build(parsed.1.unwrap_or_default().as_str(), false),
            "--job-log" => client.watch_job_log(parsed.1.unwrap_or_default().as_str(), 1),
            "--sys-log" => client.show_sys_log(),
            "--dependers" => client.show_dependers(parsed.1.unwrap_or_default().as_str()),
            "--dependencies" => client.show_dependencies(parsed.1.unwrap_or_default().as_str()),
            "--rebuilddependers" => client.rebuild_dependers(parsed.1.unwrap_or_default().as_str()),
            "--job-status" => client.show_jobs_status(false),
            "--watch-jobs" => client.watch_jobs(parsed.1.unwrap_or_default().as_str()),
            "--latest-log-complete" => client.show_latest_complete_log(),
            "--latest-log-running" => client.show_latest_running_log(),
            "--client-status" => client.show_clients(),
            "--client-info" => client.show_client_info(parsed.1.unwrap_or_default().as_str()),
            "--managed-pkgs" => client.show_managed_pkgs(),
            "--managed-pkgbs" => client.show_managed_pkgbs(),
            "--diff" => client.show_diff(),
            "--clear-completed" => client.clear_completed(),
            "--cancel-queued" => client.cancel_queued(Some(parsed.1.unwrap_or_default().as_str())),
            "--cancel-all-queued" => client.cancel_queued(None),
            "--submit-solution-release" => client.submit_solution(parsed.1.unwrap_or_default().as_str(), true),
            "--submit-solution-cross" => client.submit_solution(parsed.1.unwrap_or_default().as_str(), false),
            "--edit" => client.edit(parsed.1.unwrap_or_default().as_str(), &config.get_client().get_editor()),
            "--edit-local" => client.edit_local(parsed.1.unwrap_or_default().as_str(), &config.get_client().get_editor()),
            "--remove-pkg" => client.remove_pkg(parsed.1.unwrap_or_default().as_str()),
            "--extra-sources" => client.show_extra_sources(),
            "--remove-extrasource" => client.remove_extra_source(parsed.1.unwrap_or_default().as_str()),
            "--submit-extrasource" => client.submit_extra_source(parsed.1.unwrap_or_default().as_str()),
            "--export" => client.export(),
            "--import" => client.import(parsed.1.unwrap_or_default().as_str()),
            "--configure" => configure(&confpath, &config.get_client().get_editor()),
            "--help" => Ok(()),
            "--fetch-pkg" => client.get_pkg(&config.get_master().get_fetch_url(), parsed.1.unwrap_or_default().as_str()),
            arg => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unimplemented argument {}", arg),
            )),
        };
        match result {
            Ok(_) => trace!("Handled {}", parsed.0),
            Err(err) => {
                error!("Failed on {}, reason: {}", parsed.0, err);
                client.shutdown()?;
                exit(-1);
            }
        }
    }

    Ok(())
}
