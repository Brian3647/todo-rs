use structopt::StructOpt;

use std::fs::create_dir_all;
use std::fs::write;
use std::fs::File;
use std::path::PathBuf;

use crate::helpers::get_home_dir;

#[derive(StructOpt, Debug)]
pub enum Command {
	/// Create a new task by prompting the information.
	#[structopt(name = "new")]
	New,

	/// Delete a task specified by id.
	#[structopt(name = "delete")]
	Delete { id: usize },

	/// Get full info of a task specified by id.
	#[structopt(name = "info")]
	Info { id: usize },

	/// Show the full list of tasks.
	#[structopt(name = "all")]
	All,

	/// Mark a task as done.
	#[structopt(name = "done")]
	Done { id: usize },

	/// Mark a task as NOT done.
	#[structopt(name = "undone")]
	UnDone { id: usize }
}

impl Command {
	pub fn file(&self) -> PathBuf {
		let mut default = get_home_dir();

		default.push(".todo");
		default.push("data.json");

		if !default.as_path().exists() {
			create_dir_all(&default.parent().unwrap()).unwrap();
			File::create(&default).unwrap();
			write(&default, "[]").unwrap();
		}

		default
	}
}
