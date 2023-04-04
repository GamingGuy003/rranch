use args::argparser::Arg;
use log::{debug, error, info, trace};
use structs::pkgbuild;

use crate::{args::argparser::ArgParser, connection::client::Client};
mod args;
mod connection;
mod json;
mod structs;
mod util;

fn main() -> std::io::Result<()> {
    pretty_env_logger::init_custom_env("rranch_log");

    let mut ap = ArgParser::new(Vec::new(), None, Vec::new());

    ap.define_arg(Arg::new("h", "help", "Outputs this help menu", None));
    ap.define_arg(Arg::new(
        "dbs",
        "debugshell",
        "Opens a debug shell for the remote server",
        None,
    ));

    ap.define_arg(Arg::new(
        "bs",
        "build-status",
        "Displays the status of jobs on the server",
        None,
    ));
    ap.define_arg(Arg::new(
        "cs",
        "client-status",
        "Displays the status of currently connected cleints",
        None,
    ));
    ap.define_arg(Arg::new(
        "ci",
        "client-info",
        "Displays info to the specified client",
        Some("client".to_owned()),
    ));
    ap.define_arg(Arg::new(
        "sl",
        "syslog",
        "Displays the servers syslog",
        None,
    ));
    ap.define_arg(Arg::new(
        "bl",
        "build-log",
        "Displays the build log for the specified job",
        Some("job_id".to_owned()),
    ));

    ap.define_arg(Arg::new(
        "rb",
        "release-build",
        "Requests a releasebuild for the specified package",
        Some("pkg_name".to_owned()),
    ));
    ap.define_arg(Arg::new(
        "cb",
        "cross-build",
        "Requests a crossbuild for the specified package",
        Some("pkg_name".to_owned()),
    ));
    ap.define_arg(Arg::new(
        "cj",
        "cancel-job",
        "Cancels the specified job",
        Some("job_id".to_owned()),
    ));
    ap.define_arg(Arg::new(
        "caj",
        "cancel-all-jobs",
        "Cancels all queued jobs",
        None,
    ));
    ap.define_arg(Arg::new(
        "cc",
        "clear-completed",
        "Clears all completed jobs",
        None,
    ));
    ap.define_arg(Arg::new(
        "rd",
        "rebuild-dependers",
        "Queues rebuild for all dependers of the specified package",
        Some("pkg_name".to_owned()),
    ));
    ap.define_arg(Arg::new(
        "ed",
        "edit",
        "Edits the requested pkgbuild with the configured editor",
        Some("pkg_name".to_owned()),
    ));
    ap.define_arg(Arg::new(
        "ea",
        "export-all",
        "Exports all pkgbuilds from server into cwd",
        None,
    ));
    ap.define_arg(Arg::new(
        "ia",
        "import-all",
        "Imports all pkgbuilds recursively from specified location",
        Some("path".to_owned()),
    ));
    ap.define_arg(Arg::new(
        "ies",
        "import-extra-source",
        "Imports the specified extrasource",
        Some("path".to_owned()),
    ));
    ap.define_arg(Arg::new(
        "res",
        "remove-extra-source",
        "Removes the specified extra source",
        Some("es_id".to_owned()),
    ));
    ap.define_arg(Arg::new(
        "raes",
        "remove-all-extra-sources",
        "Removes all extra sources from the server",
        None,
    ));

    ap.define_arg(Arg::new(
        "c",
        "checkout",
        "Checks out the specified package",
        Some("pkg_name".to_owned()),
    ));
    ap.define_arg(Arg::new(
        "s",
        "submit",
        "Submits the specified pkgbuild",
        Some("path".to_owned()),
    ));
    ap.define_arg(Arg::new(
        "ssr",
        "submit-solution-release",
        "Submits a solution file and requests a releasebuild for all contained packages",
        Some("path".to_owned()),
    ));
    ap.define_arg(Arg::new(
        "ssc",
        "submit-solution-cross",
        "Submits a solution file and requests a crossbuild for all contained packages",
        Some("path".to_owned()),
    ));
    ap.define_arg(Arg::new("lp", "list-pkgs", "Lists all managed pkgs", None));
    ap.define_arg(Arg::new(
        "lpb",
        "list-pkgbs",
        "Lists all managed pkgbuilds",
        None,
    ));
    ap.define_arg(Arg::new(
        "d",
        "diff",
        "Shows diff between pkgs / pkgbs",
        None,
    ));
    ap.define_arg(Arg::new(
        "ld",
        "list-dependers",
        "Shows all dependers of the specified pkg and their status",
        Some("pkg_name".to_owned()),
    ));
    ap.define_arg(Arg::new(
        "ldd",
        "list-dependencies",
        "Shows all dependencies of the specified pkg and their status",
        Some("pkg_name".to_owned()),
    ));
    ap.define_arg(Arg::new(
        "ses",
        "show-extra-sources",
        "Shows all managed extra sources",
        None,
    ));
    ap.define_arg(Arg::new(
        "dpb",
        "delete-pkgbuild",
        "Deletes the specified pkgbuild",
        Some("pkg_name".to_owned()),
    ));

    ap.define_arg(Arg::new(
        "t",
        "template",
        "Creates a template pkgbuild",
        None,
    ));

    ap.parse_args();

    let mut client = match Client::new(
        "",
        27015,
        Some("".to_string()),
        "rranch-client".to_owned(),
        "CONTROLLER".to_owned(),
    ) {
        Ok(client) => client,
        Err(err) => {
            error!("Failed to connect to server: {err}");
            return Ok(());
        }
    };

    match client.auth() {
        Ok(()) => info!("Successfully authenticated"),
        Err(err) => error!("Failed to authenticate: {err}"),
    }

    match client.set_type() {
        Ok(()) => info!("Successfully set machine type"),
        Err(err) => error!("Failed to set machine type: {err}"),
    }

    match client.set_name() {
        Ok(()) => info!("Successfully set machine name"),
        Err(err) => error!("Failed to set machine name: {err}"),
    };
    for arg in ap.get_parsed() {
        let result = match arg.0.as_str() {
            "--debugshell" => client.debug_shell(),
            "--build-status" => client.build_status(),
            "--client-status" => client.client_status(),
            "--client-info" => client.client_info(&arg.1.unwrap_or_default()),
            "--syslog" => client.sys_log(),
            "--build-log" => client.build_log(&arg.1.unwrap_or_default()),
            "--release-build" => client.build(true, &arg.1.unwrap_or_default()),
            "--cross-build" => client.build(false, &arg.1.unwrap_or_default()),
            "--cancel-job" => client.cancel_job(&arg.1.unwrap_or_default()),
            "--cancel-all-jobs" => client.cancel_all_jobs(),
            "--clear-completed" => client.clear_completed(),
            "--rebuild-dependers" => client.rebuild_dependers(&arg.1.unwrap_or_default()),
            "--edit" => client.edit(&arg.1.unwrap_or_default(), "vim"),
            "--export-all" => client.export_all(),
            "--import-all" => client.import_folder(&arg.1.unwrap_or_default()),
            "--import-extra-source" => client.submit_extra_source(&arg.1.unwrap_or_default()),
            "--remove-extra-source" => client.remove_extra_source(&arg.1.unwrap_or_default()),
            "--remove-all-extra-sources" => client.remove_all_extra_sources(),
            "--checkout" => client.checkout(&arg.1.unwrap_or_default()),
            "--submit" => client.submit(&arg.1.unwrap_or_default()),
            "--submit-solution-release" => client.submit_sol(true, &arg.1.unwrap_or_default()),
            "--submit-solution-cross" => client.submit_sol(false, &arg.1.unwrap_or_default()),
            "--list-pkgs" => client.get_packages(),
            "--list-pkgbs" => client.get_packagebuilds(),
            "--diff" => client.get_diff(),
            "--list-dependers" => client.get_dependers(&arg.1.unwrap_or_default()),
            "--list-dependencies" => client.get_dependencies(&arg.1.unwrap_or_default()),
            "--show-extra-sources" => client.get_extra_sources(),
            "--delete-pkgbuikd" => client.delete_pkgbuild(&arg.1.unwrap_or_default()),
            "--template" => client.template(),
            other => {
                trace!("{other}");
                Ok(())
            }
        };
        match result {
            Ok(()) => {}
            Err(err) => {
                error!("{}", err);
                client.exit_clean(-1)?;
            }
        }
    }

    match client.close_connection() {
        Ok(()) => debug!("Successfully shut down connection"),
        Err(err) => error!("Failed to shut down connection: {err}"),
    }

    Ok(())
}
