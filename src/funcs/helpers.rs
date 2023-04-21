use std::{process::Command, time::Duration};

use console::Term;
use indicatif::ProgressBar;
use log::info;

use crate::{
    json::{
        dependers::Dependers,
        jobs_status::{Job, JobsStatus},
        pkgbuild::PackageBuild,
        request::Request,
        response::{Response, StatusCode},
    },
    structs::{client::Client, diff::Diff},
    util::funcs::{get_pkgbs, get_yn},
};

impl Client {
    pub fn show_latest_log(&mut self) -> Result<(), std::io::Error> {
        let req = Request::new("GETJOBSTATUS", None);

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => self.show_job_log(
                serde_json::from_value::<JobsStatus>(resp.payload)?
                    .completedjobs
                    .last()
                    .unwrap_or(&Job::default())
                    .job_id
                    .as_str(),
            ),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn watch_jobs(&mut self, interval: &str) -> Result<(), std::io::Error> {
        let n = interval.parse::<u64>().unwrap_or(1);
        loop {
            self.show_jobs_status(true)?;
            std::thread::sleep(Duration::from_secs(n));
        }
    }

    pub fn get_dependecies(
        &mut self,
        pkgname: &str,
    ) -> Result<(Vec<String>, Vec<String>), std::io::Error> {
        let pkgb = &self.get_pkgb(pkgname)?;
        Ok((
            pkgb.build_dependencies.clone(),
            pkgb.cross_dependencies.clone(),
        ))
    }

    pub fn get_dependers(
        &mut self,
        pkgname: &str,
    ) -> Result<(Vec<String>, Vec<String>), std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(
            &Request::new("GETDEPENDERS", Some(serde_json::to_value(pkgname)?)),
        )?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok((
                serde_json::from_value::<Dependers>(resp.payload.clone())?.releasebuild,
                serde_json::from_value::<Dependers>(resp.payload)?.crossbuild,
            )),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn get_diff(&mut self) -> Result<Vec<Diff>, std::io::Error> {
        let pkgbs = self.get_managed_pkgbs()?;
        let pkgs = self.get_managed_pkgs()?;
        let mut combined = pkgbs.clone();
        combined.sort();
        pkgs.iter().for_each(|pkg| {
            if !combined.contains(pkg) {
                combined.push(pkg.clone())
            }
        });
        Ok(combined
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
            .collect::<Vec<Diff>>())
    }

    pub fn get_managed_pkgs(&mut self) -> Result<Vec<String>, std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(
            &Request::new("GETMANAGEDPKGS", None),
        )?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(serde_json::from_value::<Vec<String>>(resp.payload)?),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn get_managed_pkgbs(&mut self) -> Result<Vec<String>, std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(
            &Request::new("GETMANAGEDPKGBUILDS", None),
        )?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(serde_json::from_value::<Vec<String>>(resp.payload)?),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn get_pkgb(&mut self, pkgname: &str) -> Result<PackageBuild, std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(
            &Request::new("CHECKOUT", Some(serde_json::to_value(pkgname)?)),
        )?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(serde_json::from_value::<PackageBuild>(resp.payload)?),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn edit(&mut self, pkgname: &str, editor: &str) -> Result<(), std::io::Error> {
        self.checkout(pkgname)?;
        let path = format!("{}/package.bpb", pkgname);
        let child = Command::new(editor).arg(path.clone()).spawn();

        match child {
            Ok(mut child) => {
                if !child.wait()?.success() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Editor closed with error",
                    ));
                }
            }
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Editor {editor} not found"),
                ));
            }
        }

        if get_yn("Do you want to submit the changes?", false)? {
            self.submit(path.as_str())?;
        } else {
            info!("Aborted submit due to user choice.");
        }

        if get_yn("Do you want to delete the local packagebuild", true)? {
            std::fs::remove_dir_all(pkgname)?;
        }
        Ok(())
    }

    pub fn export(&mut self) -> Result<(), std::io::Error> {
        let mut pkgbs = self.get_managed_pkgbs()?;
        pkgbs.sort();
        if !get_yn(
            &format!("Do you want to fetch {} pkgbuilds?", pkgbs.len()),
            false,
        )? {
            info!("Aborted due to user choice");
            return Ok(());
        }

        let progress = ProgressBar::new(pkgbs.len() as u64);
        let term = Term::stdout();
        for pkgb in pkgbs {
            term.clear_screen()?;
            progress.inc(1);
            self.checkout(&pkgb)?;
        }

        progress.finish();
        term.clear_screen()?;
        Ok(())
    }

    pub fn import(&mut self, path: &str) -> Result<(), std::io::Error> {
        let pkgbs = get_pkgbs(path)?;

        if !get_yn(
            &format!("Do you want to submit {} pkgbuilds?", pkgbs.len()),
            false,
        )? {
            info!("Aborted due to user choice");
            return Ok(());
        }
        let progress = ProgressBar::new(pkgbs.len() as u64);
        let term = Term::stdout();
        for pkgb in pkgbs {
            term.clear_screen()?;
            progress.inc(1);
            self.submit(&pkgb)?;
        }

        term.clear_screen()?;
        progress.finish();
        Ok(())
    }
}
