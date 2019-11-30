#![feature(try_trait)]
#[macro_use]
extern crate quick_error;

// Workaround for NoneError not converting to std Error
quick_error! {
	#[derive(Debug)]
	pub enum Error {
		Io(err: std::io::Error) {
			from()
		}
		No(err: std::option::NoneError) {
			from()
		}
	}
}

#[macro_use]
mod video;

mod day1;

aoc_runner_derive::aoc_lib! { year = 2019 }
