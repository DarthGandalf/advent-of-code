#![feature(try_trait)]
#[macro_use]
extern crate quick_error;

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

impl PartialEq for Error {
	fn eq(&self, _: &Error) -> bool {
		true
	}
}

#[macro_use]
mod video;

mod day1;
mod day2;
mod day3;

aoc_runner_derive::aoc_lib! { year = 2019 }
