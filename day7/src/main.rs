use itertools::Itertools;
use std::collections::HashMap;

fn classify_hand(hand: &str) -> &str {
    let mut frequencies: HashMap<char, u8> = HashMap::new();
    for card in hand.chars() {
        let count = frequencies.get(&card).unwrap_or(&0);
        frequencies.insert(card, count + 1);
    }

    match frequencies.len() {
        1 => "fiveofakind",
        2 => {
            let mut values: Vec<_> = frequencies.values().collect();
            values.sort();
            match values.as_slice() {
                [1, 4] => "fourofakind",
                [2, 3] => "fullhouse",
                _ => panic!("invalid hand"),
            }
        }
        3 => {
            let mut values: Vec<_> = frequencies.values().collect();
            values.sort();
            match values.as_slice() {
                [1, 1, 3] => "threeofakind",
                [1, 2, 2] => "twopair",
                _ => panic!("invalid hand"),
            }
        }
        4 => "onepair",
        5 => "highcard",
        _ => panic!("invalid hand"),
    }
}

fn classify_hand_with_jokers(hand: &str) -> String {
    // check if any of the characters is J
    // if there is at least one J, then figure out what cards the Js can be replaced by to end up in the strongest bucket
    // if there are no Js, then just classify the hand normally

    let num_jokers = hand.matches('J').count();
    if num_jokers > 0 {
        let cards = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];
        let mut strongest_hand = "highcard".to_string();
        let hand_strengths = [
            "highcard",
            "onepair",
            "twopair",
            "threeofakind",
            "fullhouse",
            "fourofakind",
            "fiveofakind",
        ];

        // Generate all combinations of replacements for the jokers
        let replacements = cards.iter().combinations_with_replacement(num_jokers);

        for replacement in replacements {
            let mut replaced_hand = hand.to_string();
            for &card in &replacement {
                replaced_hand = replaced_hand.replacen('J', &card.to_string(), 1);
            }

            let hand_strength = classify_hand(&replaced_hand);

            // Compare hand_strength with current strongest_hand and update if stronger
            let current_strength_idx = hand_strengths
                .iter()
                .position(|&r| r == hand_strength)
                .unwrap_or(0);
            let strongest_hand_idx = hand_strengths
                .iter()
                .position(|&r| r == strongest_hand)
                .unwrap_or(0);

            if current_strength_idx > strongest_hand_idx {
                strongest_hand = hand_strength.to_string();
            }
        }
        strongest_hand
    } else {
        classify_hand(hand).to_string()
    }
}

fn bucket_and_sort_with_jokers(hands_and_bids: Vec<(&str, u16)>) -> Vec<(&str, u16)> {
    let mut buckets = HashMap::from([
        ("highcard", vec![]),
        ("onepair", vec![]),
        ("twopair", vec![]),
        ("threeofakind", vec![]),
        ("fullhouse", vec![]),
        ("fourofakind", vec![]),
        ("fiveofakind", vec![]),
    ]);

    for (hand, bid) in hands_and_bids {
        let hand_type = classify_hand_with_jokers(hand);
        match buckets.get_mut(hand_type.as_str()) {
            Some(bucket) => bucket.push((hand, bid)),
            None => panic!("invalid hand type"),
        }
    }

    let card_scores = HashMap::from([
        ('J', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);

    for bucket in buckets.values_mut() {
        bucket.sort_by(|(hand1, _bid1), (hand2, _bid2)| {
            // compare character by character
            let hand1_chars: Vec<_> = hand1.chars().collect();
            let hand2_chars: Vec<_> = hand2.chars().collect();

            for i in 0..5 {
                let card1 = hand1_chars[i];
                let card2 = hand2_chars[i];

                let card1_score = card_scores.get(&card1).unwrap();
                let card2_score = card_scores.get(&card2).unwrap();

                if card1_score > card2_score {
                    return std::cmp::Ordering::Greater;
                } else if card1_score < card2_score {
                    return std::cmp::Ordering::Less;
                }
            }

            // if we get here, the hands are equal
            std::cmp::Ordering::Equal
        });
    }

    // now join the buckets back into a single vector
    let mut sorted_hands_and_bids = vec![];
    sorted_hands_and_bids.extend(buckets.get("highcard").unwrap());
    sorted_hands_and_bids.extend(buckets.get("onepair").unwrap());
    sorted_hands_and_bids.extend(buckets.get("twopair").unwrap());
    sorted_hands_and_bids.extend(buckets.get("threeofakind").unwrap());
    sorted_hands_and_bids.extend(buckets.get("fullhouse").unwrap());
    sorted_hands_and_bids.extend(buckets.get("fourofakind").unwrap());
    sorted_hands_and_bids.extend(buckets.get("fiveofakind").unwrap());

    sorted_hands_and_bids
}

fn bucket_and_sort(hands_and_bids: Vec<(&str, u16)>) -> Vec<(&str, u16)> {
    let mut buckets = HashMap::from([
        ("highcard", vec![]),
        ("onepair", vec![]),
        ("twopair", vec![]),
        ("threeofakind", vec![]),
        ("fullhouse", vec![]),
        ("fourofakind", vec![]),
        ("fiveofakind", vec![]),
    ]);

    for (hand, bid) in hands_and_bids {
        let hand_type = classify_hand(hand);
        match buckets.get_mut(hand_type) {
            Some(bucket) => bucket.push((hand, bid)),
            None => panic!("invalid hand type"),
        }
    }

    // now sort the buckets internally from weakest to strongest

    let card_scores = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);

    for bucket in buckets.values_mut() {
        bucket.sort_by(|(hand1, _bid1), (hand2, _bid2)| {
            // compare character by character
            let hand1_chars: Vec<_> = hand1.chars().collect();
            let hand2_chars: Vec<_> = hand2.chars().collect();

            for i in 0..5 {
                let card1 = hand1_chars[i];
                let card2 = hand2_chars[i];

                let card1_score = card_scores.get(&card1).unwrap();
                let card2_score = card_scores.get(&card2).unwrap();

                if card1_score > card2_score {
                    return std::cmp::Ordering::Greater;
                } else if card1_score < card2_score {
                    return std::cmp::Ordering::Less;
                }
            }

            // if we get here, the hands are equal
            std::cmp::Ordering::Equal
        });
    }

    // now join the buckets back into a single vector
    let mut sorted_hands_and_bids = vec![];
    sorted_hands_and_bids.extend(buckets.get("highcard").unwrap());
    sorted_hands_and_bids.extend(buckets.get("onepair").unwrap());
    sorted_hands_and_bids.extend(buckets.get("twopair").unwrap());
    sorted_hands_and_bids.extend(buckets.get("threeofakind").unwrap());
    sorted_hands_and_bids.extend(buckets.get("fullhouse").unwrap());
    sorted_hands_and_bids.extend(buckets.get("fourofakind").unwrap());
    sorted_hands_and_bids.extend(buckets.get("fiveofakind").unwrap());

    sorted_hands_and_bids
}

fn part1() {
    let input = include_str!("./input.txt");

    let hands_and_bids_raw: Vec<_> = input.split("\n").collect();
    let hands_and_bids: Vec<(&str, u16)> = hands_and_bids_raw
        .iter()
        .map(|line| {
            let split: Vec<_> = line.split_whitespace().collect();
            let hand = split[0];
            let bid = split[1].parse::<u16>().unwrap();
            (hand, bid)
        })
        .collect();

    let sorted_hands_and_bids = bucket_and_sort(hands_and_bids);

    // multiply each bid by its rank (which is its index + 1)
    let mut total = 0;
    for (i, (_hand, bid)) in sorted_hands_and_bids.iter().enumerate() {
        total += (i + 1) * (*bid as usize);
    }

    println!("part 1: {}", total);
}

fn part2() {
    let input = include_str!("./input.txt");
    let hands_and_bids_raw: Vec<_> = input.split("\n").collect();
    let hands_and_bids: Vec<(&str, u16)> = hands_and_bids_raw
        .iter()
        .map(|line| {
            let split: Vec<_> = line.split_whitespace().collect();
            let hand = split[0];
            let bid = split[1].parse::<u16>().unwrap();
            (hand, bid)
        })
        .collect();

    let sorted_hands_and_bids = bucket_and_sort_with_jokers(hands_and_bids);

    let mut total = 0;
    for (i, (_hand, bid)) in sorted_hands_and_bids.iter().enumerate() {
        total += (i + 1) * (*bid as usize);
    }

    println!("part 2: {}", total);
}

fn main() {
    part1();
    part2();
}
