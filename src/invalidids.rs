use std::{ io::BufRead};

#[derive(Debug, PartialEq)]
pub struct IdRange {
    min: u64,
    max: u64,
}
// invalid ids are ids made up of sequences of two identical sequences.
// i.e. 123123 == invalid, 55 is invalid.
// 
// odd number of digits can never be invalid

pub fn iter_ranges<R: std::io::Read>(rdr: R) -> impl Iterator<Item=Result<IdRange, IdRangeError>> {
    std::io::BufReader::new(rdr)
        .split(b',')
        .filter(|b| match b {
            Err(_) => true,
            Ok(v) => !v.is_empty()
        })
        .map(|r| match r {
            Ok(v) => IdRange::try_from(v.as_slice()),
            Err(x) => Err(IdRangeError::from(x))
        })
}

pub fn naive_invalid_id(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();
    if !len.is_multiple_of(2) {
        return false;
    }
    let half = len / 2;
    let (first, second) = s.split_at(half);
    first == second
}

pub fn naive_invalid_ids(range: &IdRange) -> Vec<u64> {
    let mut invalids = Vec::new();
    for id in range.min..=range.max {
        if naive_invalid_id(id) {
            invalids.push(id);
        }
    }
    invalids
}

#[derive(Debug)]
pub enum IdRangeError {
    ParseError(std::num::ParseIntError),
    UTF8Error(std::str::Utf8Error),
    IOError(std::io::Error),
    RangeError,
}

impl PartialEq for IdRangeError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (IdRangeError::ParseError(a), IdRangeError::ParseError(b)) => a == b,
            (IdRangeError::UTF8Error(a), IdRangeError::UTF8Error(b)) => a == b,
            (IdRangeError::IOError(_), IdRangeError::IOError(_)) => false,
            (IdRangeError::RangeError, IdRangeError::RangeError) => true,
            _ => false,
        }
    }
}

impl From<std::io::Error> for IdRangeError {
    fn from(err: std::io::Error) -> Self {
        IdRangeError::IOError(err)
    }
}

impl From<std::num::ParseIntError> for IdRangeError {
    fn from(err: std::num::ParseIntError) -> Self {
        IdRangeError::ParseError(err)
    }
}

impl From<std::str::Utf8Error> for IdRangeError {
    fn from(err: std::str::Utf8Error) -> Self {
        IdRangeError::UTF8Error(err)
    }
}
impl TryFrom<&str> for IdRange {
    type Error = IdRangeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // do we care about leading zeroes for parsing ?
        let parts: Vec<&str> = s.split('-').collect();
        let min = parts[0].parse::<u64>()?;
        let max = parts[1].parse::<u64>()?;
        if min >= max {
            return Err(IdRangeError::RangeError);
        }
        Ok(IdRange { min, max })
    }
}

impl TryFrom<&[u8]> for IdRange {
    type Error = IdRangeError;

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        let s_str = std::str::from_utf8(s)?;
        IdRange::try_from(s_str)
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufRead;
    use std::io::Cursor;
    use super::*;
    static GIVEN_TESTCASE: &str = "\
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124";
    #[test]
    fn test_naive_invalid_id() {
        assert!(naive_invalid_id(1212));
        assert!(naive_invalid_id(123123));
        assert!(naive_invalid_id(12341234));
        assert!(!naive_invalid_id(12345));
        assert!(!naive_invalid_id(1231231));
    }

    #[test]
    fn given_testcase() {
        let rdr = Cursor::new(GIVEN_TESTCASE);
        let items: Vec<IdRange> = iter_ranges(rdr).collect::<Result<Vec<IdRange>, IdRangeError>>().unwrap();
        let mut res = 0u64;

        for r in &items {
            let invalids = naive_invalid_ids(r);
            res += invalids.iter().sum::<u64>();
        }
        assert_eq!(res, 1227775554);
    }

    #[test]
    fn test_id_range_try_from() {
        assert_eq!(IdRange::try_from("100-200"), Ok(IdRange { min: 100, max: 200 }));
        assert_eq!(IdRange::try_from("200-100"), Err(IdRangeError::RangeError));
        assert_eq!(IdRange::try_from("100-100"), Err(IdRangeError::RangeError));
        assert!(matches!(IdRange::try_from("abc-def"), Err(IdRangeError::ParseError(_))));
        assert!(matches!(IdRange::try_from("10-def"), Err(IdRangeError::ParseError(_))));
    }

    #[test]
    fn test_id_ranges_from_reader() {
        let rdr = Cursor::new(
            "11-22,95-115,998-1012,1188511880-1188511890,\
            222220-222224,1698522-1698528,446443-446449,38593856-38593862,\
            565653-565659,824824821-824824827,2121212118-2121212124");   
        let items: Vec<IdRange> = iter_ranges(rdr).collect::<Result<Vec<IdRange>, IdRangeError>>().unwrap();
        assert_eq!(items, vec![
            IdRange { min: 11, max: 22 },
            IdRange { min: 95, max: 115 },
            IdRange { min: 998, max: 1012 },
            IdRange { min: 1188511880, max: 1188511890 },
            IdRange { min: 222220, max: 222224 },
            IdRange { min: 1698522, max: 1698528 },
            IdRange { min: 446443, max: 446449 },
            IdRange { min: 38593856, max: 38593862 },
            IdRange { min: 565653, max: 565659 },
            IdRange { min: 824824821, max: 824824827 },
            IdRange { min: 2121212118, max: 2121212124 },
        ]);
    }

    #[test]
    fn test_naive_invalid_ids() {
        let rdr = Cursor::new("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124");

        let mut brc = std::io::BufReader::new(rdr).split(b',').map(|r| match r {
            Ok(v) => IdRange::try_from(v.as_slice()),
            Err(x) => Err(IdRangeError::from(x))
        });
        let i1 = brc.next().unwrap().unwrap();
        assert_eq!(i1, IdRange { min: 11, max: 22 });

    }
}