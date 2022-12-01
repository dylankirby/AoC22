use std::cmp::Ordering;

#[derive(Debug)]
struct Elf {
	food_calorie_set: Vec<u32>
}

impl Elf {

	fn new(food_calorie_set: Vec<String>) -> Elf {
		let calories_as_u8 = food_calorie_set.iter().map(|calorie| calorie.parse::<u32>().unwrap()).collect();
		Elf {food_calorie_set: calories_as_u8}
	}

	fn total_calories(&self) -> u32 {
		self.food_calorie_set.iter().sum()
	}
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_calories().cmp(&other.total_calories())
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.total_calories() == other.total_calories()
    }
}

impl Eq for Elf { }

pub fn solve_day_1(contents: String) {
	println!("Solving Day 1");

	let elves = create_elves_from_dataset_contents(contents);

	let part_one = elves[0].total_calories();
	println!("Part one answer {}", part_one);

	let part_two = &elves[..3].iter().fold(0, |acc, elf| acc + elf.total_calories());

	println!("Part two Answer {}", part_two);
}

fn create_elves_from_dataset_contents(contents: String) -> Vec<Elf> {
	let mut elves = vec!();
	for calorie_set in contents.split("\n\n") {
		let c: Vec<String> = calorie_set.split("\n")
	    .map(|line| line.to_string())
	    .collect();

		elves.push(Elf::new(c));
	}

	elves.sort();
	elves.reverse();
	elves
}
