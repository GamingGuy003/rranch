use std::process::exit;

use log::debug;

#[derive(Clone)]
pub struct ArgParser {
    //vec of received arg as long + optional value for arg
    recargs: Vec<(String, Option<String>)>,
    //description
    desc: String,
    //vec of valid args
    validarg: Vec<Arg>,
}

#[derive(Clone)]
struct Arg {
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
            param: param,
        }
    }

    pub fn to_string(&self) -> String {
        if self.param.is_some() {
            format!(
                "{:6} {:20} = {:10}\t{}",
                self.short,
                self.long,
                format!("<{}>", self.param.clone().unwrap_or("".to_string())),
                self.desc
            )
        } else {
            format!("{:6} {:33}\t{}", self.short, self.long, self.desc)
        }
    }
}

impl ArgParser {
    pub fn new() -> Self {
        Self {
            recargs: Vec::new(),
            desc: String::from("The AcaciaLinux branch client rewritten in rust."),
            validarg: vec![
                Arg::new("h", "help", "Prints this help dialogue.", None),
                Arg::new(
                    "ds",
                    "debugshell",
                    "Runs a debugshell on the remote server.",
                    None,
                ),
                Arg::new(
                    "c",
                    "checkout",
                    "Checks out the specified packagebuild from the server.",
                    Some("name".to_owned()),
                ),
                Arg::new("t", "template", "Creates a template packagebuild.", None),
                Arg::new(
                    "s",
                    "submit",
                    "Submits the specified packagebuild file to the server.",
                    Some("filename".to_owned()),
                ),
                Arg::new(
                    "rb",
                    "releasebuild",
                    "Requests a releasebuild for the specified package.",
                    Some("name".to_owned()),
                ),
                Arg::new(
                    "cb",
                    "crossbuild",
                    "Requests a crossbuild for the specified package.",
                    Some("name".to_owned()),
                ),
                Arg::new(
                    "vl",
                    "viewlog",
                    "Requests build log of the specified completed job.",
                    Some("job_id".to_owned()),
                ),
                Arg::new(
                    "vll",
                    "viewlastlog",
                    "Requests the log of the last job that completed.",
                    None
                ),
                Arg::new(
                    "st",
                    "status",
                    "Requests a list of running / completed / queued jobs.",
                    None,
                ),
                Arg::new(
                    "w",
                    "watchjobs",
                    "Watches the joblist in the given interval in seconds.",
                    Some("interval".to_owned()),
                ),
                Arg::new(
                    "cs",
                    "clientstatus",
                    "Requests a list of clients connected to the server.",
                    None,
                ),
                Arg::new(
                    "cj",
                    "clearjobs",
                    "Clears the completed jobs from the server.",
                    None,
                ),
                Arg::new(
                    "caj",
                    "cancelalljobs",
                    "Cancels all currently queued jobs.",
                    None,
                ),
                Arg::new(
                    "cn",
                    "canceljob",
                    "Cancels specified currently queued job.",
                    Some("job_id".to_owned()),
                ),
                Arg::new(
                    "mp",
                    "managedpkgs",
                    "Requests list of managed packages.",
                    None,
                ),
                Arg::new(
                    "mk",
                    "managedpkgbuilds",
                    "Requests list of managed packagebuilds.",
                    None,
                ),
                Arg::new(
                    "dp",
                    "differencepkgs",
                    "Requests difference between packagebuilds and packages.",
                    None,
                ),
                Arg::new(
                    "sys",
                    "viewsyslog",
                    "Requests buildbot system logs from server.",
                    None,
                ),
                Arg::new(
                    "vd",
                    "viewdependers",
                    "Requests all dependers for specified package.",
                    Some("name".to_owned()),
                ),
                Arg::new(
                    "vdp",
                    "viewdependencies",
                    "Shows if dependencies of a package have a pkgbuild / pkg",
                    Some("name".to_owned()),
                ),
                Arg::new(
                    "rd",
                    "rebuilddependers",
                    "Rebuild dependers of specified package.",
                    Some("name".to_owned()),
                ),
                Arg::new(
                    "rbs",
                    "releasebuildsol",
                    "Submits a branch solution to server. (RELEASEBUILD)",
                    Some("sol_file".to_owned()),
                ),
                Arg::new(
                    "cbs",
                    "crossbuildsol",
                    "Submits a branch solution to server. (CROSSBUILD)",
                    Some("sol_file".to_owned()),
                ),
            ],
        }
    }

    //read recieved arguments and parse them into internal array
    pub fn args(&mut self) -> Result<(), std::io::Error> {
        debug!("Fetching args...");
        let args: Vec<String> = std::env::args().collect();
        let mut skip: bool = false;
        for (mut idx, arg) in args.iter().enumerate().skip(1) {
            if !skip {
                idx += 1;
                let option: (String, Option<String>);
                let complete = match self.is_valid(&arg) {
                    Ok(complete) => complete,
                    Err(err) => {
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
                    }
                };
                //complete
                if complete {
                    if match self.has_value(&arg) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
                        }
                    } {
                        //complete and has value (arg=value)
                        let parts = arg.split("=").collect::<Vec<&str>>();
                        let long = match self.get_arg_long(&parts[0]) {
                            Ok(long) => long,
                            Err(err) => {
                                // gehirn leistung
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidInput,
                                    err,
                                ));
                            }
                        };
                        option = (long, Some(parts[1].to_owned()));
                        self.recargs.push(option);
                    } else {
                        //complete and has no value (arg)
                        let long = match self.get_arg_long(&arg) {
                            Ok(long) => long,
                            Err(err) => {
                                // gehirn leistung
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidInput,
                                    err,
                                ));
                            }
                        };
                        option = (long.to_owned(), None);
                        if option.0 == "--help".to_owned() {
                            self.help();
                            exit(0)
                        }
                        self.recargs.push(option);
                    }
                    //incomplete
                } else {
                    if args.get(idx) != None {
                        let next_elem = match args.get(idx) {
                            Some(next) => next,
                            None => {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidInput,
                                    format!("No value supplied supplied for {}", *arg),
                                ))
                            }
                        };
                        let long = match self.get_arg_long(&arg) {
                            Ok(long) => long,
                            Err(err) => {
                                // gehirn leistung
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidInput,
                                    err,
                                ));
                            }
                        };
                        option = (long, Some(next_elem.to_string()));
                        self.recargs.push(option);
                        idx += 1;
                        let _ = idx;
                        skip = true;
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("No value supplied supplied for {}", *arg),
                        ));
                    }
                }
            } else {
                skip = false;
            }
        }
        Ok(())
    }

    //fetches vec of argument name and optional value for argument
    pub fn funcs(&self) -> Vec<(String, Option<String>)> {
        self.recargs.clone()
    }

    //prints help for arguments
    pub fn help(&self) {
        println!(
            "{}\n\nLoglevel rranch_log=[info, debug, trace]\n\nOptions:",
            self.desc
        );
        for h in self.validarg.clone() {
            println!("\t{}", h.to_string());
        }
    }

    //checks if argument is valid and syntax correct. returns true if argument complete and false if argument needs value, err(string) if something else is wrong
    pub fn is_valid(&self, arg: &str) -> Result<bool, String> {
        let split = arg.split("=").collect::<Vec<&str>>();
        let arghead = split[0];
        for element in self.validarg.clone() {
            //args exists
            if arghead == element.short || arghead == element.long {
                //if requires no value
                if element.param.is_none() {
                    return Ok(true);
                } else if split.len() == 2 {
                    if split[1].len() == 0 {
                        return Err("Invalid value".to_owned());
                    } else {
                        return Ok(true);
                    }
                } else {
                    return Ok(false);
                }
            }
        }
        Err(String::from("Invalid argument"))
    }

    //checks if argument needs value
    pub fn has_value(&self, arg: &str) -> Result<bool, String> {
        let arghead = arg.split("=").collect::<Vec<&str>>()[0];
        for element in self.validarg.clone() {
            //args exists
            if arghead == element.short || arghead == element.long {
                //if requires value
                if element.param.is_some() {
                    return Ok(true);
                } else {
                    return Ok(false);
                };
            }
        }
        Err(String::from("Invalid argument"))
    }

    //returns long version of argument string
    pub fn get_arg_long(&self, arg: &str) -> Result<String, String> {
        for element in self.validarg.clone() {
            //args exists
            if arg == element.short || arg == element.long {
                return Ok(element.long.clone());
            }
        }
        Err(String::from("Invalid argument"))
    }
}
