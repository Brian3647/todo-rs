use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

use crate::helpers::prompt;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
	pub title: String,
	pub info: String,
	pub id: usize,
	pub done: bool
}

impl Display for Task {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let title = if self.done {
			(&self.title).strikethrough().to_string()
		} else {
			self.title.clone()
		};

		write!(f, "{} {}", (self.id.to_string()).bright_black(), title)
	}
}

impl Task {
	pub fn show_info(&self) {
		let title = if self.done {
			self.title.clone() + " " + &"(done)".on_white().bright_black().to_string()
		} else {
			self.title.clone()
		};

		println!("{}", title.underline().bold());
		println!("- {}", self.info);
	}

	pub fn from_prompt(id: usize) -> Self {
		let title = prompt("> What should the title of the task be? ");
		let info = prompt("> Please add more information of it: ");

		Self {
			title,
			info,
			id,
			done: false
		}
	}
}
