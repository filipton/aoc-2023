use std::{cmp::Ordering, collections::HashMap};

fn main() {
    part1();
}

const P1_LABELS: &str = "AKQJT98765432";
const P2_LABELS: &str = "AKQT98765432J";
fn part1() {
    let input = include_str!("input.txt");

    let mut cards = input.lines().map(|l| Card::new(l)).collect::<Vec<Card>>();
    cards.sort_by(|a, b| {
        if a.card_type == b.card_type {
            compare_cards(&a.card_string, &b.card_string, false)
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

    cards.sort_by(|a, b| {
        if a.joker_type == b.joker_type {
            compare_cards(&a.card_string, &b.card_string, true)
        } else {
            a.joker_type.cmp(&b.joker_type)
        }
    });

    let mut res = 0;
    let mut i = 1;

    for card in cards.iter().rev() {
        res += card.bid * i;
        i += 1;
    }

    println!("Part 2: {}", res);
}

fn compare_cards(a: &str, b: &str, part2: bool) -> Ordering {
    let labels = if part2 { P2_LABELS } else { P1_LABELS };

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
    joker_type: CardType,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
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

        let card_type = CardType::from_string(splitted[0]);
        let mut joker_type = card_type.clone();
        let mut tmp_card = splitted[0].to_string();

        for label in P2_LABELS.chars() {
            tmp_card = tmp_card.replace("J", &label.to_string());
            let ctype = CardType::from_string(&tmp_card);
            if ctype < joker_type {
                joker_type = ctype;
            }

            tmp_card = splitted[0].to_string();
        }

        Card {
            card_string: splitted[0].to_string(),
            bid,
            card_type,
            joker_type,
        }
    }
}

impl CardType {
    fn from_string(card: &str) -> Self {
        let mut groups = HashMap::new();
        for c in card.chars() {
            groups.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }

        if groups.len() == 1 {
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
        }
    }
}
