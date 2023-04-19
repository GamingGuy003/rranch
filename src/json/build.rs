use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Build {
    pub pkgname: String,
    pub buildtype: String,
}

impl Build {
    pub fn new(pkgname: &str, release: bool) -> Self {
        if release {
            Self {
                pkgname: pkgname.to_owned(),
                buildtype: String::from("RELEASE"),
            }
        } else {
            Self {
                pkgname: pkgname.to_owned(),
                buildtype: String::from("CROSS"),
            }
        }
    }
}
