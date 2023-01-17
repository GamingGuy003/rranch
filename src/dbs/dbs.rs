use std::{io::Write, net::TcpStream, process::exit};

use log::{debug, error};

use crate::coms::coms::write_and_read;

//a little debug shell
pub fn run_dbs(sock: &TcpStream) {
    debug!("Starting debug shell...");
    println!("Type quit to quit client or enter to quit dbs");
    loop {
        print!("[branch] ~> ");
        let mut user_input = String::new();
        _ = std::io::stdout().flush();
        _ = std::io::stdin().read_line(&mut user_input);
        let send = user_input.trim();

        //enter to quit dbs quit to quit program
        match send {
            "" => break,
            "quit" => exit(0),
            _ => {}
        }

        match write_and_read(&sock, send.to_owned()) {
            Ok(msg) => println!("Response: {}", msg),
            Err(err) => {
                error!("{}", err);
                exit(-1)
            }
        };
    }
}
