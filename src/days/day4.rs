use std::iter;

use crate::{Day, Timer};

pub struct Day4;

impl Day for Day4 {
    fn solve(&self) {
        let input = String::from("iwrupvqb");

        {
            let _timer = Timer::new();
            let number = iter::successors(Some(1), |n| Some(*n + 1))
                .find(|x| {
                    let md5 = md5::compute(format!("{input}{x}"));
                    md5.0[2] > 0 && md5.0[2] < 16 && md5.iter().take(2).all(|&b| b == 0)
                })
                .unwrap();
            println!("Part 1: {number}");
        }

        {
            let _timer = Timer::new();
            let number = iter::successors(Some(1), |n| Some(*n + 1))
                .find(|x| {
                    md5::compute(format!("{input}{x}"))
                        .0
                        .iter()
                        .take(3)
                        .all(|&b| b == 0)
                })
                .unwrap();
            println!("Part 2: {number}");
        }
    }
}
