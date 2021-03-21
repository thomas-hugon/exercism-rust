use crate::poker::Hand;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let scored_hands: Vec<(u64, Hand)> = hands
        .iter()
        .map(|hand_str| Hand::from(hand_str))
        .map(|h| (h.rank(), h))
        .collect();

    let max_score = scored_hands.iter().map(|(r, _)| r).max();

    max_score.map(|max_score| {
        scored_hands
            .iter()
            .filter(|(r, _)| r == max_score)
            .map(|(_, h)| h.cards_str)
            .collect()
    })
}

mod poker {

    #[derive(Clone)]
    pub struct Card {
        value: u8,
        color: char,
    }

    pub struct Hand<'a> {
        pub cards_str: &'a str,
        cards: Vec<Card>,
    }

    impl Card {
        pub fn from(card_str: &str) -> Card {
            let (value, color) = card_str.split_at(card_str.len() - 1);
            let value = value
                .chars()
                .map(|c| match c {
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => c.to_digit(10).expect("not a valid card value") as u8,
                })
                .fold(0, |nb, d| nb * 10 + d);

            let color = color.chars().next().unwrap();

            Card { color, value }
        }
    }

    impl<'a> Hand<'a> {
        pub fn from(cards_str: &str) -> Hand {
            let mut cards: Vec<Card> = cards_str
                .split_ascii_whitespace()
                .map(|s| Card::from(s))
                .collect();
            cards.sort_by_key(|c| c.value);

            Hand { cards_str, cards }
        }

        fn flush(&self) -> Option<&[Card]> {
            //all cards must have the same color
            self.cards
                .windows(5)
                .find(|w| w.iter().all(|c| c.color == w[0].color))
        }

        fn straight(&self) -> Option<&[Card]> {
            //all cards have sequential value
            if self.cards.windows(2).all(|w| w[0].value + 1 == w[1].value) {
                Some(&self.cards)
            } else {
                None
            }
        }

        fn low_straight(&self) -> Option<&[Card]> {
            //a hand composed as follow: A,2,3,4,5 -> A is 14, so total is 28
            if self.cards.iter().any(|c| c.value == 14)
                && self.cards.iter().map(|c| c.value).sum::<u8>() == 28
            {
                Some(&self.cards)
            } else {
                None
            }
        }

        fn n_of_a_kind(cards: &[Card], n: usize) -> (Vec<&[Card]>, Vec<Card>) {
            // windows of n cards all having the same value is a n of a kind
            // (n-x) of a kind may be searched in remaining cards
            let matching: Vec<&[Card]> = cards
                .windows(n)
                .filter(|w| w.iter().all(|c| c.value == w[0].value))
                .collect();
            if matching.is_empty() {
                (matching, cards.to_vec())
            } else {
                let remaining = cards
                    .iter()
                    .filter(|x| {
                        !matching
                            .iter()
                            .any(|cards| cards.iter().any(|c| c.value == x.value))
                    })
                    .cloned()
                    .collect();
                (matching, remaining)
            }
        }

        fn score_cards(cards: &[Card]) -> u64 {
            //when all cards must be scored ( in case of flush without straight or high card)
            //in that case, card 1 score 1, card 2 ->2, card 3 -> 4, card 4 -> 8, card 5 -> 16....
            //and cards score is the sum of each individual score
            cards
                .iter()
                .fold(0, |acc: u64, card| acc + 2u64.pow(card.value as u32 - 1))
        }

        pub fn rank(&self) -> u64 {
            let flush = self.flush();
            let straight = self.straight();
            let low_straight = self.low_straight();
            let (n_4_oak, remaining) = Self::n_of_a_kind(&self.cards, 4);
            let (n_3_oak, remaining) = Self::n_of_a_kind(&remaining, 3);
            let (pairs, cards) = Self::n_of_a_kind(&remaining, 2);
            let straight_flush = flush.and(straight);
            let full = if !n_3_oak.is_empty() && !pairs.is_empty() {
                Some((n_3_oak[0], pairs[0]))
            } else {
                None
            };

            let score_straight = low_straight
                .map(|x| x[3].value)
                .or_else(|| straight.map(|x| x[4].value));
            let score_straight_flush = straight_flush.and(score_straight).unwrap_or(0) as u64;
            let score_4oak = n_4_oak.get(0).map_or(0, |cards| cards[0].value) as u64;
            let score_full = full.map_or(0, |_| 1) as u64;
            let score_flush = flush.map_or(0, |x| Self::score_cards(x)) as u64;
            let score_straight = score_straight.unwrap_or(0) as u64;
            let score_3oak = n_3_oak.get(0).map_or(0, |cards| cards[0].value) as u64;
            let score_2pairs = pairs.get(1).map_or(0, |p| p[0].value) as u64;
            let score_pair = pairs.get(0).map_or(0, |p| p[0].value) as u64;
            let score_cards = Self::score_cards(&cards);

            score_straight_flush * 14u64.pow(15)
                + score_4oak * 14u64.pow(14)
                // 14^13 > 14^8 * 2^14
                + score_full * 14u64.pow(13)
                + score_flush * 14u64.pow(8)
                + score_straight * 14u64.pow(7)
                + score_3oak * 14u64.pow(6)
                + score_2pairs * 14u64.pow(5)
                // 14^4 > 2^14
                + score_pair * 14u64.pow(4)
                + score_cards
        }
    }
}
