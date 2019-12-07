#![feature(generators)]
#![feature(try_trait)]
#![feature(stmt_expr_attributes)]

#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate pest_derive;

// Workaround for NoneError not converting to std Error
#[cfg(not(feature = "video"))]
quick_error! {
	#[derive(Debug)]
	pub enum Error {
		ParseInt(err: std::num::ParseIntError) {
			from()
		}
		Str(err: String) {
			from()
		}
		IO(err: std::io::Error) {
			from()
		}
		None(err: std::option::NoneError) {
			from()
		}
	}
}
#[cfg(feature = "video")]
quick_error! {
	#[derive(Debug)]
	pub enum Error {
		ParseInt(err: std::num::ParseIntError) {
			from()
		}
		Str(err: String) {
			from()
		}
		IO(err: std::io::Error) {
			from()
		}
		None(err: std::option::NoneError) {
			from()
		}
		Image(err: image::ImageError) {
			from()
		}
	}
}

impl<X: Copy + Ord + std::hash::Hash + std::fmt::Debug> From<pest::error::Error<X>> for Error {
	fn from(err: pest::error::Error<X>) -> Error {
		Error::Str(format!("{}", &err))
	}
}

impl PartialEq for Error {
	fn eq(&self, _: &Error) -> bool {
		true
	}
}

#[macro_use]
mod video;

mod intcode;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

aoc_runner_derive::aoc_lib! { year = 2019 }
