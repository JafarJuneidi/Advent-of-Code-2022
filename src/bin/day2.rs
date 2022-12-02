use std::str::FromStr;

#[derive(Debug)]
enum Sign {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum SignConversionErr {
    WrongInput,
}

impl FromStr for Sign {
    type Err = SignConversionErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Sign::Rock),
            "B" | "Y" => Ok(Sign::Paper),
            "C" | "Z" => Ok(Sign::Scissors),
            _ => Err(SignConversionErr::WrongInput),
        }
    }
}

impl Sign {
    fn result(self: &Self, opponent: &Self) -> u16 {
        match self {
            Sign::Rock => match opponent {
                Sign::Rock => 3 + 1,
                Sign::Paper => 0 + 1,
                Sign::Scissors => 6 + 1,
            },
            Sign::Paper => match opponent {
                Sign::Rock => 6 + 2,
                Sign::Paper => 3 + 2,
                Sign::Scissors => 0 + 2,
            },
            Sign::Scissors => match opponent {
                Sign::Rock => 0 + 3,
                Sign::Paper => 6 + 3,
                Sign::Scissors => 3 + 3,
            },
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

#[derive(Debug)]
enum OutcomeConversionErr {
    WrongInput,
}

impl FromStr for Outcome {
    type Err = OutcomeConversionErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(OutcomeConversionErr::WrongInput),
        }
    }
}

impl Outcome {
    fn result(self: &Self, opponent: &Sign) -> u16 {
        match self {
            Outcome::Win => match opponent {
                Sign::Rock => 6 + 2,
                Sign::Paper => 6 + 3,
                Sign::Scissors => 6 + 1,
            },
            Outcome::Draw => match opponent {
                Sign::Rock => 3 + 1,
                Sign::Paper => 3 + 2,
                Sign::Scissors => 3 + 3,
            },
            Outcome::Loss => match opponent {
                Sign::Rock => 0 + 3,
                Sign::Paper => 0 + 1,
                Sign::Scissors => 0 + 2,
            },
        }
    }
}

fn str_to_sign_outcome(str: &str) -> (Sign, Outcome) {
    let str: Vec<&str> = str.split(" ").collect();
    (str[0].parse().unwrap(), str[1].parse().unwrap())
}

fn main() {
    let output: u16 = include_str!("./day2.txt")
        .lines()
        .map(|round| {
            // let vector: Vec<Sign> = round.split(" ").flat_map(str::parse::<Sign>).collect();
            // vector[1].result(&vector[0])
            let tuple = str_to_sign_outcome(round);
            tuple.1.result(&tuple.0)
        })
        .sum();

    println!("{:?}", output);
}
