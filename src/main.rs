use rand::Rng;
use std::{fmt, fmt::{Formatter}};
use ansi_term::Color;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Variants {
    Red,
    Purple,
    Yellow,
    Grey,
    Blue,
    Cyan,
    Orange,
    White,
    Empty,
}

impl fmt::Display for Variants {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            Variants::Red => format!("{}", Color::Red.paint("Red")),
            Variants::Purple => format!("{}", Color::Purple.paint("Purple")),
            Variants::Yellow => format!("{}", Color::Yellow.paint("Yellow")),
            Variants::Grey => format!("{}", Color::RGB(128, 128, 128).paint("Grey")),
            Variants::Blue => format!("{}", Color::Blue.paint("Blue")),
            Variants::Cyan => format!("{}", Color::Cyan.paint("Cyan")),
            Variants::Orange => format!("{}", Color::RGB(255, 128, 0).paint("Orange")),
            Variants::White => format!("{}", Color::White.paint("White")),
            Variants::Empty => panic!()
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug)]
struct Secret {
    color1: Variants,
    color2: Variants,
    color3: Variants,
    color4: Variants,
}

fn random_color() -> Variants {
    let mut rng = rand::thread_rng();
    match rng.gen_range(1..8) {
        1 => Variants::Red,
        2 => Variants::Purple,
        3 => Variants::Yellow,
        4 => Variants::Grey,
        5 => Variants::Blue,
        6 => Variants::Cyan,
        7 => Variants::Orange,
        _ => Variants::White,
    }
}

impl Secret {
    fn new() -> Secret {
        Secret {
            color1: random_color(),
            color2: random_color(),
            color3: random_color(),
            color4: random_color(),
        }
    }

    fn to_string(&self) -> String {
        format!("{} {} {} {}", self.color1, self.color2, self.color3, self.color4)
    }

    fn compare(&self, guess: Guess) -> (Guess, i32, i32) {
        let mut right_color = 0;
        let mut wrong_color = 0;
        let mut already_matched_as_wrong: Vec<&Variants> = Vec::new();
        if &self.color1 == &guess.color1 {
            right_color += 1;
        } else {
            for item in &guess.iter() {
                if item == &self.color1 && !already_matched_as_wrong.contains(&item) {
                    wrong_color += 1;
                    already_matched_as_wrong.push(item);
                    break;
                }
            }
        }
        if &self.color2 == &guess.color2 {
            right_color += 1;
        } else {
            for item in &guess.iter() {
                if item == &self.color2 {
                    wrong_color += 1;
                    break;
                }
            }
        }
        if &self.color3 == &guess.color3 {
            right_color += 1;
        } else {
            for item in &guess.iter() {
                if item == &self.color3 {
                    wrong_color += 1;
                    break;
                }
            }
        }
        if &self.color4 == &guess.color4 {
            right_color += 1;
        } else {
            for item in &guess.iter() {
                if item == &self.color4 {
                    wrong_color += 1;
                    break;
                }
            }
        }
        (guess, right_color, wrong_color)
    }
}

#[derive(Debug)]
struct GuessSequence {
    sequence: Vec<(Guess, i32, i32)>,
}

impl GuessSequence {
    fn new() -> Self {
        GuessSequence { sequence: Vec::default() }
    }

    fn push(&mut self, equality: (Guess, i32, i32)) {
        self.sequence.push((equality.0, equality.1, equality.2));
    }

    fn print(&self) -> String {
        let mut body = String::from("\n");
        for (guess_item, right_num, wrong_num) in &self.sequence {
            body.push_str(
                &format!(
                    "< Right={} {} {}=Wrong >\n",
                    right_num,
                    &*guess_item.to_string(),
                    wrong_num,
                )
            );
        }
        body
    }
}

#[derive(Debug)]
struct Guess {
    color1: Variants,
    color2: Variants,
    color3: Variants,
    color4: Variants,
}

impl Default for Guess {
    fn default() -> Self {
        Guess {
            color1: Variants::Empty,
            color2: Variants::Empty,
            color3: Variants::Empty,
            color4: Variants::Empty,
        }
    }
}

impl Guess {
    fn iter(&self) -> Vec<Variants> {
        let guess_vector: Vec<Variants> = vec!(
            *&self.color1.clone(),
            *&self.color2.clone(),
            *&self.color3.clone(),
            *&self.color4.clone(),
        );
        guess_vector
    }
}

fn match_color(color: &char) -> Variants {
    let uppercase_char = &color.to_uppercase().to_string()[..];
    match uppercase_char {
        "R" => Variants::Red,
        "P" => Variants::Purple,
        "Y" => Variants::Yellow,
        "G" => Variants::Grey,
        "B" => Variants::Blue,
        "C" => Variants::Cyan,
        "O" => Variants::Orange,
        _ => Variants::White,
    }
}

impl Guess {
    fn from(color_input: &str) -> Guess {
        Guess {
            color1: match_color(&color_input.chars().nth(0).unwrap()),
            color2: match_color(&color_input.chars().nth(1).unwrap()),
            color3: match_color(&color_input.chars().nth(2).unwrap()),
            color4: match_color(&color_input.chars().nth(3).unwrap()),
        }
    }

    fn to_string(&self) -> String {
        format!("{} {} {} {}", self.color1, self.color2, self.color3, self.color4)
    }
}

fn input_variants() -> String {
    println!("{}{}{}",
             "Please enter colours. You can enter the following characters:",
             "\nR => Red   P => Purple  Y => Yellow  C => Cyan\nB => Blue  ",
             "G => Grey    O => Orange  W => White");
    let mut ret = String::new();
    match std::io::stdin().read_line(&mut ret) {
        Ok(_) => {}
        Err(error) => println!("Error! {}", error),
    }
    ret.replace("\n", "").to_uppercase()
}

fn check_colors(color_input: &str) -> bool {
    let color_table = [
        'R', 'P', 'Y', 'C', 'B', 'G', 'O', 'W',
        'r', 'p', 'y', 'c', 'b', 'g', 'o', 'w'
    ];
    let mut all_colors_correct = true;
    for color in color_input.chars() {
        if !color_table.contains(&color) {
            println!("Wrong color {}", color.to_uppercase());
            all_colors_correct = false;
            break;
        }
    }
    if color_input.len() == 4 {
        all_colors_correct
    } else {
        println!("Wrong number of colors {}", color_input.len());
        false
    }
}

fn input_new_game() -> bool {
    let mut ret = String::new();
    match std::io::stdin().read_line(&mut ret) {
        Ok(_) => {}
        Err(error) => println!("Error! {}", error),
    }
    ret = ret.replace("\n", "");
    if ret == "Y" || ret == "y" { true } else { false }
}

fn main() {
    let mut secret = Secret::new();
    let mut guess_sequence = GuessSequence::new();
    let mut score = (1, 0, 0); // Round, wins, looses
    println!("\n{} {}",
             "Let's start the game.",
             "New Secret already generated, enter your color sequence!"
    );
    loop {
        println!(
            "Round: {} attempts left: {} Wins: {} Looses: {}",
            score.0, 12 - score.0, score.1, score.2
        );
        let some = input_variants();
        if check_colors(&some[..]) {
            score.0 += 1;
            let guess = Guess::from(&some);
            let equality_tuple = secret.compare(guess);
            if &equality_tuple.1 == &4 || score.0 > 12 {
                if &equality_tuple.1 == &4 || score.0 <= 12 {
                    score.1 += 1;
                    score.0 = 1;
                    println!(
                        "\nYou are win!!! Congratulations!!! \nSecret: {}\n Guess: {}",
                        secret.to_string(),
                        equality_tuple.0.to_string(),
                    );
                } else {
                    score.2 += 1;
                    score.0 = 1;
                    println!(
                        "\n{}{}\nSecret: {}\n Guess: {}",
                        "You have lost! \nYou have wasted all the attempts, ",
                        "next time you will surely guess the code!",
                        secret.to_string(),
                        equality_tuple.0.to_string(),
                    );
                }
                println!("Are you want to start new game? Y/N");
                if input_new_game() {
                    println!("{} {}",
                             "Let's start the game.",
                             "New Secret already generated, enter your color sequence!"
                    );
                    secret = Secret::new();
                } else {
                    println!("Thanks for playing, see you again!");
                    break;
                }
            } else {
                guess_sequence.push(equality_tuple);
                println!("Guess sequence:\n{}", guess_sequence.print());
            }
        }
    }
}
