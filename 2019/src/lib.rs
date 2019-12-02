#![feature(try_trait)]
#[macro_use]
extern crate quick_error;

// Workaround for NoneError not converting to std Error
#[cfg(not(feature = "video"))]
quick_error! {
	#[derive(Debug)]
	pub enum Error {
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

#[macro_use]
mod video;

mod day1;

aoc_runner_derive::aoc_lib! { year = 2019 }
