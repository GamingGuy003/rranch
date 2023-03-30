use args::argparser::Arg;
use log::{debug, error, info, trace};

use crate::{args::argparser::ArgParser, connection::client::Client};
mod args;
mod connection;
mod json;
mod structs;
mod util;

fn main() -> std::io::Result<()> {
    pretty_env_logger::init_custom_env("rranch_log");

    let mut ap = ArgParser::new(Vec::new(), None, Vec::new());

    ap.define_arg(Arg::new("h", "help", "Prints this help dialog", None));
    ap.define_arg(Arg::new(
        "dbs",
        "debugshell",
        "Opens a remote debugshell",
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
