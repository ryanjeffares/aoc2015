use std::collections::BinaryHeap;

use crate::{Day, Timer};

pub struct Day2;

impl Day for Day2 {
    fn solve(&self) {
        let input = include_str!("../../inputs/day2.txt");

        {
            let _timer = Timer::new();
            let amount = input
                .split('\n')
                .map(|line| {
                    line.trim()
                        .split('x')
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<BinaryHeap<i32>>()
                        .into_sorted_vec()
                })
                .map(|nums| {
                    nums[0] * nums[1]
                        + 2 * nums[0] * nums[1]
                        + 2 * nums[0] * nums[2]
                        + 2 * nums[1] * nums[2]
                })
                .sum::<i32>();
            println!("Part 1: {amount}");
        }

        {
            let _timer = Timer::new();
            let amount = input
                .split('\n')
                .map(|line| {
                    line.trim()
                        .split('x')
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<BinaryHeap<i32>>()
                        .into_sorted_vec()
                })
                .map(|nums| 2 * nums[0] + 2 * nums[1] + nums[0] * nums[1] * nums[2])
                .sum::<i32>();
            println!("Part 1: {amount}");
        }
    }
}
