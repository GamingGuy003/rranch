use std::path::Path;

use console::Style;
use log::{debug, info};

use crate::{
    json::{
        auth::{AuthRequest, AuthResponse},
        build::Build,
        clients::Clients,
        extra_source::{ExtraSourceReceive, ExtraSourceSubmit},
        pkgbuild::PackageBuild,
        request::Request,
        response::{Response, StatusCode},
        solution::Solution,
    },
    structs::{client::Client, deps::Deps, diff::Diff},
    util::funcs::{get_input, print_cols},
};

impl Client {
    pub fn auth(&mut self, machine_idenifier: &str, machine_type: &str, machine_authkey: &str, version: u16) -> Result<AuthResponse, std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new(
            "AUTH",
            Some(serde_json::to_value(AuthRequest::new(machine_idenifier, machine_type, machine_authkey, version))?),
        ))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(serde_json::from_value::<AuthResponse>(resp.payload)?),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn checkout(&mut self, pkgname: &str) -> Result<(), std::io::Error> {
        self.get_pkgb(pkgname)?.create_workdir()
    }

    pub fn submit(&mut self, path: &str) -> Result<(), std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new(
            "SUBMIT",
            Some(serde_json::to_value(PackageBuild::from_str(&std::fs::read_to_string(path)?)?)?),
        ))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}", serde_json::to_string(&resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn build(&mut self, pkgname: &str, release: bool) -> Result<(), std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("BUILD", Some(serde_json::to_value(Build::new(pkgname, release))?)))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}", serde_json::to_string(&resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn show_sys_log(&mut self) -> Result<(), std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("GETSYSLOG", None))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(serde_json::from_value::<Vec<String>>(resp.payload)?.iter().for_each(|line| println!("{line}"))),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn show_dependers(&mut self, pkgname: &str) -> Result<(), std::io::Error> {
        let bold = Style::new().bold();

        let dependers = self.get_dependers(pkgname)?;
        let pkgbs = self.get_managed_pkgbs()?;
        let pkgs = self.get_managed_pkgs()?;

        let release = dependers
            .0
            .iter()
            .map(|name| {
                let mut elem = Diff::new(name.clone());
                if pkgbs.contains(name) {
                    elem.pkgb = true;
                }
                if pkgs.contains(name) {
                    elem.pkg = true;
                }
                elem
            })
            .collect::<Vec<Diff>>();

        let cross = dependers
            .1
            .iter()
            .map(|name| {
                let mut elem = Diff::new(name.clone());
                if pkgbs.contains(name) {
                    elem.pkgb = true;
                }
                if pkgs.contains(name) {
                    elem.pkg = true;
                }
                elem
            })
            .collect::<Vec<Diff>>();

        println!("{}", bold.apply_to("Releasebuild"));
        print_cols(release.iter().map(|diffelem| format!("{diffelem}")).collect::<Vec<String>>(), None, 8, 3);
        println!("{}", bold.apply_to("Crossbuild"));
        print_cols(cross.iter().map(|diffelem| format!("{diffelem}")).collect::<Vec<String>>(), None, 8, 3);
        Ok(())
    }

    pub fn show_dependencies(&mut self, pkgname: &str) -> Result<(), std::io::Error> {
        let bold = Style::new().bold();

        let dependencies = self.get_dependecies(pkgname)?;
        let pkgbs = self.get_managed_pkgbs()?;
        let pkgs = self.get_managed_pkgs()?;

        let release = dependencies
            .0
            .iter()
            .map(|name| {
                let mut elem = Diff::new(name.clone());
                if pkgbs.contains(name) {
                    elem.pkgb = true;
                }
                if pkgs.contains(name) {
                    elem.pkg = true;
                }
                elem
            })
            .collect::<Vec<Diff>>();

        let cross = dependencies
            .1
            .iter()
            .map(|name| {
                let mut elem = Diff::new(name.clone());
                if pkgbs.contains(name) {
                    elem.pkgb = true;
                }
                if pkgs.contains(name) {
                    elem.pkg = true;
                }
                elem
            })
            .collect::<Vec<Diff>>();

        println!("{}", bold.apply_to("Releasebuild"));
        print_cols(release.iter().map(|diffelem| format!("{diffelem}")).collect::<Vec<String>>(), None, 8, 3);
        println!("{}", bold.apply_to("Crossbuild"));
        print_cols(cross.iter().map(|diffelem| format!("{diffelem}")).collect::<Vec<String>>(), None, 8, 3);
        Ok(())
    }

    pub fn rebuild_dependers(&mut self, pkgname: &str) -> Result<(), std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("REBUILDDEPENDERS", Some(serde_json::to_value(pkgname)?)))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}", serde_json::to_string(&resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn show_jobs_status(&mut self, clear_screen: bool) -> Result<(), std::io::Error> {
        if clear_screen {
            console::Term::clear_screen(&console::Term::stdout())?;
        }

        println!("{}", self.get_jobs()?);
        Ok(())
    }

    pub fn show_clients(&mut self) -> Result<(), std::io::Error> {
        let bold = Style::new().bold();

        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("GETCONNECTEDCLIENTS", None))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => {
                {
                    let clients = serde_json::from_value::<Clients>(resp.payload)?;
                    println!("{}", bold.apply_to("Controllers"));
                    print_cols(clients.controllers, None, 0, 3);
                    println!("{}", bold.apply_to("Buildbots"));
                    print_cols(clients.buildbots, None, 0, 3);
                };
                Ok(())
            }
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn show_managed_pkgs(&mut self) -> Result<(), std::io::Error> {
        let bold = Style::new().bold();

        let pkgs = self.get_managed_pkgs()?;

        println!("{}", bold.apply_to("Managed pkgs"));

        print_cols(
            self.get_diff()?
                .iter()
                .filter(|predicate| pkgs.contains(&predicate.name))
                .map(|diff| format!("{diff}"))
                .collect::<Vec<String>>(),
            None,
            8,
            3,
        );
        Ok(())
    }

    pub fn show_managed_pkgbs(&mut self) -> Result<(), std::io::Error> {
        let bold = Style::new().bold();

        let pkgbs = self.get_managed_pkgbs()?;

        println!("{}", bold.apply_to("Managed pkgbs"));

        print_cols(
            self.get_diff()?
                .iter()
                .filter(|predicate| pkgbs.contains(&predicate.name))
                .map(|diff| format!("{diff}"))
                .collect::<Vec<String>>(),
            None,
            8,
            3,
        );
        Ok(())
    }

    pub fn clear_completed(&mut self) -> Result<(), std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("CLEARCOMPLETEDJOBS", None))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}", serde_json::to_string(&resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn cancel_queued(&mut self, job_id: Option<&str>) -> Result<(), std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&if job_id.is_some() {
            Request::new("CANCELQUEUEDJOB", Some(serde_json::to_value(job_id.unwrap_or_default())?))
        } else {
            Request::new("CANCELQUEUEDJOBS", None)
        })?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}", serde_json::to_string(&resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn submit_solution(&mut self, path: &str, release: bool) -> Result<(), std::io::Error> {
        let resp = serde_json::from_str::<Response>(
            &self.write_read(&serde_json::to_string(&Request::new(
                "SUBMITSOLUTION",
                Some(serde_json::to_value(Solution::new(
                    std::fs::read_to_string(path)?
                        .lines()
                        .map(|line| line.split(';').map(|token| token.trim().to_owned()).collect::<Vec<String>>())
                        .collect::<Vec<Vec<String>>>(),
                    release,
                ))?),
            ))?)?,
        )?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}", serde_json::to_string(&resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn show_client_info(&mut self, clientname: &str) -> Result<(), std::io::Error> {
        let bold = Style::new().bold();

        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("GETCLIENTINFO", Some(serde_json::to_value(clientname)?)))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}\n{}", bold.apply_to(clientname), serde_json::from_value::<crate::json::client::Client>(resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn remove_pkg(&mut self, pkgname: &str) -> Result<(), std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("DELETEPKG", Some(serde_json::to_value(pkgname)?)))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}", serde_json::to_string(&resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn show_extra_sources(&mut self) -> Result<(), std::io::Error> {
        let bold = Style::new().bold();
        let italic = Style::new().italic();

        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("GETMANAGEDEXTRASOURCES", None))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => {
                println!("{}", bold.apply_to("Managed Extra Sources"));
                println!("{}", italic.apply_to(format!("{:<40} {:<35} {}", "ID", "File", "Description")));
                Ok(serde_json::from_value::<Vec<ExtraSourceReceive>>(resp.payload)?
                    .iter()
                    .for_each(|extra_source| println!("{extra_source}")))
            }
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn remove_extra_source(&mut self, es_id: &str) -> Result<(), std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("REMOVEEXTRASOURCE", Some(serde_json::to_value(es_id)?)))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}", serde_json::to_string(&resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn submit_extra_source(&mut self, path: &str) -> Result<(), std::io::Error> {
        print!("Description for {path}: ");
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new(
            "TRANSFEREXTRASOURCE",
            Some(serde_json::to_value(ExtraSourceSubmit::new(path, get_input()?.as_str())?)?),
        ))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => debug!("{}", serde_json::to_string(&resp.payload)?),
            StatusCode::InternalServerError | StatusCode::RequestFailure => return Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }

        self.write_raw(std::fs::read(Path::new(path))?)?;

        let resp = serde_json::from_str::<Response>(&self.read()?)?;

        match resp.statuscode {
            StatusCode::Ok => info!("{}", serde_json::to_string(&resp.payload)?),
            StatusCode::InternalServerError | StatusCode::RequestFailure => return Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }

        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("COMPLETETRANSFER", None))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}", serde_json::to_string(&resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn show_diff(&mut self) -> Result<(), std::io::Error> {
        let diff = self.get_diff()?;
        let bold = Style::new().bold();

        println!("{}", bold.apply_to("Diff pkgs / pkgbs"));
        print_cols(diff.iter().map(|diffelem| format!("{diffelem}")).collect::<Vec<String>>(), None, 8, 3);
        Ok(())
    }

    pub fn rebuild_dependencies(&mut self, pkgname: &str, deps: Deps) -> Result<(), std::io::Error> {
        let deps = match deps {
            Deps::Deps => self.get_pkgb(pkgname)?.dependencies,
            Deps::Build => self.get_pkgb(pkgname)?.build_dependencies,
            Deps::Cross => self.get_pkgb(pkgname)?.cross_dependencies,
        };
        for dep in deps {
            self.build(dep.as_str(), true)?;
        }
        Ok(())
    }
}
