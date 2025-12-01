fn main() {}

mod aoc {
	pub fn day_1_part_1() {
		const INPUT: &str = include_str!("../input/day-1.txt");

		const INIT: u8 = 50;

		let mut pos = INIT;

		let mut passed_0_count: usize = 0;

		for line in INPUT.lines() {
			let mut chars = line.chars();
			let direction = chars.next().expect("should not be an empty line");
			let num = (chars.as_str().parse::<usize>().unwrap() % 100) as u8;

			if direction == 'R' {
				let sum = pos + num;
				if pos + num >= 100 {
					pos = sum % 100;
				} else {
					pos = sum;
				}
			} else if num > pos {
				pos += 100 - num;
			} else {
				pos -= num;
			}
			if pos == 0 {
				passed_0_count += 1;
			}
		}
	}
}
