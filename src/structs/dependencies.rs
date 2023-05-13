pub struct Dependencies {
    pub runtime: Vec<String>,
    pub release: Vec<String>,
    pub cross: Vec<String>,
}

impl Dependencies {
    pub fn new(runtime: Vec<String>, release: Vec<String>, cross: Vec<String>) -> Self {
        Self { runtime, release, cross }
    }
}
