#[macro_export]
macro_rules! warn {
	($($arg:tt)*) => {{
		use colored::Colorize;

		println!("{} {}", "error:".yellow(), format_args!($($arg)*));
	}};
}

#[macro_export]
macro_rules! error {
	($($arg:tt)*) => {{
		use colored::Colorize;

		println!("{} {}", "error:".red(), format_args!($($arg)*));
	}};
}
