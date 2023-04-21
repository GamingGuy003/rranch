use std::fmt::Display;

use console::Style;

#[derive(Clone, Default)]
pub struct Diff {
    pub name: String,
    pub pkg: bool,
    pub pkgb: bool,
}

impl Diff {
    pub fn new(name: String) -> Self {
        Self {
            name,
            pkg: false,
            pkgb: false,
        }
    }
}

impl Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let style = if self.pkg && self.pkgb {
            Style::new().green()
        } else if self.pkgb && !self.pkg {
            Style::new().yellow()
        } else if self.pkg && !self.pkgb {
            Style::new().red()
        } else {
            Style::new().magenta()
        };
        write!(f, "{}", style.apply_to(self.name.clone()))
    }
}
