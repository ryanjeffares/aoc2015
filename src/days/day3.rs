use std::collections::{HashMap, HashSet};

use crate::{Day, Timer};

pub struct Day3;

impl Day for Day3 {
    fn solve(&self) {
        let input = include_str!("../../inputs/day3.txt");

        {
            let _timer = Timer::new();

            let mut houses = HashMap::from([(0, HashSet::from([0]))]);
            let (mut x, mut y) = (0, 0);

            input.as_bytes().iter().for_each(|&c| {
                match c {
                    b'>' => x += 1,
                    b'<' => x -= 1,
                    b'^' => y += 1,
                    b'v' => y -= 1,
                    _ => (),
                };

                houses
                    .entry(x)
                    .and_modify(|set| {
                        set.insert(y);
                    })
                    .or_insert(HashSet::from([y]));
            });

            let num_houses = houses.iter().map(|(_, set)| set.len()).sum::<usize>();
            println!("Part 1: {num_houses}");
        }

        {
            let _timer = Timer::new();

            let mut houses = HashMap::from([(0, HashSet::from([0]))]);
            let (mut santa_x, mut santa_y, mut robo_x, mut robo_y) = (0, 0, 0, 0);

            input.as_bytes().chunks(2).for_each(|c| {
                match c[0] {
                    b'>' => santa_x += 1,
                    b'<' => santa_x -= 1,
                    b'^' => santa_y += 1,
                    b'v' => santa_y -= 1,
                    _ => (),
                };

                match c[1] {
                    b'>' => robo_x += 1,
                    b'<' => robo_x -= 1,
                    b'^' => robo_y += 1,
                    b'v' => robo_y -= 1,
                    _ => (),
                };

                houses
                    .entry(santa_x)
                    .and_modify(|set| {
                        set.insert(santa_y);
                    })
                    .or_insert(HashSet::from([santa_y]));
                houses
                    .entry(robo_x)
                    .and_modify(|set| {
                        set.insert(robo_y);
                    })
                    .or_insert(HashSet::from([robo_y]));
            });

            let num_houses = houses.iter().map(|(_, set)| set.len()).sum::<usize>();
            println!("Part 2: {num_houses}");
        }
    }
}
