// Ingredient quantities are scaled by multiplying rational numbers, and
// f64 rounding compounds across a whole recipe: 1/3 cup scaled by 2/3
// should come back out to 2/9, not 0.2222...4 truncated somewhere. Keeping
// everything as an exact num/den pair means the only rounding that ever
// happens is the final cosmetic step of picking how to print the result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction {
    pub num: i64,
    pub den: i64,
}

impl Fraction {
    pub fn new(num: i64, den: i64) -> Self {
        assert!(den != 0, "fraction denominator cannot be zero");
        let sign = if den < 0 { -1 } else { 1 };
        let num = num * sign;
        let den = den * sign;
        let divisor = gcd(num.unsigned_abs(), den.unsigned_abs()).max(1) as i64;
        Fraction {
            num: num / divisor,
            den: den / divisor,
        }
    }

    pub fn whole(n: i64) -> Self {
        Fraction::new(n, 1)
    }

    pub fn add(self, other: Fraction) -> Fraction {
        Fraction::new(self.num * other.den + other.num * self.den, self.den * other.den)
    }

    pub fn mul(self, other: Fraction) -> Fraction {
        Fraction::new(self.num * other.num, self.den * other.den)
    }

    pub fn div(self, other: Fraction) -> Fraction {
        Fraction::new(self.num * other.den, self.den * other.num)
    }

    pub fn is_positive(self) -> bool {
        self.num > 0
    }

    pub fn is_zero(self) -> bool {
        self.num == 0
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Parses a plain decimal string ("12", "1.5", "-0.25") into an exact
// fraction. Rejects anything with more than one decimal point or stray
// characters rather than falling back to f64::parse, since that is what
// lets a value like "1.1" stay exactly 11/10 instead of the nearest f64.
pub fn parse_decimal(text: &str) -> Option<Fraction> {
    let (negative, text) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit() || b == b'.') {
        return None;
    }

    let magnitude = match text.split_once('.') {
        None => Fraction::whole(text.parse::<i64>().ok()?),
        Some((int_part, frac_part)) => {
            if frac_part.is_empty() || frac_part.contains('.') {
                return None;
            }
            let int_val: i64 = if int_part.is_empty() { 0 } else { int_part.parse().ok()? };
            let frac_val: i64 = frac_part.parse().ok()?;
            let den = 10i64.checked_pow(frac_part.len() as u32)?;
            Fraction::new(int_val.checked_mul(den)?.checked_add(frac_val)?, den)
        }
    };

    Some(if negative {
        Fraction::new(-magnitude.num, magnitude.den)
    } else {
        magnitude
    })
}
