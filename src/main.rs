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

// Card vektor létrehozása
fn input_to_cards(card_input: Vec<String>) -> Vec<Card>{
    let mut cards: Vec<Card> = vec![];

    for i in card_input{
        let parts: Vec<&str> = i.split(':').collect();
        let card_id: i32 = parts[0]
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let numbers: Vec<i32> = parts[1]
            .split('|')
            .nth(0)
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        let winning_numbers: Vec<i32> = parts[1]
        .split('|')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

        let card = Card{
            card_id,
            numbers,
            winning_numbers,
            points: 0
        };

        cards.push(card)
    }
    
    return cards;
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
