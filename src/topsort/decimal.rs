use std::cmp::Ordering;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug,Clone,Copy,Hash,PartialEq)]
pub struct Decimal {
    whole: isize,
    fractional: isize

}

impl Ord for Decimal {
	fn cmp(&self, &other: &Self) -> Ordering {
		let whole_comparison = self.whole.cmp(&other.whole);
		if whole_comparison != Ordering::Equal {
			return whole_comparison;
		}
		self.fractional.cmp(&other.fractional)
	}
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Decimal {}

impl FromStr for Decimal {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
    	let mut parts = s.split('.');

        let whole = parts.next().map_or(0,|x| x.parse::<isize>().unwrap());
        let fractional = parts.next().map_or(0,|x| x.parse::<isize>().unwrap());

        Ok(Decimal { whole, fractional })
    }
}
