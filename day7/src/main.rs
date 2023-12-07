use std::{collections::HashMap, cmp::Ordering};

fn main() {
    part1();
}

const labels: &str = "AKQJT98765432";
fn part1() {
    let input = include_str!("input.txt");

    let mut cards = input.lines().map(|l| Card::new(l)).collect::<Vec<Card>>();
    cards.sort_by(|a, b| {
        if a.card_type == b.card_type {
            compare_cards(&a.card_string, &b.card_string)
        } else {
            a.card_type.cmp(&b.card_type)
        }
    });

    let mut res = 0;
    let mut i = 1;

    for card in cards.iter().rev() {
        res += card.bid * i;
        i += 1;
    }

    println!("Part 1: {}", res);
}

fn compare_cards(a: &str, b: &str) -> Ordering {
    for (i, c) in a.chars().enumerate() {
        let a_index = labels.find(c).unwrap();
        let b_index = labels.find(b.chars().nth(i).unwrap()).unwrap();
        if a_index > b_index {
            return Ordering::Greater;
        } else if a_index < b_index {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Card {
    card_string: String,
    bid: usize,
    card_type: CardType,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum CardType {
    FiveKind = 1,
    FourKind = 2,
    FullHouse = 3,
    ThreeKind = 4,
    TwoPair = 5,
    OnePair = 6,
    HighCard = 7,
}

impl Card {
    fn new(card: &str) -> Self {
        let splitted = card.split(" ").collect::<Vec<&str>>();
        let bid = splitted[1].parse::<usize>().unwrap();

        let mut groups = HashMap::new();
        for c in splitted[0].chars() {
            groups.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }

        let ctype = if groups.len() == 1 {
            CardType::FiveKind
        } else if groups.iter().any(|(_, v)| *v == 4) {
            CardType::FourKind
        } else if groups.iter().any(|(_, v)| *v == 3) && groups.iter().any(|(_, v)| *v == 2) {
            CardType::FullHouse
        } else if groups.iter().any(|(_, v)| *v == 3) {
            CardType::ThreeKind
        } else if groups.iter().filter(|(_, v)| *v == &2).count() == 2 {
            CardType::TwoPair
        } else if groups.iter().filter(|(_, v)| *v == &2).count() == 1 {
            CardType::OnePair
        } else {
            CardType::HighCard
        };

        Card {
            card_string: splitted[0].to_string(),
            bid,
            card_type: ctype,
        }
    }
}
