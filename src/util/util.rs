use std::{io::Write, net::TcpStream, process::exit};

use console::{Style, Term};
use log::trace;

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

pub fn get_choice(text: &str) -> bool {
    let red = Style::new().red();
    let red_b = Style::new().red().bold();
    let green = Style::new().green();
    let green_b = Style::new().green().bold();

    let mut _failed = false;
    loop {
        let mut input = String::new();
        if _failed {
            println!("Invalid input, please try again");
            _failed = false;
        }
        print!(
            "{}? [{}{}/{}{}] ",
            text,
            green_b.apply_to("Y"),
            green.apply_to("es"),
            red_b.apply_to("N"),
            red.apply_to("o"),
        );
        std::io::stdout().flush().unwrap_or(());
        std::io::stdin().read_line(&mut input).unwrap_or(0);
        let input = input.trim();
        if input.len() == 0 || input.to_lowercase() == "no" || input.to_lowercase() == "n" {
            return false;
        } else if input.to_lowercase() == "yes" || input.to_lowercase() == "y" {
            return true;
        } else {
            _failed = true;
        }
    }
}

pub fn cleanup(socket: Option<TcpStream>, code: Option<i32>) {
    match socket {
        Some(sock) => {
            sock.shutdown(std::net::Shutdown::Both)
                .unwrap_or(trace!("Failed to close socket"));
        }
        None => trace!("No socket to close"),
    }
    if code.is_some() {
        exit(code.unwrap_or(-1))
    }
}
