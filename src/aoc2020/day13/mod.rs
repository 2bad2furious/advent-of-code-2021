use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use std::time::Instant;

use aoc2020::read_asset;

pub fn solve_part2() {
    let input: Vec<Option<u32>> = read_asset(13).split_whitespace()
        .skip(1)
        .flat_map(|x| x.split(","))
        .map(|x| { FromStr::from_str(x).ok() })
        .collect();
    let start = Instant::now();
    let result = part2(input.as_ref());
    println!("{} in {}us", result.unwrap_or(0), start.elapsed().as_micros());
}

pub fn part2(ids: &[Option<u32>]) -> Option<u64> {
    let mut offset: i64 = 0;
    let mut iterator = ids.into_iter();
    let mut last_meet: u64 = iterator.next()?.unwrap().into();
    let mut lcm: u64 = last_meet;

    while let Some(id) = iterator.next() {
        offset = offset + 1;
        if id.is_none() { continue; }
        let c: i64 = -offset - i64::try_from(last_meet).unwrap();
        let expression = Expression::from(lcm.try_into().unwrap(), id.unwrap().into(), c);

        let x = expression.smallest_positive_x();

        last_meet = (x * lcm) + last_meet;
        lcm = u64::try_from(expression.b_div_gcd()).unwrap() * lcm
    }

    return Some(last_meet);
}

struct Expression {
    a: i64,
    x0: i64,
    b: i64,
    //y0: i64,
    c: i64,
    gcd: u64,
}

impl Expression {
    fn b_div_gcd(&self) -> i64 {
        return self.b / i64::try_from(self.gcd).unwrap();
    }

    fn smallest_positive_x(&self) -> u64 {
        let b_dividied_by_gcd = self.b_div_gcd();
        let t: f64 = (self.x0 as f64) / (b_dividied_by_gcd as f64);
        return (self.x0 - (t.floor() as i64) * b_dividied_by_gcd) as u64;
    }

    fn from(a: i64, b: i64, c: i64) -> Expression {
        let bezout = gcd_bezout(a, b);
        let gcd = bezout.gcd as i64;
        assert_eq!(c % gcd, 0);

        let ratio = c / gcd;
        return Expression { a, x0: bezout.a * ratio, b, c, gcd: bezout.gcd };
    }
}

pub struct Bezout {
    gcd: u64,
    a: i64,
    b: i64,
}

// @link(https://introcs.cs.princeton.edu/java/99crypto/ExtendedEuclid.java.html)
pub fn gcd_bezout(p: i64, q: i64) -> Bezout {
    if q == 0 { return Bezout { gcd: p.unsigned_abs(), a: 1, b: 0 }; }
    let Bezout { gcd: d, a: b0, b: c, .. } = gcd_bezout(q, p % q);
    let b = b0 - (p / q) * c;
    return Bezout { gcd: d, a: c, b };
}
