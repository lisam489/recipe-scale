// Formats a scaled quantity back into text a cook would actually write.
// Plain decimals for anything without a common kitchen fraction, otherwise
// a mixed number ("2 1/2") since that is how measuring cups are marked.
pub fn format_quantity(value: f64) -> String {
    const EPSILON: f64 = 0.01;
    let whole = value.trunc();
    let fraction = value - whole;

    let known_fractions: [(f64, &str); 7] = [
        (1.0 / 8.0, "1/8"),
        (1.0 / 4.0, "1/4"),
        (1.0 / 3.0, "1/3"),
        (1.0 / 2.0, "1/2"),
        (2.0 / 3.0, "2/3"),
        (3.0 / 4.0, "3/4"),
        (7.0 / 8.0, "7/8"),
    ];

    for (frac_value, label) in known_fractions {
        if (fraction - frac_value).abs() < EPSILON {
            return if whole.abs() < EPSILON {
                label.to_string()
            } else {
                format!("{} {}", whole as i64, label)
            };
        }
    }

    if fraction.abs() < EPSILON {
        return format!("{}", whole as i64);
    }

    let rounded = (value * 100.0).round() / 100.0;
    let text = format!("{:.2}", rounded);
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}
