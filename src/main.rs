use args::argparser::Arg;
use log::{error, info};

use crate::{args::argparser::ArgParser, connection::client::Client};
mod args;
mod cmds;
mod connection;
mod sockops;
mod structs;
mod util;

fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let mut ap = ArgParser::new(Vec::new(), None, Vec::new());
    ap.define_arg(Arg::new("t", "typ", "typo", None));
    ap.define_arg(Arg::new("m", "monn", "monnolo", Some("gaming".to_owned())));
    ap.parse_args();
    println!("{:#?}", ap.get_parsed());
    let mut client = match Client::new(
        "localhost",
        27015,
        Some("hirn".to_string()),
        "schitcliönt".to_owned(),
        "CONTROLLER".to_owned(),
    ) {
        Ok(client) => client,
        Err(err) => {
            error!("Failed to connect to server: {err}");
            return Ok(());
        }
    };

    match client.auth() {
        Ok(_) => info!("Successfully authenticated"),
        Err(err) => error!("Failed to authenticate: {err}"),
    }

    match client.close_connection() {
        Ok(()) => info!("Successfully shut down connection"),
        Err(err) => error!("Failed to shut down connection: {err}"),
    }
    Ok(())
}
