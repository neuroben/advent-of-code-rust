use std::fs::read_to_string;

struct Card{
    card_id: i32,
    numbers: Vec<i32>,
    winning_numbers: Vec<i32>,
    points: i32
}

fn main() {
    let input_cards = input_read(None);

    let cards = input_to_cards(input_cards);

    for i in cards{
        println!("gameID: {}, numbers: {:?}, winning_numbers: {:?}",i.card_id,i.numbers,i.winning_numbers)
    }
}

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
        .inspect(|line| println!("{}",line))
        .map(String::from)
        .collect()
}
