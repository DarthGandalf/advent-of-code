#![feature(generators)]
#![feature(stmt_expr_attributes)]

#[macro_use]
extern crate pest_derive;

// Workaround for std NoneError from try_trait feature not converting to std Error
trait NoneError<T> {
	fn none_err(self) -> anyhow::Result<T>;
}
impl<T> NoneError<T> for Option<T> {
	fn none_err(self) -> anyhow::Result<T> {
		if let Some(value) = self {
			Ok(value)
		} else {
			Err(anyhow::anyhow!("Option is None"))
		}
	}
}

mod numbers;

mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;

aoc_runner_derive::aoc_lib! { year = 2017 }
