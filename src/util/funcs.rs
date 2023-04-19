use std::io::Write;

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
        format!(
            "{} [{}{}/{}] ",
            text,
            green.apply_to("Y").bold().underlined(),
            green.apply_to("es").underlined(),
            red.apply_to("No")
        )
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