use json::{
    auth::{AuthRequest, AuthResponse},
    response::StatusCode,
};
use structs::client::Client;

use crate::json::{request::Request, response::Response};

mod args;
mod json;
mod structs;

fn main() -> std::io::Result<()> {
    pretty_env_logger::init_custom_env("rranch_log");
    let mut client = Client::new("localhost", 27015)?;
    let auth_req = AuthRequest::new("test", "CONTROLLER", "default", 0);
    let req = Request::new("AUTH", Some(serde_json::to_value(&auth_req)?));
    let resp =
        serde_json::from_str::<Response>(&client.write_read(&serde_json::to_string(&req)?)?)?;
    match resp.statuscode {
        StatusCode::Ok => println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::from_value::<AuthResponse>(resp.payload)?)?
        ),
        _ => println!("{resp:#?}"),
    }

    Ok(())
}
