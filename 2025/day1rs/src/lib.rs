#[pyo3::pymodule]
mod day1rs {

use pyo3::prelude::*;
use regex::Regex;

#[pyclass]
struct Solver {
    lines: Vec<(char, i32)>,
}

#[pymethods]
impl Solver {
    #[new]
    fn new() -> Self {
        Self {
            lines: vec![],
        }
    }

    fn parse(&mut self, f: Bound<PyAny>) -> PyResult<()> {
        let re = Regex::new(r"^(L|R)(\d+)").unwrap();
        let pylines = f.call_method0("readlines")?;
        let lines = pylines.extract::<Vec<String>>()?;
        for line in lines {
            let cap = re.captures(&line).unwrap();
            let c = cap[1].chars().next().unwrap();
            let off = cap[2].parse()?;
            self.lines.push((c, off));
        }
        Ok(())
    }

    fn part1(&self) -> u16 {
        let mut zeros = 0;
        let mut now = 50;
        for (c, off) in &self.lines {
            match c {
                'L' => now -= off,
                'R' => now += off,
                _ => panic!(),
            }
            now = now.rem_euclid(100);
            if now == 0 {
                zeros += 1;
            }
        }
        zeros
    }

    fn part2(&self) -> i32 {
        let mut zeros = 0;
        let mut now: i32 = 50;
        for (c, off) in &self.lines {
            if off == &0 {
                continue;
            }
            match c {
                'L' => {
                    now -= 1;
                    zeros += now.div_euclid(100) - (now - off).div_euclid(100);
                    now += 1;
                    now -= off;
                }
                'R' => {
                    zeros += (now + off).div_euclid(100) - now.div_euclid(100);
                    now += off;
                }
                _ => panic!(),
            }
        }
        zeros
    }
}
}
