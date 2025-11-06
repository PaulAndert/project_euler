use std::fs;
use std::cmp;
use itertools::Itertools;

// 9 : High Card: Highest value card.
//- 8 : One Pair: Two cards of the same value.
//- 7 : Two Pairs: Two different pairs.
//- 6 : Three of a Kind: Three cards of the same value.
//- 5 : Straight: All cards are consecutive values.
//- 4 : Flush: All cards of the same suit.
//- 3 : Full House: Three of a kind and a pair.
//- 2 : Four of a Kind: Four cards of the same value.
//- 1 : Straight Flush: All cards are consecutive values of same suit.
//- 0 : Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.

// The cards are valued in the order:
// 2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.
// 2, 3, 4, 5, 6, 7, 8, 9, T, J, Q, K, A

#[derive(Debug, Eq, Clone)]
struct Card {
    value: u64,
    // 2,3,4,5,6,7,8,9,10,11,12,13,14
    suit: char,
    // C_lubs, S_pades, H_earts, D_iamonds
}
impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.value == other.value
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<cmp::Ordering> {
        Some(other.cmp(self))
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Card) -> cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

fn str_to_card(card_str: &str) -> Card {
    let mut chars = card_str.chars();
    let opt = chars.next().unwrap();
    let num: u64 = match opt {
        '0'..='9' => opt as u64 - 48,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => {
            println!("Error");
            0
        }
    };
    return Card {
        value: num,
        suit: chars.next().unwrap(),
    };
}

fn get_number_of_kinds(cards: Vec<Card>) -> Vec<u64> {
    let mut return_vec: Vec<u64> = vec![0; 15];
    for c in cards {
        return_vec[c.value as usize] += 1;
    }
    return return_vec;
}

fn check_flush(cards: Vec<Card>) -> bool {
    if cards.iter().map(|c| c.suit).unique().map(|_| 1).sum::<u64>() == 1 {
        return true;
    }else {
        return false;
    }
}

fn check_straight(mut cards: Vec<Card>) -> bool {
    cards.sort();
    cards.reverse();
    let mut before: u64 = cards[0].value + 1;
    for i in 1..cards.len() {
        if before == cards[i].value {
            before += 1;
        }else {
            return false;
        }
    }
    return true;
}

fn get_bigger(mut cards_p1: Vec<Card>, mut cards_p2: Vec<Card>) -> u64 {
    cards_p1.sort();
    cards_p2.sort();
    for i in 0..5 {
        if cards_p1[i].value < cards_p2[i].value {
            return 2;
        }
        if cards_p1[i].value > cards_p2[i].value {
            return 1;
        }
    }
    return 0;
}

fn compare(cards_p1: Vec<Card>, cards_p2: Vec<Card>) -> u64 { // return 0 -> error, 1 -> P1, 2 -> P2

    // Player 1
    let flush_p1: bool = check_flush(cards_p1.clone());
    let straight_p1: bool = check_straight(cards_p1.clone());
    let kinds_p1: Vec<u64> = get_number_of_kinds(cards_p1.clone());
    // Player 2
    let flush_p2: bool = check_flush(cards_p2.clone());
    let straight_p2: bool = check_straight(cards_p2.clone());
    let kinds_p2: Vec<u64> = get_number_of_kinds(cards_p2.clone());

    
    if flush_p1 && straight_p1 && cards_p1[4].value == 10 {
        if flush_p2 && straight_p2 && cards_p2[4].value == 10 {
            return 0;
        }else {
            return 1;
        }
    }
    if flush_p2 && straight_p2 && cards_p2[4].value == 10 {
        return 2;
    }

    if flush_p1 && straight_p1 {
        if flush_p2 && straight_p2 {
            return get_bigger(cards_p1, cards_p2);
        }else {
            return 1;
        }
    }
    if flush_p2 && straight_p2 {
        return 2;
    }

    if kinds_p1.contains(&4) {
        if kinds_p2.contains(&4) {
            if kinds_p1.iter().position(|&r| r == 4).unwrap() > kinds_p2.iter().position(|&p| p == 4).unwrap() {
                return 1;
            }else {
                return 2;
            }
        }else {
            return 1;
        }
    }
    if kinds_p2.contains(&4) {
        return 2;
    }

    if kinds_p1.contains(&3) && kinds_p1.contains(&2) {
        if kinds_p2.contains(&3) && kinds_p2.contains(&2) {
            let mut index_p1 = kinds_p1.iter().position(|&r| r == 3).unwrap();
            let mut index_p2 = kinds_p2.iter().position(|&r| r == 3).unwrap();
            if index_p1 > index_p2 {
                return 1;
            }else if index_p1 < index_p2 {
                return 2;
            }else {
                index_p1 = kinds_p1.iter().position(|&r| r == 2).unwrap();
                index_p2 = kinds_p2.iter().position(|&r| r == 2).unwrap();
                if index_p1 > index_p2 {
                    return 1;
                }else {
                    return 2;
                }
            }
        }else {
            return 1;
        }
    }
    if kinds_p2.contains(&3) && kinds_p2.contains(&2) {
        return 2;
    }

    if flush_p1 {
        if flush_p2 {
            return get_bigger(cards_p1, cards_p2);
        }else {
            return 1;
        }
    }
    if flush_p2 {
        return 2;
    }

    if straight_p1 {
        if straight_p2 {
            return get_bigger(cards_p1, cards_p2);
        }else {
            return 1;
        }
    }
    if straight_p2 {
        return 2;
    }

    if kinds_p1.contains(&3) {
        if kinds_p2.contains(&3) {
            let index_p1 = kinds_p1.iter().position(|&r| r == 3).unwrap();
            let index_p2 = kinds_p2.iter().position(|&r| r == 3).unwrap();
            if index_p1 > index_p2 {
                return 1;
            }else {
                return 2;
            }
        }else {
            return 1;
        }
    }
    if kinds_p2.contains(&3) {
        return 2;
    }

    let mut cnt_p1 = 0;
    let mut sum_p1 = 0;
    let mut cnt_p2 = 0;
    let mut sum_p2 = 0;
    for i in 0..15 {
        if kinds_p1[i] == 2 {
            cnt_p1 += 1;
            sum_p1 += i;
        }
        if kinds_p2[i] == 2 {
            cnt_p2 += 1;
            sum_p2 += i;
        }
    }
    // let cnt_p1 = kinds_p1.iter().map(|k| if *k == 2 {1}else {0}).sum::<u64>();
    // let cnt_p2 = kinds_p2.iter().map(|k| if *k == 2 {1}else {0}).sum::<u64>();

    if cnt_p1 == 2 {
        if cnt_p2 == 2 {
            if sum_p1 > sum_p2 {
                return 1;
            }else {
                return 2;
            }
        }else {
            return 1;
        }
    }
    if cnt_p2 == 2 {
        return 2;
    }

    if cnt_p1 == 1 {
        if cnt_p2 == 1 {
            let index_p1 = kinds_p1.iter().position(|&r| r == 2).unwrap();
            let index_p2 = kinds_p2.iter().position(|&r| r == 2).unwrap();
            if index_p1 > index_p2 {
                return 1;
            }else if index_p1 < index_p2 {
                return 2;
            }else {
                return get_bigger(cards_p1, cards_p2);
            }
        }else {
            return 1;
        }
    }
    if cnt_p2 == 1 {
        return 2;
    }

    return get_bigger(cards_p1, cards_p2);
}

pub fn main() {

    let mut player_one_wins: u64 = 0;
    let contents: String = fs::read_to_string("resources/054_poker.txt")
        .expect("Should have been able to read the file");
    for row in contents.split("\n") {

        let mut cards: Vec<Card> = Vec::new();
        for string in row.split(" ") {
            if string.len() > 0 {
                cards.push(str_to_card(string));
            }else {
                break;
            }
        }
        if cards.len() == 10 {
            let mut p1: Vec<Card> = cards[0..5].to_vec();
            p1.sort();
            let mut p2: Vec<Card> = cards[5..10].to_vec();
            p2.sort();

            if compare(p1, p2) == 1 {
                player_one_wins += 1;
            }
        }
    }

    println!("Player 1 wins: {}", player_one_wins);
}
