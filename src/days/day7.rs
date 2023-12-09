use std::{io::BufRead, collections::{HashMap, BinaryHeap, VecDeque}, cmp::Ordering};

use crate::helpers::{self, Args};
use anyhow::{Context, Result};
use itertools::Itertools;
use log::info;

pub fn run(args: &mut Args) -> Result<()> {
    info!(target: "Day 7", "Running...");
    info!(target: "Day 7", "Parsing input from file");

    let reader = helpers::read_input_from_file(args)
        .with_context(|| "Problem with buffered file read")?;

    info!(target: "Day 7", "Solving...");

    let mut heap: BinaryHeap<Hand> = BinaryHeap::new();
    let mut heap_with_joker: BinaryHeap<HandWithJoker> = BinaryHeap::new();
    
    for line in reader.lines() {
        let values = line.with_context(|| "Couldn't read line")?;
        let raw_hand = values.split_whitespace().collect::<Vec<&str>>()[0].to_string();
        let bid = values.split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let hand_type = determine_type(&raw_hand.clone());
        let hand_type_with_joker = determine_type_with_joker(&raw_hand.clone());
        let value = raw_hand_to_values(&raw_hand.clone());
        let value_with_joker = raw_hand_to_values_with_joker(&raw_hand.clone());
        
        let hand = Hand {
            value,
            hand_type,
            bid,
        };

        let hand_with_joker = HandWithJoker {
            value: value_with_joker,
            hand_type: hand_type_with_joker,
            bid,
        };

        heap.push(hand);
        heap_with_joker.push(hand_with_joker);
    }
    
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;
    let mut number_of_hands = heap.len();
    while number_of_hands > 0 {
        let hand = heap.pop();
        let hand_with_joker = heap_with_joker.pop();
        total_part_1 += number_of_hands as i32 * hand.unwrap().bid;
        total_part_2 += number_of_hands as i32 * hand_with_joker.unwrap().bid;
        number_of_hands -= 1;
    }

    println!("Part 1: {}", total_part_1);
    println!("Part 2: {}", total_part_2);

    Ok(())
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum CardValueWithJoker {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}


#[derive(Debug, Eq)]
struct Hand {
    value: VecDeque<CardValue>,
    hand_type: HandType,
    bid: i32,
}

#[derive(Debug, Eq)]
struct HandWithJoker {
    value: VecDeque<CardValueWithJoker>,
    hand_type: HandType,
    bid: i32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.value == other.value
    }
}

impl PartialEq for HandWithJoker {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.value == other.value
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
       let compare_by_hand_type = self.hand_type.cmp(&other.hand_type);
       match compare_by_hand_type {
        Ordering::Greater => {Ordering::Greater},
        Ordering::Less => {Ordering::Less},
        Ordering::Equal => {self.value.cmp(&other.value)}
       }
    }
}

impl Ord for HandWithJoker {
    fn cmp(&self, other: &Self) -> Ordering {
       let compare_by_hand_type = self.hand_type.cmp(&other.hand_type);
       match compare_by_hand_type {
        Ordering::Greater => {Ordering::Greater},
        Ordering::Less => {Ordering::Less},
        Ordering::Equal => {self.value.cmp(&other.value)}
       }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for HandWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn raw_hand_to_values(hand: &str) -> VecDeque<CardValue> {
    let mut values: VecDeque<CardValue> = VecDeque::new();
    for c in hand.chars() {
        match c {
            '2' => values.push_back(CardValue::Two),
            '3' => values.push_back(CardValue::Three),
            '4' => values.push_back(CardValue::Four),
            '5' => values.push_back(CardValue::Five),
            '6' => values.push_back(CardValue::Six),
            '7' => values.push_back(CardValue::Seven),
            '8' => values.push_back(CardValue::Eight),
            '9' => values.push_back(CardValue::Nine),
            'T' => values.push_back(CardValue::Ten),
            'J' => values.push_back(CardValue::Jack),
            'Q' => values.push_back(CardValue::Queen),
            'K' => values.push_back(CardValue::King),
            'A' => values.push_back(CardValue::Ace),
            _ => panic!("Can't find a matching card value"),
        }
    }
    values
}

fn raw_hand_to_values_with_joker(hand: &str) -> VecDeque<CardValueWithJoker> {
    let mut values: VecDeque<CardValueWithJoker> = VecDeque::new();
    for c in hand.chars() {
        match c {
            '2' => values.push_back(CardValueWithJoker::Two),
            '3' => values.push_back(CardValueWithJoker::Three),
            '4' => values.push_back(CardValueWithJoker::Four),
            '5' => values.push_back(CardValueWithJoker::Five),
            '6' => values.push_back(CardValueWithJoker::Six),
            '7' => values.push_back(CardValueWithJoker::Seven),
            '8' => values.push_back(CardValueWithJoker::Eight),
            '9' => values.push_back(CardValueWithJoker::Nine),
            'T' => values.push_back(CardValueWithJoker::Ten),
            'J' => values.push_back(CardValueWithJoker::Joker),
            'Q' => values.push_back(CardValueWithJoker::Queen),
            'K' => values.push_back(CardValueWithJoker::King),
            'A' => values.push_back(CardValueWithJoker::Ace),
            _ => panic!("Can't find a matching card value"),
        }
    }
    values
}

fn determine_type(hand: &str) -> HandType {
    is_five_of_a_kind(hand)
}

fn determine_type_with_joker(hand: &str) -> HandType {
    let starting_type = is_five_of_a_kind(hand);
    if starting_type != HandType::FiveOfAKind {
        for c in hand.chars() {
            if c == 'J' {
                return get_hand_type_optimized(hand);
            }
        }
    }
    starting_type
}

fn get_hand_type_optimized(hand: &str) -> HandType {
    let map: HashMap<char, u32> = count_cards(hand);
    if map[&'J'] == 4 {
        return HandType::FiveOfAKind;
    }
    if map[&'J'] == 3 {
        if map.values().contains(&2) {
            return HandType::FiveOfAKind;
        }
        return HandType::FourOfAKind;
    }

    if map[&'J'] == 2 {
        if map.values().contains(&3) {
            return HandType::FiveOfAKind;
        }
        if map.values().contains(&2) {
            let mut count = 0;
            for (_, v) in map {
                if v == 2 {
                    count += 1;
                }
            }
            if count == 2 {
                return HandType::FourOfAKind;
            }
            return HandType::ThreeOfAKind;
        }
        return HandType::ThreeOfAKind;
    }

    if map[&'J'] == 1 {
        if map.values().contains(&4) {
            return HandType::FiveOfAKind;
        }
        if map.values().contains(&3) {
            return HandType::FourOfAKind;
        }
        if map.values().contains(&2) {
            let mut count = 0;
            for (_, v) in map {
                if v == 2 {
                    count += 1;
                }
            }
            if count == 2 {
                return HandType::FullHouse;
            }
            return HandType::ThreeOfAKind;
        }
    }
    HandType::OnePair
}

fn is_five_of_a_kind(hand: &str)-> HandType {
    let map: HashMap<char, u32> = count_cards(hand);
    if map.len() == 1 {
        return HandType::FiveOfAKind;
    }
    is_four_of_a_kind(hand)
}

fn is_four_of_a_kind(hand: &str) -> HandType {
    let map: HashMap<char, u32> = count_cards(hand);
    if map.values().contains(&4) {
        return HandType::FourOfAKind
    }
    is_full_house(hand)
}

fn is_full_house(hand: &str) -> HandType {
    let map: HashMap<char, u32> = count_cards(hand);
    if map.values().contains(&3) && map.values().contains(&2) {
        return HandType::FullHouse
    }
    is_three_of_a_kind(hand)
}

fn is_three_of_a_kind(hand: &str) -> HandType {
    let map: HashMap<char, u32> = count_cards(hand);
    if map.values().contains(&3) && map.values().contains(&1) {
        return HandType::ThreeOfAKind
    }
    is_two_pair(hand)
}

fn is_two_pair(hand: &str) -> HandType {
    let map: HashMap<char, u32> = count_cards(hand);
    let mut twos: i32 = 0;
    for value in map.values() {
        if value == &2 {
            twos += 1;
        }
    }
    if twos == 2 {
        return HandType::TwoPair;
    }
    is_one_pair(hand)
}

fn is_one_pair(hand: &str) -> HandType {
    let map: HashMap<char, u32> = count_cards(hand);
    let mut twos: i32 = 0;
    let mut threes: i32 = 0;
    for value in map.values() {
        if value == &2 {
            twos += 1;
        }
        if value == &3 {
            threes += 1;
        }
    }
    if twos == 1 && threes == 0 {
        return HandType::OnePair;
    }
    HandType::HighCard
}

fn count_cards(hand: &str) -> HashMap<char, u32> {
    let map: HashMap<char, u32> = hand
        .chars()
        .sorted()
        .into_group_map_by(|&x| x)
        .into_iter()
        .map(|(k, v)| (k, v.len() as u32))
        .collect();

    map
}