mod utils;

use utils::get_file;

#[derive(PartialEq, Eq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("Invalid move '{}'", c),
        }
    }
}

impl Move {
    fn get_points(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn get_winner(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn get_loser(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn get_draw(&self) -> Move {
        match self { Move::Rock => Move::Rock, Move::Paper => Move::Paper, Move::Scissors => Move::Scissors }
    }

    fn get_match_points(&self, against: &Move) -> i32 {
        if against.get_winner() == *self {
            6
        } else if against.get_loser() == *self {
            0
        } else {
            3
        }
    }

    fn play(self: &Move, against: &Move) -> i32 {
        self.get_points() + self.get_match_points(against)
    }

    fn play2(self: &Move, against: &Move) -> i32 {
        let my_move = match self {
            // Scissors = Z = win
            Move::Scissors => against.get_winner(),
            // Paper = Y = tie
            Move::Paper => against.get_draw(),
            // Rock = X = lose
            Move::Rock => against.get_loser(),
        };

        my_move.play(against)
    }
}

fn main() {
    let input_str = get_file("day2.input");

    let mut score = 0;
    let mut score2 = 0;
    
    for line in input_str.trim().split("\n") {
        let mut chars = line.chars();

        let opp_move = Move::from(chars.nth(0).unwrap());
        let my_move = Move::from(chars.nth(1).unwrap());

        score += my_move.play(&opp_move);
        score2 += my_move.play2(&opp_move);
    }

    println!("part 1 => {}", score);
    println!("part 2 => {}", score2);
}


