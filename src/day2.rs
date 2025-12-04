const INPUT: &str = include_str!("../input/day-2.txt");

pub fn part_1() -> usize {
	INPUT
		.split(',')
		.map(|range| {
			let (start, end) = range.split_once('-').unwrap();
			(start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap())
				.filter(|num| {
					let num_str = num.to_string();
					let len = num_str.len();
					if len % 2 == 0 {
						let (first, second) = num_str.split_at(len / 2);
						return first == second;
					}
					false
				})
				.sum::<usize>()
		})
		.sum::<usize>()
}
