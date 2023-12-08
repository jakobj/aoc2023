use std::{cmp::Ordering, collections::HashMap, fs, str::FromStr};

fn main() {
    let filename = "inputs/7a.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let mut hands_bids = content
        .lines()
        .map(|l| {
            let l_split = l.split(' ').collect::<Vec<&str>>();
            let hand = l_split[0]
                .parse::<Hand>()
                .unwrap_or_else(|_| panic!("Could not convert line \"{l}\" to `Game`"));
            let bid = l_split[1]
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("Could not convert line \"{l}\" to `Game`"));
            (hand, bid)
        })
        .collect::<Vec<(Hand, usize)>>();
    // TODO avoid clone here?!
    hands_bids.sort_unstable_by_key(|k| k.0.clone());
    let winnings = hands_bids
        .iter()
        .enumerate()
        .map(|(i, (_, w))| (i + 1) * w)
        .sum::<usize>();
    println!("The total winnings are {winnings}.");
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
}

impl FromStr for Hand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<Card>, ParseError>>()?;
        Ok(Hand { cards })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_self = HandType::from(&self.cards);
        let hand_type_other = HandType::from(&other.cards);
        let cmp = hand_type_self.cmp(&hand_type_other);
        if cmp != Ordering::Equal {
            return cmp;
        } else {
            for (cs, co) in self.cards.iter().zip(other.cards.iter()) {
                if cs == co {
                    continue;
                } else {
                    return cs.cmp(co);
                }
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Card {
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

impl TryFrom<char> for Card {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err(ParseError),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&Vec<Card>> for HandType {
    fn from(value: &Vec<Card>) -> Self {
        let mut counter: HashMap<Card, usize> = HashMap::new();
        for c in value.iter() {
            *counter.entry(*c).or_default() += 1;
        }
        let mut counter = counter.values().copied().collect::<Vec<usize>>();
        counter.sort_unstable();
        if counter == vec![5] {
            HandType::FiveOfAKind
        } else if counter == vec![1, 4] {
            HandType::FourOfAKind
        } else if counter == vec![2, 3] {
            HandType::FullHouse
        } else if counter == vec![1, 1, 3] {
            HandType::ThreeOfAKind
        } else if counter == vec![1, 2, 2] {
            HandType::TwoPair
        } else if counter == vec![1, 1, 1, 2] {
            HandType::OnePair
        } else {
            assert!(counter == vec![1, 1, 1, 1, 1]);
            HandType::HighCard
        }
    }
}
