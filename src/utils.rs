use std::fs;
use std::path::Path;

const DATASET_PATH: &str = "datasets";

pub fn read_day_dataset(day: &str) -> String {
	let file_path = Path::new(DATASET_PATH).join(format!("{}.txt", day));
	println!("Reading Date in file {:?}", file_path);

	fs::read_to_string(file_path).expect("Could not read dataset file")
}