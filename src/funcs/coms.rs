use console::Style;

use crate::{
    json::{
        auth::{AuthRequest, AuthResponse},
        build::Build,
        clients::Clients,
        dependers::Dependers,
        jobs_status::{Job, JobsStatus},
        pkgbuild::PackageBuild,
        request::Request,
        response::{Response, StatusCode},
    },
    structs::client::Client,
    util::funcs::print_vec_cols,
};

impl Client {
    pub fn auth(
        &mut self,
        machine_idenifier: &str,
        machine_type: &str,
        machine_authkey: &str,
        version: i32,
    ) -> Result<AuthResponse, std::io::Error> {
        let req = Request::new(
            "AUTH",
            Some(serde_json::to_value(AuthRequest::new(
                machine_idenifier,
                machine_type,
                machine_authkey,
                version,
            ))?),
        );

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(serde_json::from_value::<AuthResponse>(resp.payload)?),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn checkout(&mut self, pkgname: &str) -> Result<(), std::io::Error> {
        let req = Request::new("CHECKOUT", Some(serde_json::to_value(pkgname)?));

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => {
                serde_json::from_value::<PackageBuild>(resp.payload)?.create_workdir()
            }
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn submit(&mut self, path: &str) -> Result<(), std::io::Error> {
        let pkgbuild = PackageBuild::from_str(&std::fs::read_to_string(path)?)?;

        let req = Request::new("SUBMIT", Some(serde_json::to_value(pkgbuild)?));

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}", serde_json::to_string(&resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn get_job_log(&mut self, job_id: &str) -> Result<(), std::io::Error> {
        let req = Request::new("GETJOBLOG", Some(serde_json::to_value(job_id)?));

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => {
                serde_json::from_value::<Vec<String>>(resp.payload)?
                    .iter()
                    .for_each(|line| println!("{line}"));
                Ok(())
            }
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn build(&mut self, pkgname: &str, release: bool) -> Result<(), std::io::Error> {
        let req = Request::new(
            "BUILD",
            Some(serde_json::to_value(Build::new(pkgname, release))?),
        );

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}", serde_json::to_string(&resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn get_sys_log(&mut self) -> Result<(), std::io::Error> {
        let req = Request::new("GETSYSLOG", None);

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => {
                serde_json::from_value::<Vec<String>>(resp.payload)?
                    .iter()
                    .for_each(|line| println!("{line}"));
                Ok(())
            }
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn get_dependers(&mut self, pkgname: &str) -> Result<(), std::io::Error> {
        let bold = Style::new().bold();

        let req = Request::new("GETDEPENDERS", Some(serde_json::to_value(pkgname)?));

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => {
                let dependers = serde_json::from_value::<Dependers>(resp.payload)?;
                println!("{}", bold.apply_to("Releasebuild"));
                print_vec_cols(dependers.releasebuild, None, 0);
                println!("{}", bold.apply_to("Crossbuild"));
                print_vec_cols(dependers.crossbuild, None, 0);
                Ok(())
            }
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn rebuild_dependers(&mut self, pkgname: &str) -> Result<(), std::io::Error> {
        let req = Request::new("REBUILDDEPENDERS", Some(serde_json::to_value(pkgname)?));

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}", serde_json::to_string(&resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn get_jobs_status(&mut self, clear_screen: bool) -> Result<(), std::io::Error> {
        let req = Request::new("GETJOBSTATUS", None);

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        if clear_screen {
            console::Term::clear_screen(&console::Term::stdout())?;
        }
        match resp.statuscode {
            StatusCode::Ok => Ok(println!(
                "{}",
                serde_json::from_value::<JobsStatus>(resp.payload)?
            )),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn get_clients(&mut self) -> Result<(), std::io::Error> {
        let req = Request::new("GETCONNECTEDCLIENTS", None);
        let bold = Style::new().bold();

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok({
                let clients = serde_json::from_value::<Clients>(resp.payload)?;
                println!("{}", bold.apply_to("Controllers"));
                print_vec_cols(clients.controllers, None, 0);
                println!("{}", bold.apply_to("Buildbots"));
                print_vec_cols(clients.buildbots, None, 0);
            }),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn get_managed_pkgs(&mut self) -> Result<(), std::io::Error> {
        let req = Request::new("GETMANAGEDPKGS", None);
        let bold = Style::new().bold();

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok({
                println!("{}", bold.apply_to("Managed Packages"));
                let mut pkgs = serde_json::from_value::<Vec<String>>(resp.payload)?;
                pkgs.sort();
                print_vec_cols(pkgs, None, 0)
            }),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn get_managed_pkgbs(&mut self) -> Result<(), std::io::Error> {
        let req = Request::new("GETMANAGEDPKGBUILDS", None);
        let bold = Style::new().bold();

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok({
                println!("{}", bold.apply_to("Managed Packagebuilds"));
                let mut pkgbs = serde_json::from_value::<Vec<String>>(resp.payload)?;
                pkgbs.sort();
                print_vec_cols(pkgbs, None, 0)
            }),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn clear_completed(&mut self) -> Result<(), std::io::Error> {
        let req = Request::new("CLEARCOMPLETEDJOBS", None);

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}", serde_json::to_string(&resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn cancel_queued(&mut self, job_id: Option<&str>) -> Result<(), std::io::Error> {
        let req = if job_id.is_some() {
            Request::new(
                "CANCELQUEUEDJOB",
                Some(serde_json::to_value(job_id.unwrap_or_default())?),
            )
        } else {
            Request::new("CANCELQUEUEDJOBS", None)
        };

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(println!("{}", serde_json::to_string(&resp.payload)?)),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }
}
