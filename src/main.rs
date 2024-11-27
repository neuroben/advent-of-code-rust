use std::fs::read_to_string;
use std::io;
use std::io::prelude::*;


struct Card{
    card_id: i32,
    numbers: Vec<i32>,
    winning_numbers: Vec<i32>,
    points: i32,
    winning_number_count: i32
}

impl Card{
    fn calculate_points(&mut self){
        let mut sum: i32 = 0;
        let mut winning_number_count: i32 = 0;

        for i in &self.numbers{
            for j in &self.winning_numbers{
                if i == j{
                    if sum == 0{
                        sum += 1;
                    }
                    else{
                        sum *= 2;
                    }
                    // A második feladat részhez a nyertes számok, számának meghatározása
                    winning_number_count += 1;
                }
            }
        }
        self.winning_number_count = winning_number_count;
        self.points = sum;
    }
}

fn main() {
    let input_cards = input_read(None);

    let cards = input_to_cards(input_cards);

    let first_part_result = first_part(cards);

    println!("1. part result: {}",first_part_result);
    
}




// --- First part --- //
fn first_part(cards: Vec<Card>) -> i32{
    let mut sum: i32 = 0;

    for mut card in cards{
        card.calculate_points();
        sum += card.points;
    }

    sum
}
// Card obijektumok létrehozása
fn input_to_cards(card_input: Vec<String>) -> Vec<Card> {
    card_input
        .into_iter()
        .map(parse_card)
        .collect() // Iterátor lánc, a Card-ok vektorát adja vissza
}
// Card vektor létrehozása
fn parse_card(input: String) -> Card {
    let parts: Vec<&str> = input.split(':').collect();

    let card_id: i32 = parts[0]
    .split_whitespace()
    .nth(1)
    .unwrap()
    .parse()
    .unwrap();

    let (numbers, winning_numbers) = parse_number(parts[1]);

    Card {
        card_id,
        numbers,
        winning_numbers,
        points: 0
    }


}
// A szám sztringek átalakítása
fn parse_number(numbers_input: &str) -> (Vec<i32>, Vec<i32>) {
    let parts: Vec<&str> = numbers_input.split('|').collect();

    let numbers = parts[0]
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    let winning_numbers = parts[1]
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    (numbers, winning_numbers)
}
// Option típusú paraméter
fn input_read(file_path: Option<&str>) -> Vec<String>{
    let file_path = file_path.unwrap_or("input/input4.txt");
    read_to_string(file_path)
        .unwrap()
        .lines()
        //.inspect(|line| println!("{}",line))
        .map(String::from)
        .collect()
}