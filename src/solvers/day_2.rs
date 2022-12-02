trait Score {
    fn score(&self) -> i32;
}

#[derive(Debug)]
enum GameState {
	Win,
	Loss,
	Tie
}

impl Score for GameState {
	fn score(&self) -> i32 {
		match &self {
			GameState::Win => 6,
			GameState::Tie => 3,
			GameState::Loss => 0
		}
	}
}

impl GameState {
	fn my_result_from_choices(my_choise: &RPSChoice, opp_choice: &RPSChoice) -> Self {
		match (my_choise, opp_choice) {
			(RPSChoice::Rock, RPSChoice::Rock) => GameState::Tie,
			(RPSChoice::Rock, RPSChoice::Paper) => GameState::Loss,
			(RPSChoice::Rock, RPSChoice::Scissors) => GameState::Win,
			(RPSChoice::Paper, RPSChoice::Rock) => GameState::Win,
			(RPSChoice::Paper, RPSChoice::Paper) => GameState::Tie,
			(RPSChoice::Paper, RPSChoice::Scissors) => GameState::Loss,
			(RPSChoice::Scissors, RPSChoice::Rock) => GameState::Loss,
			(RPSChoice::Scissors, RPSChoice::Paper) => GameState::Win,
			(RPSChoice::Scissors, RPSChoice::Scissors) => GameState::Tie
		}
	}

	fn my_choice_from_opponent_and_outcome(opp_choice: &RPSChoice, outcome: &GameState) -> RPSChoice {
		match (opp_choice, outcome) {
			(RPSChoice::Rock, GameState::Tie) => RPSChoice::Rock,
			(RPSChoice::Rock, GameState::Win) => RPSChoice::Paper,
			(RPSChoice::Rock, GameState::Loss) => RPSChoice::Scissors,
			(RPSChoice::Paper, GameState::Tie) => RPSChoice::Paper,
			(RPSChoice::Paper, GameState::Win) => RPSChoice::Scissors,
			(RPSChoice::Paper, GameState::Loss) => RPSChoice::Rock,
			(RPSChoice::Scissors, GameState::Tie) => RPSChoice::Scissors,
			(RPSChoice::Scissors, GameState::Win) => RPSChoice::Rock,
			(RPSChoice::Scissors, GameState::Loss) => RPSChoice::Paper,
		}
	}
}

impl From<&str> for GameState {
	fn from(item: &str) -> Self {
		match item {
			"X" => GameState::Loss,
			"Y" => GameState::Tie,
			"Z" => GameState::Win,
			_ => panic!("Could not match type to RPS value")
		}
	}
}

#[derive(Debug)]
enum RPSChoice {
	Rock,
	Paper,
	Scissors
}

impl From<&str> for RPSChoice {
	fn from(item: &str) -> Self {
		match item {
			"A" | "X" => RPSChoice::Rock,
			"B" | "Y" => RPSChoice::Paper,
			"C" | "Z" => RPSChoice::Scissors,
			_ => panic!("Could not match type to RPS value")
		}
	}
}


impl Score for RPSChoice {
	fn score(&self) -> i32 {
		match &self {
			RPSChoice::Rock => 1,
			RPSChoice::Paper => 2,
			RPSChoice::Scissors => 3
		}
	}
}

#[derive(Debug)]
struct RPSGame {
	opponent_choice: RPSChoice,
	my_choice: RPSChoice,
	game_state: GameState
}

impl From<&str> for RPSGame {
    fn from(item: &str) -> Self {
		let split: Vec<&str> = item.split_whitespace().collect();
		
		let opp_choice = RPSChoice::from(split[0]);
		let my_choice = RPSChoice::from(split[1]);
		let result = GameState::my_result_from_choices(&my_choice, &opp_choice);

		RPSGame {
			opponent_choice: RPSChoice::from(split[0]),
			my_choice: RPSChoice::from(split[1]),
			game_state: result
		}
    }
}

impl Score for RPSGame {
	fn score(&self) -> i32 {
		self.my_choice_score() + self.game_outsome_score()
	}
}

impl RPSGame {
	fn my_choice_score(&self) -> i32 {
		self.my_choice.score()
	}

	fn game_outsome_score(&self) -> i32 {
		self.game_state.score()
	}

	fn from_choice_and_state(opp_choice: RPSChoice, result: GameState) -> Self {
		let my_choice = GameState::my_choice_from_opponent_and_outcome(&opp_choice, &result);
		RPSGame {
			opponent_choice: opp_choice,
			my_choice: my_choice,
			game_state: result
		}
	}
}

pub fn solve_day_2(contents: String) {
	println!("Solving Day 2");
	solve_part_1(&contents);
	solve_part_2(&contents);

}

fn solve_part_1(contents: &String) {
	let games: Vec<RPSGame> = create_games_from_contents(contents);

	let my_score_part_1: i32 = games.iter().fold(0, |score, game| score + game.score());

	println!("Day 2 Part 1 Answer: {}", my_score_part_1);
}


fn solve_part_2(contents: &String) {
	let games: Vec<RPSGame> = {
		let mut games = vec!();
		for game_data in contents.split("\n") {
			let split: Vec<&str> = game_data.split_whitespace().collect();
			
			let opp_choice = RPSChoice::from(split[0]);
			let result = GameState::from(split[1]);
			games.push(RPSGame::from_choice_and_state(opp_choice, result));
		}

		games
	};

	let my_score_part_2: i32 = games.iter().fold(0, |score, game| score + game.score());

	println!("Day 2 Part 2 Answer: {}", my_score_part_2);
}


fn create_games_from_contents(contents: &String) -> Vec<RPSGame> {
	let mut games = vec!();
	for game_data in contents.split("\n") {
		games.push(RPSGame::from(game_data));
	}

	games
}