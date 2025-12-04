const INPUT: &str = include_str!("../input/day-1.txt");

const INIT: u8 = 50;

pub fn part_1() -> usize {
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
		} else {
			if num > pos {
				pos += 100 - num;
			} else {
				pos -= num;
			}
		}
		if pos == 0 {
			passed_0_count += 1;
		}
	}
	passed_0_count
}

pub fn part_2() -> usize {
	let mut pos = INIT;

	let mut passed_0_count: usize = 0;

	for line in INPUT.lines() {
		let mut chars = line.chars();
		let direction = chars.next().unwrap();
		let parsed = chars.as_str().parse::<usize>().unwrap();

		if direction == 'R' {
			let t = pos as usize + parsed;
			passed_0_count += t / 100;
			pos = (t % 100) as u8;
		} else {
			if parsed >= pos as usize {
				let t = parsed - pos as usize;
				passed_0_count += t / 100 + if pos == 0 { 0 } else { 1 };

				let t = (parsed % 100) as u8;
				if t > pos {
					pos += 100 - t;
				} else {
					pos -= t;
				}
			} else {
				pos -= parsed as u8;
			}
		}
	}
	passed_0_count
}
