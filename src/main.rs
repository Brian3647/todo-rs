#![warn(clippy::all)]

mod args;
mod helpers;
mod log;
mod todo;

use structopt::StructOpt;

use std::error::Error;
use std::fs::read_to_string;
use std::fs::write;
use std::process::exit;

use args::Command;
use helpers::prompt;
use todo::Task;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
	let cmd = Command::from_args();
	let file = cmd.file();
	let contents = read_to_string(&file)?;
	let tasks = match serde_json::from_str::<Vec<Task>>(&contents) {
		Ok(x) => x,
		Err(e) => {
			error!("Error parsing ~/.todo/data.json");
			error!("Did you edit manually the file?");
			error!(
				"Please try removing the file or manually fixing the eror: '{}'",
				e
			);

			exit(1);
		}
	};

	let mut res: Vec<Task>;

	match cmd {
		Command::All => {
			tasks.into_iter().for_each(|p| println!("{}", p));
			exit(0);
		}

		Command::New => {
			res = tasks.clone();

			res.push(Task::from_prompt(tasks.len() + 1));
		}

		Command::Done { id } => {
			res = tasks.clone();
			let task = tasks.into_iter().position(|p| p.id == id);

			match task {
				Some(x) => res[x].done = true,
				None => {
					error!("Couldn't find any task with id {}", id);
					exit(1);
				}
			};
		}

		Command::UnDone { id } => {
			res = tasks.clone();
			let task = tasks.into_iter().position(|p| p.id == id);

			match task {
				Some(x) => res[x].done = false,
				None => {
					error!("Couldn't find any task with id {}", id);
					exit(1);
				}
			};
		}

		Command::Delete { id } => {
			if id == 0 {
				warn!("This will remove all taks");
				let confirm = prompt("Are you sure you want to do it? [Y/n] ");

				if confirm.is_empty() || confirm.to_lowercase() == "y" {
					res = vec![];
				} else {
					exit(0);
				}
			} else {
				let mut iter = tasks.into_iter();

				if iter.find(|p| p.id == id).is_none() {
					error!("Couldn't find a post with id {}", id);
					exit(0);
				} else {
					res = iter.filter(|p| p.id != id).collect();
				}
			}
		}

		Command::Info { id } => {
			let task = tasks.into_iter().find(|p| p.id == id);

			let task = match task {
				Some(x) => x,
				None => {
					error!("Couldn't find any task with id {}", id);
					exit(1);
				}
			};

			task.show_info();

			exit(0);
		}
	}

	write(&cmd.file(), serde_json::to_string(&res).unwrap()).unwrap();

	Ok(())
}
