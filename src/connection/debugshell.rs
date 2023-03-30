use std::io::Write;

use log::{debug, error};

use super::client::Client;

impl Client {
    // starts debug shell
    pub fn debug_shell(&mut self) -> Result<(), std::io::Error> {
        debug!("Starting debug shell...");
        println!("Type quit or enter to quit dbs");
        let mut send;
        loop {
            let mut user_input = String::new();
            print!("[rranch] ~> ");
            std::io::stdout().flush()?;
            std::io::stdin().read_line(&mut user_input)?;
            send = user_input.trim();

            //enter to quit dbs quit to quit program
            match send {
                "" | "quit" => return self.exit_clean(0),
                _ => {}
            }

            match self.write_and_read(send) {
                Ok(msg) => println!("{}", msg),
                Err(err) => {
                    error!("{}", err);
                    return self.exit_clean(-1);
                }
            };
        }
    }
}
