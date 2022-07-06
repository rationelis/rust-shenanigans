use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Game {
    token: String,
    hangman: String,
}

#[derive(Serialize, Deserialize)]
struct GuessResult {
    hangman: String,
    correct: bool,
    token: String,
}

const API_URL: &str = "https://hangman-api.herokuapp.com/";

fn main() {
    let client = reqwest::blocking::Client::new();

    let mut game = start_game(&client);

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    loop {
        println!("{}", &game.hangman);

        let char = read_char();

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    
        if guess_letter(&client, &mut game, &char) {
            println!("Correct answer!\n");
        } else {
            println!("Incorrect answer!\n");
        }

        if !&game.hangman.contains('_') {
            break;
        }
    }

    println!("The word was: {:?}", &game.hangman);
    println!("You won!");
}

fn start_game(client: &reqwest::blocking::Client) -> Game {
    let request = client.post(format!("{}{}", API_URL, "hangman")).send();
    return serde_json::from_str(&request.unwrap().text().unwrap()).unwrap();
}

fn guess_letter(client: &reqwest::blocking::Client, game: &mut Game, char: &std::string::String) -> bool {
    let mut map = HashMap::new();
    map.insert("token", &game.token);
    map.insert("letter", char);

    let request = client
        .put(format!("{}{}", API_URL, "hangman"))
        .form(&map)
        .send();

    let guess_result: GuessResult =
        serde_json::from_str(&request.unwrap().text().unwrap()).unwrap();

    if guess_result.correct {
        for (i, c) in guess_result.hangman.chars().enumerate() {
            if c != '_' {
                game.hangman.replace_range(i..i+1, &c.to_string());
            }
        }
        return true;
    }

    return false;
}

fn read_char() -> String {
    println!("\nQuess: ");
    
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    let byte: u8 = input.bytes().nth(0).expect("No byte read");
    return String::from(std::str::from_utf8(&[byte]).unwrap());
}