const INPUT: &str = include_str!("../input/day-2.txt");

pub fn part_1() -> usize {
	INPUT
		.split(',')
		.map(|range| {
			let (start, end) = range.split_once('-').unwrap();
			(start.parse::<usize>().unwrap()..=end.parse().unwrap())
				.filter(|num| {
					let num_str = num.to_string();
					let len = num_str.len();
					if len % 2 != 0 {
						return false;
					}
					let (first, second) = num_str.split_at(len / 2);
					first == second
				})
				.sum::<usize>()
		})
		.sum::<usize>()
}

pub fn part_2() -> u64 {
	INPUT
		.split(',')
		.map(|range| {
			let (start, end) = range.split_once('-').unwrap();
			(start.parse::<u64>().unwrap()..=end.parse().unwrap())
				.filter(|&n| is_repeating_sequence(n))
				.sum::<u64>()
		})
		.sum::<u64>()
}

const TEN: u64 = 10;

fn is_repeating_sequence(num: u64) -> bool {
	let total_digits = digits_in_numer(num);

	for digits_to_cmp in 1..=total_digits / 2 {
		if total_digits % digits_to_cmp != 0 {
			continue;
		}

		let mut cur = 0;

		let twice = digits_to_cmp + digits_to_cmp;

		let mut agg = true;

		while cur + twice <= total_digits {
			let next_cur = cur + digits_to_cmp;

			let first_mul = TEN.pow((total_digits - cur) as u32);
			let first =
				(num - (num / first_mul) * first_mul) / TEN.pow((total_digits - next_cur) as u32);

			let second_mul = TEN.pow((total_digits - next_cur) as u32);
			let second = (num - (num / second_mul) * second_mul)
				/ TEN.pow((total_digits - (next_cur + digits_to_cmp)) as u32);

			agg = agg && first == second;

			if !agg {
				break;
			}

			cur = next_cur;
		}
		if agg {
			return true;
		}
	}

	false
}

fn digits_in_numer(mut num: u64) -> u64 {
	let mut c = 0;

	while num != 0 {
		num /= TEN;
		c += 1;
	}

	c
}
