use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Solution {
    pub solution: Vec<Vec<String>>,
    pub buildtype: String,
}

impl Solution {
    pub fn new(solution: Vec<Vec<String>>, release: bool) -> Self {
        Self {
            solution,
            buildtype: if release {
                "RELEASE".to_owned()
            } else {
                "CROSS".to_owned()
            },
        }
    }
}
