use std::collections::HashSet;

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

pub fn part_2() -> usize {
	INPUT
		.split(',')
		.map(|range| {
			let (start, end) = range.split_once('-').unwrap();
			(start.parse::<usize>().unwrap()..=end.parse().unwrap())
				.filter(|num| {
					let num_str = num.to_string();
                    let mut chars = num_str.chars();
                    let first = chars.next().unwrap();

                    let all_chars = HashSet::<char>::from_iter(chars);

					let mut matched_indices: Vec<_> = num_str
						.match_indices(first)
						.map(|m| m.0)
						.collect();

					if matched_indices.len() == 1 {
						return false;
					}
					if *matched_indices.last().unwrap() != num_str.len() {
						matched_indices.push(num_str.len());
					}

                    // FIXME: does not work when the same digit is repeated in sequence, example:
                    // 11851185
                    // TRY: merging windows until the window has all the distinct characters that
                    // the entire string has
					for window in matched_indices.windows(3) {
						if num_str[window[0]..window[1]] != num_str[window[1]..window[2]] {
							return false;
						}
					}

					true
				})
				.sum::<usize>()
		})
		.sum::<usize>()
}
