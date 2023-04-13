use std::fmt::Display;

use log::{error, warn};

#[derive(Clone)]
pub struct ArgParser {
    parsed_args: Vec<(String, Option<String>)>,
    desc: String,
    defined_args: Vec<Arg>,
}

#[derive(Clone)]
pub struct Arg {
    short: String,
    long: String,
    desc: String,
    param: Option<String>,
}

impl Arg {
    pub fn new(short: &str, long: &str, desc: &str, param: Option<String>) -> Self {
        Self {
            short: format!("-{}", short),
            long: format!("--{}", long),
            desc: desc.to_string(),
            param,
        }
    }
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.param.is_some() {
                format!(
                    "{:40}\t{}",
                    format!(
                        "{}\t{} = <{}>",
                        self.short,
                        self.long,
                        self.param.clone().unwrap_or_default()
                    ),
                    self.desc
                )
            } else {
                format!(
                    "{:40}\t{}",
                    format!("{}\t{}", self.short, self.long),
                    self.desc
                )
            }
        )
    }
}

impl ArgParser {
    //returns a new instance
    pub fn new(
        parsed_args: Vec<(String, Option<String>)>,
        desc: Option<&str>,
        defined_args: Vec<Arg>,
    ) -> Self {
        Self {
            parsed_args,
            desc: desc.unwrap_or("No description set").to_owned(),
            defined_args,
        }
    }

    // defines an argument
    pub fn define_arg(&mut self, arg: Arg) {
        self.defined_args.push(arg);
    }

    // returns some argument if long or short name match given or none
    pub fn get_arg_by_name(&self, name: &str) -> Option<Arg> {
        self.defined_args
            .iter()
            .find(|element| element.long == name || element.short == name)
            .cloned()
    }

    //read recieved arguments and parse them into internal array
    pub fn parse_args(&mut self) {
        let mut args = std::env::args().collect::<Vec<String>>();
        args.remove(0);
        //remove stray '=' fields and trim spaces
        args = args
            .iter()
            .flat_map(|x| x.split('='))
            .map(|x| x.trim_matches(|c: char| c.is_whitespace() || c == '='))
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let mut skip = false;
        for (index, arg) in args.iter().enumerate() {
            if skip {
                skip = false;
                continue;
            }

            if let Some(found) = self.get_arg_by_name(arg) {
                if found.param.is_some() && args.get(index + 1).is_some() {
                    self.parsed_args
                        .push((found.long, args.get(index + 1).cloned()));
                    skip = true;
                } else if found.param.is_some() && args.get(index + 1).is_none() {
                    error!("Missing value for argument {index} ({arg})");
                    return;
                } else {
                    if found.long.clone() == "--help" {
                        self.help();
                    }
                    self.parsed_args.push((found.long.clone(), None))
                }
            } else {
                warn!("Unrecognized argument {arg}")
            }
        }
    }

    // returns the parsed argument and value pairs
    pub fn get_parsed(&self) -> Vec<(String, Option<String>)> {
        self.parsed_args.clone()
    }

    // prints help for arguments
    pub fn help(&self) {
        println!(
            "{}\n\nLogLevel rranch_log=[info, debug, trace]\n\nOptions:",
            self.desc
        );
        for h in self.defined_args.clone() {
            println!("\t{}", h);
        }
    }
}
