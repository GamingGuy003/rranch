use crate::{
    json::{
        auth::{AuthRequest, AuthResponse},
        pkgbuild::PackageBuild,
        request::Request,
        response::{Response, StatusCode},
    },
    structs::client::Client,
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

    pub fn checkout(&mut self, pkg_name: &str) -> Result<(), std::io::Error> {
        let req = Request::new("CHECKOUT", Some(serde_json::to_value(pkg_name)?));

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

    pub fn get_job_log(&mut self, job_id: &str) -> Result<(), std::io::Error> {
        let req = Request::new("GETJOBLOG", Some(serde_json::to_value(job_id)?));

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        println!("{resp:#?}");
        Ok(())
    }
}
