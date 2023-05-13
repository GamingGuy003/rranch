use std::{io::Write, process::Command};

use console::{Style, Term};

pub fn get_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

pub fn get_yn(text: &str, default: bool) -> Result<bool, std::io::Error> {
    let red = Style::new().red();
    let green = Style::new().green();

    let question = if default {
        format!("{} [{}{}/{}] ", text, green.apply_to("Y").bold().underlined(), green.apply_to("es").underlined(), red.apply_to("No"))
    } else {
        format!(
            "{} [{}/{}{}] ",
            text,
            green.apply_to("Yes").bold(),
            red.apply_to("N").bold().underlined(),
            red.apply_to("o").underlined()
        )
    };

    loop {
        print!("{question}");
        let input = get_input()?;

        if input.is_empty() {
            return Ok(default);
        }

        if input.to_lowercase() == "no" || input.to_lowercase() == "n" {
            return Ok(false);
        } else if input.to_lowercase() == "yes" || input.to_lowercase() == "y" {
            return Ok(true);
        }

        println!("Invalid input, please try again");
    }
}

pub fn print_cols(vec: Vec<String>, max: Option<usize>, offset: usize, spacing: usize) {
    let max_len = (if max.is_none() {
        vec.iter().max_by_key(|val| val.len()).cloned().unwrap_or_default().len()
    } else {
        max.unwrap_or_default()
    }) + spacing;

    let colcount = Term::stdout().size().1 as usize / max_len;
    vec.iter().enumerate().for_each(|(idx, val)| {
        if idx % colcount == 0 && idx != 0 {
            println!();
        }
        print!("{:<1$}", val, max_len + offset);
    });
    println!()
}

pub fn get_pkgbs(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut pkgbs = Vec::new();
    let paths = std::fs::read_dir(path)?;

    for path in paths.flatten() {
        if let Some(file_name) = path.file_name().to_str() {
            if path.file_type()?.is_file() && file_name.ends_with(".bpb") {
                pkgbs.push(path.path().display().to_string());
            } else if path.file_type()?.is_dir() {
                pkgbs.append(&mut get_pkgbs(&path.path().display().to_string())?);
            }
        }
    }
    Ok(pkgbs)
}

pub fn configure(path: &str, editor: &str) -> Result<(), std::io::Error> {
    let child = Command::new(editor).arg(path).spawn();

    match child {
        Ok(mut child) => {
            if !child.wait()?.success() {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Editor closed with error"));
            }
        }
        Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Problem with editor: {err}"))),
    }
    Ok(())
}

pub fn truncate_to(input: String, max_chars: usize) -> String {
    let mut s = input;
    if s.len() > max_chars {
        s.truncate(max_chars);
        s.replace_range(max_chars - 3..=s.len() - 1, "...");
        s.to_string()
    } else {
        s.to_string()
    }
}
