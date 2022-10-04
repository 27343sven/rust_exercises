#[allow(unused)]
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard(CardValue, CardValue, CardValue, CardValue, CardValue),
    OnePair(CardValue, CardValue, CardValue, CardValue),
    TwoPair(CardValue, CardValue, CardValue),
    ThreeOfAKind(CardValue, CardValue, CardValue),
    Straight(CardValue),
    Flush(Suit),
    FullHouse(CardValue, CardValue),
    FourOfAKind(CardValue, CardValue),
    StraightFlush(CardValue, Suit),
}

macro_rules! chain_cmp {
    ($($val1:expr=>$val2:expr,)*) => {
        {
            Ordering::Equal$(
                .then($val1.cmp(&$val2))
            )*
        }
    };
    ($($val1:expr=>$val2:expr),*) => {
        chain_cmp!($($val1=>$val2,)*)
    };
}

#[derive(Eq, Clone, Copy, Hash)]
struct Card {
    value: CardValue,
    suit: Suit,
}

impl Card {
    fn new(card: &str) -> Option<Self> {
        let (s_value, s_suit) = card.split_at(card.len() - 1);
        let value = Card::parse_card(s_value)?;
        let suit = Card::parse_suit(s_suit)?;
        Some(Card { value, suit })
    }

    fn default() -> Self {
        Card {
            value: CardValue::Ace,
            suit: Suit::Spade,
        }
    }

    fn parse_suit(c: &str) -> Option<Suit> {
        match c {
            "S" => Some(Suit::Spade),
            "H" => Some(Suit::Heart),
            "D" => Some(Suit::Diamond),
            "C" => Some(Suit::Club),
            _ => None,
        }
    }

    fn parse_card(c: &str) -> Option<CardValue> {
        match c {
            "2" => Some(CardValue::Two),
            "3" => Some(CardValue::Three),
            "4" => Some(CardValue::Four),
            "5" => Some(CardValue::Five),
            "6" => Some(CardValue::Six),
            "7" => Some(CardValue::Seven),
            "8" => Some(CardValue::Eight),
            "9" => Some(CardValue::Nine),
            "10" => Some(CardValue::Ten),
            "J" => Some(CardValue::Jack),
            "Q" => Some(CardValue::Queen),
            "K" => Some(CardValue::King),
            "A" => Some(CardValue::Ace),
            _ => None,
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.value > other.value {
            Some(Ordering::Greater)
        } else if self.value < other.value {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

#[derive(Clone)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl Hand {
    fn new(hand: &str) -> Option<Self> {
        let mut cards: [Card; 5] = [
            Card::default(),
            Card::default(),
            Card::default(),
            Card::default(),
            Card::default(),
        ];
        for (i, s_card) in hand.split(' ').enumerate().take(5) {
            cards[i] = Card::new(s_card)?;
        }
        cards.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));

        let hand_type = Hand::infer_hand(&cards);

        Some(Hand { cards, hand_type })
    }

    fn infer_hand(cards: &[Card; 5]) -> HandType {
        let (is_straight, is_ace_low) = Hand::is_straight(cards);
        let is_flush = Hand::is_flush(cards);
        let (is_2kind, is_3kind, is_4kind, is_twopair) = Hand::is_n_ofakind(cards);

        if is_straight && is_flush {
            HandType::StraightFlush(if is_ace_low {cards[1].value} else {cards[0].value}, if is_ace_low {cards[1].suit} else {cards[0].suit})
        } else if let Some(card_4kind) = is_4kind {
            let leftover = Hand::get_leftover(
                cards.into_iter().map(|c| c.value).collect(),
                vec![card_4kind],
            );
            HandType::FourOfAKind(card_4kind, *leftover.get(0).unwrap_or(&card_4kind))
        } else if let (Some(card_3kind), Some(card_2kind)) = (is_3kind, is_2kind) {
            HandType::FullHouse(card_3kind, card_2kind)
        } else if is_flush {
            HandType::Flush(cards[0].suit)
        } else if is_straight {
            HandType::Straight(if is_ace_low {cards[1].value} else {cards[0].value})
        } else if let Some(card_3kind) = is_3kind {
            let leftover = Hand::get_leftover(
                cards.into_iter().map(|c| c.value).collect(),
                vec![card_3kind],
            );
            HandType::ThreeOfAKind(card_3kind, leftover[0], leftover[1])
        } else if let Some((high_pair, low_pair)) = is_twopair {
            let leftover = Hand::get_leftover(
                cards.into_iter().map(|c| c.value).collect(),
                vec![high_pair, low_pair],
            );
            HandType::TwoPair(high_pair, low_pair, leftover[0])
        } else if let Some(card_2kind) = is_2kind {
            let leftover = Hand::get_leftover(
                cards.into_iter().map(|c| c.value).collect(),
                vec![card_2kind],
            );
            HandType::OnePair(card_2kind, leftover[0], leftover[1], leftover[2])
        } else {
            HandType::HighCard(cards[0].value, cards[1].value, cards[2].value, cards[3].value, cards[4].value)
        }
    }

    fn get_leftover(cards: Vec<CardValue>, remove: Vec<CardValue>) -> Vec<CardValue> {
        cards
            .into_iter()
            .filter(|card| {
                !remove.contains(card)
            })
            .collect()
    }

    fn is_flush(cards: &[Card; 5]) -> bool {
        let mut suits: HashMap<Suit, usize> = HashMap::new();
        for card in cards {
            *suits.entry(card.suit).or_insert(0) += 1;
        }
        suits.len() == 1
    }

    fn is_straight(cards: &[Card; 5]) -> (bool, bool) {
        let mut val = cards[0].value;
        let mut is_straight = true;
        let possible_acelow = cards[0].value == CardValue::Ace && cards[4].value == CardValue::Two;
        for (i, card) in cards.into_iter().enumerate() {
            if i == 0 {
                continue;
            };
            if (val as usize - 1) != card.value as usize && !(possible_acelow && i == 1) {
                is_straight = false;
                break;
            }
            val = card.value;
        }
        (is_straight, possible_acelow)
    }

    fn is_n_ofakind(
        cards: &[Card; 5],
    ) -> (
        Option<CardValue>,
        Option<CardValue>,
        Option<CardValue>,
        Option<(CardValue, CardValue)>,
    ) {
        let mut counts: HashMap<CardValue, usize> = HashMap::new();
        for card in cards {
            *counts.entry(card.value).or_insert(0) += 1;
        }

        let mut pairs: Vec<CardValue> = counts
            .clone()
            .into_iter()
            .filter(|(_, n)| *n == 2)
            .map(|(card, _)| card)
            .collect();
        pairs.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
        let (is_3kind, is_4kind) =
            counts
                .into_iter()
                .fold((None, None), |(is_3kind, is_4kind), (card, n)| {
                    if n == 3 {
                        return (Some(card), is_4kind);
                    } else if n == 4 {
                        return (is_3kind, Some(card));
                    }
                    return (is_3kind, is_4kind);
                });

        (
            pairs.get(0).and_then(|c| Some(c.clone())),
            is_3kind,
            is_4kind,
            if pairs.len() == 2 {
                Some((pairs[0], pairs[1]))
            } else {
                None
            }, 
        )
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }

}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type > other.hand_type {
            Some(Ordering::Greater)
        } else if self.hand_type < other.hand_type {
            Some(Ordering::Less)
        } else {
            match (self.hand_type, other.hand_type) {
                (
                    HandType::StraightFlush(s_high, s_suit),
                    HandType::StraightFlush(o_high, o_suit),
                ) => Some(chain_cmp!(
                    s_high => o_high,
                    s_suit => o_suit,
                )),
                (
                    HandType::FourOfAKind(s_4kind, s_high),
                    HandType::FourOfAKind(o_4kind, o_high),
                ) => {
                    Some(chain_cmp!(
                    s_4kind => o_4kind,
                    s_high => o_high,
                    ))
                }
                (HandType::FullHouse(s_3kind, s_2kind), HandType::FullHouse(o_3kind, o_2kind)) => {
                    Some(chain_cmp!(s_3kind => o_3kind, s_2kind => o_2kind))
                }
                (HandType::Flush(s_suit), HandType::Flush(o_suit)) => Some(chain_cmp!(
                    self.cards[0].value => other.cards[0].value,
                    self.cards[1].value => other.cards[1].value,
                    self.cards[2].value => other.cards[2].value,
                    self.cards[3].value => other.cards[3].value,
                    self.cards[4].value => other.cards[4].value,
                    s_suit => o_suit,
                )),
                (HandType::Straight(s_high), HandType::Straight(o_high)) => {
                    Some(chain_cmp!(s_high => o_high))
                }
                (
                    HandType::ThreeOfAKind(s_3kind, s_high, s_low),
                    HandType::ThreeOfAKind(o_3kind, o_high, o_low),
                ) => Some(chain_cmp!(
                    s_3kind => o_3kind,
                    s_high => o_high,
                    s_low => o_low,
                )),
                (
                    HandType::TwoPair(s_high_pair, s_low_pair, s_high),
                    HandType::TwoPair(o_high_pair, o_low_pair, o_high),
                ) => Some(chain_cmp!(
                    s_high_pair => o_high_pair,
                    s_low_pair => o_low_pair,
                    s_high => o_high,
                )),
                (
                    HandType::OnePair(s_pair, s_kick1, s_kick2, s_kick3),
                    HandType::OnePair(o_pair, o_kick1, o_kick2, o_kick3),
                ) => Some(chain_cmp!(
                    s_pair => o_pair,
                    s_kick1 => o_kick1,
                    s_kick2 => o_kick2,
                    s_kick3 => o_kick3,
                )),
                (
                    HandType::HighCard(s_high, s_kick1, s_kick2, s_kick3, s_kick4), 
                    HandType::HighCard(o_high, o_kick1, o_kick2, o_kick3, o_kick4),
                ) => {
                    Some(chain_cmp!(
                        s_high => o_high,
                        s_kick1 => o_kick1,
                        s_kick2 => o_kick2,
                        s_kick3 => o_kick3,
                        s_kick4 => o_kick4,
                    ))
                }
                _ => None,
            }
        }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut sorted_hands = hands
        .into_iter()
        .zip(hands.into_iter().map(|&hand| Hand::new(hand).unwrap()))
        .collect::<Vec<(&&'a str, Hand)>>();
    sorted_hands.sort_by(|(_, hand1), (_, hand2)| hand2.partial_cmp(hand1).unwrap());

    let highest = sorted_hands[0].1.clone();
    let mut result = vec![];
    for (reference, hand) in sorted_hands {
        if hand == highest {
            result.push(*reference)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn hs_from<'a>(input: &[&'a str]) -> HashSet<&'a str> {
        let mut hs = HashSet::new();
        for item in input.iter() {
            hs.insert(*item);
        }
        hs
    }

    /// Test that the expected output is produced from the given input
    /// using the `winning_hands` function.
    ///
    /// Note that the output can be in any order. Here, we use a HashSet to
    /// abstract away the order of outputs.

    fn test<'a, 'b>(input: &[&'a str], expected: &[&'b str]) {
        assert_eq!(hs_from(&winning_hands(input)), hs_from(expected))
    }

    #[test]
    fn test_single_hand_always_wins() {
        test(&["4S 5S 7H 8D JC"], &["4S 5S 7H 8D JC"])
    }

    #[test]
    fn test_duplicate_hands_always_tie() {
        let input = &["3S 4S 5D 6H JH", "3S 4S 5D 6H JH", "3S 4S 5D 6H JH"];
        assert_eq!(&winning_hands(input), input)
    }

    #[test]
    fn test_highest_card_of_all_hands_wins() {
        test(
            &["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"],
            &["3S 4S 5D 6H JH"],
        )
    }

    #[test]
    fn test_a_tie_has_multiple_winners() {
        test(
            &[
                "4D 5S 6S 8D 3C",
                "2S 4C 7S 9H 10H",
                "3S 4S 5D 6H JH",
                "3H 4H 5C 6C JD",
            ],
            &["3S 4S 5D 6H JH", "3H 4H 5C 6C JD"],
        )
    }

    #[test]
    fn test_high_card_can_be_low_card_in_an_otherwise_tie() {
        // multiple hands with the same high cards, tie compares next highest ranked,
        // down to last card

        test(&["3S 5H 6S 8D 7H", "2S 5D 6D 8C 7S"], &["3S 5H 6S 8D 7H"])
    }

    #[test]
    fn test_one_pair_beats_high_card() {
        test(&["4S 5H 6C 8D KH", "2S 4H 6S 4D JH"], &["2S 4H 6S 4D JH"])
    }

    #[test]
    fn test_highest_pair_wins() {
        test(&["4S 2H 6S 2D JH", "2S 4H 6C 4D JD"], &["2S 4H 6C 4D JD"])
    }

    #[test]
    fn test_two_pairs_beats_one_pair() {
        test(&["2S 8H 6S 8D JH", "4S 5H 4C 8C 5C"], &["4S 5H 4C 8C 5C"])
    }

    #[test]
    fn test_two_pair_ranks() {
        // both hands have two pairs, highest ranked pair wins

        test(&["2S 8H 2D 8D 3H", "4S 5H 4C 8S 5D"], &["2S 8H 2D 8D 3H"])
    }

    #[test]
    fn test_two_pairs_second_pair_cascade() {
        // both hands have two pairs, with the same highest ranked pair,
        // tie goes to low pair

        test(&["2S QS 2C QD JH", "JD QH JS 8D QC"], &["JD QH JS 8D QC"])
    }

    #[test]
    fn test_two_pairs_last_card_cascade() {
        // both hands have two identically ranked pairs,
        // tie goes to remaining card (kicker)

        test(&["JD QH JS 8D QC", "JS QS JC 2D QD"], &["JD QH JS 8D QC"])
    }

    #[test]
    fn test_three_of_a_kind_beats_two_pair() {
        test(&["2S 8H 2H 8D JH", "4S 5H 4C 8S 4H"], &["4S 5H 4C 8S 4H"])
    }

    #[test]
    fn test_three_of_a_kind_ranks() {
        //both hands have three of a kind, tie goes to highest ranked triplet

        test(&["2S 2H 2C 8D JH", "4S AH AS 8C AD"], &["4S AH AS 8C AD"])
    }

    #[test]
    fn test_three_of_a_kind_cascade_ranks() {
        // with multiple decks, two players can have same three of a kind,

        // ties go to highest remaining cards

        test(&["4S AH AS 7C AD", "4S AH AS 8C AD"], &["4S AH AS 8C AD"])
    }

    #[test]
    fn test_straight_beats_three_of_a_kind() {
        test(&["4S 5H 4C 8D 4H", "3S 4D 2S 6D 5C"], &["3S 4D 2S 6D 5C"])
    }

    #[test]
    fn test_aces_can_end_a_straight_high() {
        // aces can end a straight (10 J Q K A)

        test(&["4S 5H 4C 8D 4H", "10D JH QS KD AC"], &["10D JH QS KD AC"])
    }

    #[test]
    fn test_aces_can_end_a_straight_low() {
        // aces can start a straight (A 2 3 4 5)

        test(&["4S 5H 4C 8D 4H", "4D AH 3S 2D 5C"], &["4D AH 3S 2D 5C"])
    }

    #[test]
    fn test_straight_cascade() {
        // both hands with a straight, tie goes to highest ranked card

        test(&["4S 6C 7S 8D 5H", "5S 7H 8S 9D 6H"], &["5S 7H 8S 9D 6H"])
    }

    #[test]
    fn test_straight_scoring() {
        // even though an ace is usually high, a 5-high straight is the lowest-scoring straight

        test(&["2H 3C 4D 5D 6H", "4S AH 3S 2D 5H"], &["2H 3C 4D 5D 6H"])
    }

    #[test]
    fn test_flush_beats_a_straight() {
        test(&["4C 6H 7D 8D 5H", "2S 4S 5S 6S 7S"], &["2S 4S 5S 6S 7S"])
    }

    #[test]
    fn test_flush_cascade() {
        // both hands have a flush, tie goes to high card, down to the last one if necessary

        test(&["4H 7H 8H 9H 6H", "2S 4S 5S 6S 7S"], &["4H 7H 8H 9H 6H"])
    }

    #[test]
    fn test_full_house_beats_a_flush() {
        test(&["3H 6H 7H 8H 5H", "4S 5C 4C 5D 4H"], &["4S 5C 4C 5D 4H"])
    }

    #[test]
    fn test_full_house_ranks() {
        // both hands have a full house, tie goes to highest-ranked triplet

        test(&["4H 4S 4D 9S 9D", "5H 5S 5D 8S 8D"], &["5H 5S 5D 8S 8D"])
    }

    #[test]
    fn test_full_house_cascade() {
        // with multiple decks, both hands have a full house with the same triplet, tie goes to the pair

        test(&["5H 5S 5D 9S 9D", "5H 5S 5D 8S 8D"], &["5H 5S 5D 9S 9D"])
    }

    #[test]
    fn test_four_of_a_kind_beats_full_house() {
        test(&["4S 5H 4D 5D 4H", "3S 3H 2S 3D 3C"], &["3S 3H 2S 3D 3C"])
    }

    #[test]
    fn test_four_of_a_kind_ranks() {
        // both hands have four of a kind, tie goes to high quad

        test(&["2S 2H 2C 8D 2D", "4S 5H 5S 5D 5C"], &["4S 5H 5S 5D 5C"])
    }

    #[test]
    fn test_four_of_a_kind_cascade() {
        // with multiple decks, both hands with identical four of a kind, tie determined by kicker

        test(&["3S 3H 2S 3D 3C", "3S 3H 4S 3D 3C"], &["3S 3H 4S 3D 3C"])
    }

    #[test]
    fn test_straight_flush_beats_four_of_a_kind() {
        test(&["4S 5H 5S 5D 5C", "7S 8S 9S 6S 10S"], &["7S 8S 9S 6S 10S"])
    }

    #[test]
    fn test_straight_flush_ranks() {
        // both hands have straight flush, tie goes to highest-ranked card

        test(&["4H 6H 7H 8H 5H", "5S 7S 8S 9S 6S"], &["5S 7S 8S 9S 6S"])
    }
}
