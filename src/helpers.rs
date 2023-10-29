use colored::Colorize;

use crate::error;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

pub fn get_home_dir() -> PathBuf {
	home::home_dir().unwrap_or_else(|| {
		error!("Couldn't find home directory.");
		std::process::exit(1);
	})
}

pub fn prompt(msg: &str) -> String {
	let mut s = String::new();

	print!("{}", msg.bold().bright_black());

	let _ = stdout().flush();

	stdin()
		.read_line(&mut s)
		.expect("Did not enter a correct string");

	if let Some('\n') = s.chars().next_back() {
		s.pop();
	}

	if let Some('\r') = s.chars().next_back() {
		s.pop();
	}

	s
}
