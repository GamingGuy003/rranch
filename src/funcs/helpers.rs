use std::{collections::HashMap, io::Write, process::Command, time::Duration};

use console::{Style, Term};
use curl::easy::{Easy, WriteError};
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info, trace};

use crate::{
    json::{
        dependers::Dependers,
        job_request::JobRequest,
        jobs_status::{Job, JobsStatus},
        pkgbuild::PackageBuild,
        request::Request,
        response::{Response, StatusCode},
    },
    structs::{client::Client, diff::Diff},
    util::funcs::{get_input, get_pkgbs, get_yn, print_cols},
};

impl Client {
    pub fn show_latest_complete_log(&mut self) -> Result<(), std::io::Error> {
        let req = Request::new("GETJOBSTATUS", None);

        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => self.watch_job_log(serde_json::from_value::<JobsStatus>(resp.payload)?.completedjobs.last().unwrap_or(&Job::default()).job_id.as_str(), 1),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn show_latest_running_log(&mut self) -> Result<(), std::io::Error> {
        let req = Request::new("GETJOBSTATUS", None);

        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => self.watch_job_log(serde_json::from_value::<JobsStatus>(resp.payload)?.runningjobs.last().unwrap_or(&Job::default()).job_id.as_str(), 1),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn watch_jobs(&mut self, interval: &str) -> Result<(), std::io::Error> {
        let n = interval.parse::<u64>().unwrap_or(1);
        loop {
            self.show_jobs_status(true)?;
            std::thread::sleep(Duration::from_secs(n));
        }
    }

    pub fn get_dependecies(&mut self, pkgname: &str) -> Result<(Vec<String>, Vec<String>), std::io::Error> {
        let pkgb = &self.get_pkgb(pkgname)?;
        Ok((pkgb.build_dependencies.clone(), pkgb.cross_dependencies.clone()))
    }

    pub fn get_dependers(&mut self, pkgname: &str) -> Result<(Vec<String>, Vec<String>), std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("GETDEPENDERS", Some(serde_json::to_value(pkgname)?)))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok((
                serde_json::from_value::<Dependers>(resp.payload.clone())?.releasebuild,
                serde_json::from_value::<Dependers>(resp.payload)?.crossbuild,
            )),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn get_diff(&mut self) -> Result<Vec<Diff>, std::io::Error> {
        let pkgbs = self.get_managed_pkgbs()?;
        let pkgs = self.get_managed_pkgs()?;
        let combined = self.get_all()?;

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
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("GETMANAGEDPKGS", None))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(serde_json::from_value::<Vec<String>>(resp.payload)?),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn get_managed_pkgbs(&mut self) -> Result<Vec<String>, std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("GETMANAGEDPKGBUILDS", None))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(serde_json::from_value::<Vec<String>>(resp.payload)?),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn get_pkgb(&mut self, pkgname: &str) -> Result<PackageBuild, std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("CHECKOUT", Some(serde_json::to_value(pkgname)?)))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(serde_json::from_value::<PackageBuild>(resp.payload)?),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn edit(&mut self, pkgname: &str, editor: &str) -> Result<(), std::io::Error> {
        self.checkout(pkgname)?;
        let path = format!("{}/package.bpb", pkgname);
        self.edit_local(&path, editor)?;
        if get_yn("Do you want to delete the local packagebuild", true)? {
            std::fs::remove_dir_all(pkgname)?;
        }
        Ok(())
    }

    pub fn edit_local(&mut self, path: &str, editor: &str) -> Result<(), std::io::Error> {
        let child = Command::new(editor).arg(path).spawn();

        match child {
            Ok(mut child) => {
                if !child.wait()?.success() {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Editor closed with error"));
                }
            }
            Err(_) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Editor {editor} not found")));
            }
        }

        if get_yn("Do you want to submit the changes?", false)? {
            self.submit(path)?;
        } else {
            info!("Aborted submit due to user choice.");
        }

        Ok(())
    }

    pub fn export(&mut self) -> Result<(), std::io::Error> {
        let mut pkgbs = self.get_managed_pkgbs()?;
        pkgbs.sort();
        if !get_yn(&format!("Do you want to fetch {} pkgbuilds?", pkgbs.len()), false)? {
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

        if !get_yn(&format!("Do you want to submit {} pkgbuilds?", pkgbs.len()), false)? {
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

    pub fn get_pkg(&mut self, url: &str, pkgname: &str) -> Result<(), std::io::Error> {
        trace!("Trying to fetch from: {url}");
        let url = format!("{}?get=package&pkgname={}", url, pkgname);
        let mut easy = Easy::new();

        easy.url(&url)?;
        easy.progress(true)?;

        let pb = ProgressBar::new(1);

        pb.set_style(
            match ProgressStyle::with_template("{percent:>3}% [{bar:.green/white}] {bytes:>7}/{total_bytes:>7} ({bytes_per_sec})") {
                Ok(pstyle) => pstyle,
                Err(err) => {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed setting progress style: {err}")));
                }
            }
            .progress_chars("#=--"),
        );

        easy.progress_function(move |dl_total, dl_now, _, _| {
            if dl_total != 0.0 {
                pb.set_length(dl_total as u64);
            }
            pb.set_position(dl_now as u64);
            true
        })?;

        let mut file = std::fs::File::create(format!("{}.tar.xz", pkgname))?;
        easy.write_function(move |data| match file.write_all(data).map(|_| data.len()) {
            Ok(size) => Ok(size),
            Err(err) => {
                error!("Failed to write content: {}", err);
                Err(WriteError::Pause)
            }
        })?;

        let transfer = easy.transfer();
        Ok(transfer.perform()?)
    }

    pub fn get_job_log(&mut self, job_id: &str, offset: usize) -> Result<Vec<String>, std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("GETJOBLOG", Some(serde_json::to_value(JobRequest::new(job_id, offset))?)))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(serde_json::from_value::<Vec<String>>(resp.payload)?),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn watch_job_log(&mut self, job_id: &str, interval: u64) -> Result<(), std::io::Error> {
        let mut offset = 0;
        loop {
            let log = self.get_job_log(job_id, offset)?;
            offset += log.len();
            log.iter().for_each(|line| println!("{line}"));
            if self.get_jobs()?.completedjobs.iter().any(|elem| elem.job_id == *job_id) {
                break;
            }
            std::thread::sleep(Duration::from_secs(interval));
        }
        Ok(println!("{}", Style::new().bold().apply_to("Job done!")))
    }

    pub fn get_jobs(&mut self) -> Result<JobsStatus, std::io::Error> {
        let resp = serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&Request::new("GETJOBSTATUS", None))?)?)?;

        match resp.statuscode {
            StatusCode::Ok => Ok(serde_json::from_value::<JobsStatus>(resp.payload)?),
            StatusCode::InternalServerError | StatusCode::RequestFailure => Err(std::io::Error::new(std::io::ErrorKind::Other, serde_json::to_string(&resp.payload)?)),
        }
    }

    pub fn new_pkgbuild(&mut self, pkgname: &str, editor: &str, templates: HashMap<String, Vec<String>>) -> Result<(), std::io::Error> {
        let mut pkgb = PackageBuild::new();
        pkgb.name = pkgname.to_owned();
        loop {
            println!("Which template do you want to use?");
            for (key, value) in templates.clone() {
                println!("{key}: {:#?}", value)
            }
            print!("Choice: ");
            let choice = get_input()?;
            match templates.get(&choice) {
                Some(value) => {
                    pkgb.build_script = value.to_vec();
                    break;
                }
                None => println!("Invalid input, try again"),
            }
        }
        pkgb.create_workdir()?;
        self.edit_local(format!("{pkgname}/package.bpb").as_str(), editor)
    }

    pub fn get_pkg_with_name(&mut self, pkgname: &str) -> Result<(), std::io::Error> {
        let bold = Style::new().bold();
        let style = Style::new().italic().bold().green();

        let combined = self.get_all()?;
        let mut found = combined
            .iter()
            .filter(|elem| elem.to_lowercase().contains(&pkgname.to_lowercase()))
            .map(|elem| elem.to_owned())
            .collect::<Vec<String>>();
        let max = Some(found.iter().max_by_key(|val| val.len()).cloned().unwrap_or_default().len());
        found = found.iter().map(|val| val.replace(pkgname, format!("{}", style.apply_to(pkgname)).as_str())).collect::<Vec<String>>();
        println!("{}", bold.apply_to(format!("Found packages matching '{}':", pkgname)));
        print_cols(found, max, 16, 3);
        Ok(())
    }

    pub fn get_info(&mut self, pkgname: &str) -> Result<(), std::io::Error> {
        let bold = Style::new().bold();
        let italic = Style::new().italic();
        let desc = self.get_pkgb(pkgname)?;
        println!("{}", bold.apply_to(format!("Package {}", pkgname)));
        println!("{:<23} {}", italic.apply_to("Name:"), desc.name);
        println!("{:<23} {} ({})", italic.apply_to("Version:"), desc.version, desc.real_version);
        println!("{:<23} {}", italic.apply_to("Description:"), desc.description);
        println!("{:<23} {}", italic.apply_to("ExtraSources:"), desc.extra_sources.join(", "));
        println!("{:23} {}", italic.apply_to("RunDeps:"), desc.dependencies.join(", "));
        println!("{:23} {}", italic.apply_to("BuildDeps:"), desc.build_dependencies.join(", "));
        println!("{:23} {}", italic.apply_to("CrossDeps:"), desc.cross_dependencies.join(", "));

        Ok(())
    }

    pub fn get_all(&mut self) -> Result<Vec<String>, std::io::Error> {
        let pkgbs = self.get_managed_pkgbs()?;
        let pkgs = self.get_managed_pkgs()?;
        let mut combined = pkgbs;
        combined.sort();
        pkgs.iter().for_each(|pkg| {
            if !combined.contains(pkg) {
                combined.push(pkg.clone())
            }
        });
        Ok(combined)
    }
}
