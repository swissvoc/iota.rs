use std::iter::FromIterator;
use std::str::FromStr;

use trinary::*;
use constants::*;
use util::*;

#[derive(Debug, Eq, PartialEq)]
pub enum TrinaryParseError {
    InvalidCharacter,
    EmptyString,
}

impl FromIterator<char> for Trinary {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut trits: Vec<Trit> = Vec::new();
        let mut bytes: Vec<i8> = Vec::new();
        let mut n: usize = 0;

        for c in iter {
            n += 1;
            trits.extend_from_slice(&tryte_to_trits(c));

            if trits.len() >= TRITS_PER_BYTE {
                bytes.push(trits_to_byte(&trits[0..TRITS_PER_BYTE]));
                trits = trits.split_off(TRITS_PER_BYTE);
            }
        }

        bytes.push(trits_to_byte(trits.as_slice()));
        Trinary::new(bytes, n * TRITS_PER_TRYTE)
    }
}

impl FromStr for Trinary {
    type Err = TrinaryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TrinaryParseError::*;
        if s.is_empty() {
            return Err(EmptyString);
        }

        if s.chars()
            .filter(|&c| TRYTE_ALPHABET.find(c).is_none())
            .count() > 0
        {
            return Err(InvalidCharacter);
        }

        // String has valid length and alphabet
        Ok(s.chars().collect())
    }
}

#[cfg(test)]
mod tests {
    use string::*;
    use TrinaryParseError::*;

    #[test]
    fn only_valid_chars() {
        assert_eq!(Trinary::from_str("a"), Err(InvalidCharacter))
    }

    #[test]
    fn no_empty_string() {
        assert_eq!(Trinary::from_str(""), Err(EmptyString))
    }

    #[test]
    fn fromstr_test1() {
        let in_bytes: [i8; 6] = [20, 25, -14, -2, 83, 1];
        let trytes = "UYSSM9KIH";

        println!("{:?}", Trinary::from_str(trytes).ok().map(|a| a.trits()));

        let opt = Trinary::from_str(trytes).ok().map(
            |a| a.bytes() == in_bytes,
        );
        assert!(opt.is_some() && opt.unwrap());
    }

    #[test]
    fn fromtostr_test1() {
        let trytes = "UYSSM9KIH";
        let back = Trinary::from_str(trytes).ok().map(|a| a.to_string());
        assert!(back.is_some() && back.unwrap() == trytes);
    }

    #[test]
    fn fromtostr_test2() {
        let trytes = "LCUNQ99HCQM9HSATSQOPOWXXKGKDVZSEKVWJZRGWFRVLEQ9XG9INIPKAM9BQVGCIRNPZOS9LUZRBHB9P";
        let back = Trinary::from_str(trytes).ok().map(|a| a.to_string());
        assert!(back.is_some() && back.unwrap() == trytes);
        
    }
}
