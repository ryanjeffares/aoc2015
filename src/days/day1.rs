use crate::{Day, Timer};

pub struct Day1;

impl Day for Day1 {
    fn solve(&self) {
        let input = include_str!("../../inputs/day1.txt");

        {
            let _timer = Timer::new();
            let floor = input
                .as_bytes()
                .iter()
                .fold(0, |acc, &c| if c == b'(' { acc + 1 } else { acc - 1 });
            println!("Part 1: {floor}");
        }

        {
            let _timer = Timer::new();
            let pos = input
                .as_bytes()
                .iter()
                .scan(0, |state, &c| {
                    if c == b'(' {
                        *state += 1;
                    } else {
                        *state -= 1;
                    }

                    if *state < 0 { None } else { Some(()) }
                })
                .count()
                + 1;
            println!("Part 2: {pos}");
        }
    }
}
