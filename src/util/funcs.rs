use std::{io::Write, net::TcpStream, process::exit};

use console::{Style, Term};
use log::trace;

use crate::json::job::Job;

pub fn print_vec_cols(vec: Vec<String>, mut max: Option<i32>, offset: i32) {
    if max.is_none() {
        max = Some(
            (vec.iter()
                .max_by_key(|s| s.chars().count())
                .unwrap_or(&"".to_owned())
                .chars()
                .count()
                + 5) as i32,
        );
    }

    let elem_width = max.unwrap_or(30) + offset;
    let colcount = (Term::stdout().size().1 / elem_width as u16) as usize;
    for (idx, val) in vec.into_iter().enumerate() {
        if idx % colcount == 0 && idx != 0 {
            println!();
        }
        print!("{:<1$}", val, elem_width as usize);
    }
    println!();
}

pub fn get_choice(text: &str, default: bool) -> Result<bool, std::io::Error> {
    let red = Style::new().red();
    let green = Style::new().green();
    let red_b = Style::new().red().bold();
    let green_b = Style::new().green().bold();
    let q = if default {
        format!(
            "{}? [{}{}/{}{}] ",
            text,
            green_b.apply_to("Y").underlined(),
            green.apply_to("es").underlined(),
            red_b.apply_to("N"),
            red.apply_to("o"),
        )
    } else {
        format!(
            "{}? [{}{}/{}{}] ",
            text,
            green_b.apply_to("Y"),
            green.apply_to("es"),
            red_b.apply_to("N").underlined(),
            red.apply_to("o").underlined(),
        )
    };

    loop {
        let input;

        print!("{}", q);
        input = get_input()?;

        if input.is_empty() || input.to_lowercase() == "no" || input.to_lowercase() == "n" {
            if input.is_empty() {
                return Ok(default);
            } else {
                return Ok(false);
            }
        } else if input.to_lowercase() == "yes" || input.to_lowercase() == "y" {
            return Ok(true);
        }

        println!("Invalid input, please try again");
    }
}

pub fn get_input() -> Result<String, std::io::Error> {
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_owned())
}

pub fn get_pkgbs(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut pkgbs = Vec::new();
    let paths = std::fs::read_dir(path)?;

    for path in paths {
        if let Ok(entry) = path {
            if let Some(file_name) = entry.file_name().to_str() {
                if entry.file_type()?.is_file() && file_name.ends_with(".bpb") {
                    pkgbs.push(entry.path().display().to_string());
                } else if entry.file_type()?.is_dir() {
                    pkgbs.append(&mut get_pkgbs(&entry.path().display().to_string())?);
                }
            }
        }
    }
    Ok(pkgbs)
}

pub fn print_job_table(jobs: Vec<Job>) {
    let italic = Style::new().italic();
    println!(
        "{}",
        italic.apply_to(format!(
            "{:<20} {:<15} {:<40} {:10}",
            "Name", "Status", "Id", "Client"
        ))
    );
    jobs.iter().for_each(|job| println!("{job}"))
}
