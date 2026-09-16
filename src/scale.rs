use crate::fraction::Fraction;

// Formats a scaled quantity back into text a cook would actually write.
// Plain decimals for anything without a common kitchen fraction, otherwise
// a mixed number ("2 1/2") since that is how measuring cups are marked.
// The fraction arithmetic upstream is exact, so the only rounding here is
// the cosmetic call between "close enough to 1/3" and "print 0.33".
pub fn format_quantity(value: Fraction) -> String {
    let whole = value.num / value.den;
    let remainder = Fraction::new(value.num - whole * value.den, value.den);

    const KNOWN_FRACTIONS: [(i64, i64, &str); 7] = [
        (1, 8, "1/8"),
        (1, 4, "1/4"),
        (1, 3, "1/3"),
        (1, 2, "1/2"),
        (2, 3, "2/3"),
        (3, 4, "3/4"),
        (7, 8, "7/8"),
    ];

    for (num, den, label) in KNOWN_FRACTIONS {
        if remainder.num == num && remainder.den == den {
            return if whole == 0 {
                label.to_string()
            } else {
                format!("{} {}", whole, label)
            };
        }
    }

    if remainder.is_zero() {
        return whole.to_string();
    }

    let hundredths = (value.num * 100 + value.den / 2) / value.den;
    let text = format!("{}.{:02}", hundredths / 100, hundredths % 100);
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}
