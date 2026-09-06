use std::fmt;

#[derive(Debug)]
pub struct Recipe {
    pub title: Option<String>,
    pub servings: f64,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Debug)]
pub struct Ingredient {
    pub quantity: f64,
    pub description: String,
}

// Modeled after compiler diagnostics: a message plus a source snippet with
// a caret under the exact character that caused the problem. Recipe files
// are short and hand-edited, so pointing at the wrong quantity by even one
// column is the difference between a fix taking five seconds or five minutes.
pub struct ParseError {
    pub file: String,
    pub line: usize,
    pub col: usize,
    pub message: String,
    pub source_line: String,
    pub span: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.line.to_string().len();
        writeln!(f, "error: {}", self.message)?;
        writeln!(f, "{}--> {}:{}:{}", " ".repeat(width + 1), self.file, self.line, self.col)?;
        writeln!(f, "{} |", " ".repeat(width))?;
        writeln!(f, "{:>width$} | {}", self.line, self.source_line, width = width)?;
        let padding = " ".repeat(self.col.saturating_sub(1));
        let carets = "^".repeat(self.span.max(1));
        write!(f, "{} | {}{}", " ".repeat(width), padding, carets)
    }
}

pub fn parse(source: &str, file: &str) -> Result<Recipe, ParseError> {
    let mut title: Option<String> = None;
    let mut servings: Option<f64> = None;
    let mut ingredients = Vec::new();
    let mut last_line = 0usize;

    for (idx, raw_line) in source.lines().enumerate() {
        let line_no = idx + 1;
        last_line = line_no;
        let content = raw_line.trim();

        if content.is_empty() || content.starts_with('#') {
            continue;
        }

        if servings.is_none() {
            let start_col = first_non_ws_col(raw_line);

            if let Some(rest) = content.strip_prefix("title:") {
                title = Some(rest.trim().to_string());
                continue;
            }

            if let Some(rest) = content.strip_prefix("servings:") {
                let leading_ws = rest.chars().take_while(|c| c.is_whitespace()).count();
                let value = rest.trim();
                let value_col = start_col + "servings:".chars().count() + leading_ws;

                if value.is_empty() {
                    return Err(ParseError {
                        file: file.to_string(),
                        line: line_no,
                        col: value_col,
                        message: "expected a number after 'servings:'".to_string(),
                        source_line: raw_line.to_string(),
                        span: 1,
                    });
                }

                let n = parse_number(value, line_no, value_col, raw_line, file)?;
                if n <= 0.0 {
                    return Err(ParseError {
                        file: file.to_string(),
                        line: line_no,
                        col: value_col,
                        message: format!("servings must be greater than zero, found \"{}\"", value),
                        source_line: raw_line.to_string(),
                        span: value.chars().count().max(1),
                    });
                }
                servings = Some(n);
                continue;
            }

            return Err(ParseError {
                file: file.to_string(),
                line: line_no,
                col: start_col,
                message: format!("expected a 'servings: <number>' line, found \"{}\"", content),
                source_line: raw_line.to_string(),
                span: content.chars().count().max(1),
            });
        }

        ingredients.push(parse_ingredient_line(raw_line, line_no, file)?);
    }

    let servings = servings.ok_or_else(|| ParseError {
        file: file.to_string(),
        line: last_line + 1,
        col: 1,
        message: "recipe is missing a 'servings: <number>' line".to_string(),
        source_line: String::new(),
        span: 1,
    })?;

    if ingredients.is_empty() {
        return Err(ParseError {
            file: file.to_string(),
            line: last_line + 1,
            col: 1,
            message: "recipe has no ingredient lines".to_string(),
            source_line: String::new(),
            span: 1,
        });
    }

    Ok(Recipe {
        title,
        servings,
        ingredients,
    })
}

// An ingredient line is a quantity followed by free-text description, e.g.
// "1 1/2 cups flour". The quantity may be a whole number, a decimal, a
// simple fraction ("1/2"), or a mixed number ("1 1/2").
fn parse_ingredient_line(raw_line: &str, line_no: usize, file: &str) -> Result<Ingredient, ParseError> {
    let start_col = first_non_ws_col(raw_line);
    let content = raw_line.trim();
    let chars: Vec<char> = content.chars().collect();

    if chars.is_empty() {
        return Err(ParseError {
            file: file.to_string(),
            line: line_no,
            col: start_col,
            message: "expected an ingredient line, found nothing".to_string(),
            source_line: raw_line.to_string(),
            span: 1,
        });
    }

    let mut i = 0;
    while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
        i += 1;
    }
    if i == 0 {
        return Err(ParseError {
            file: file.to_string(),
            line: line_no,
            col: start_col,
            message: format!(
                "expected a quantity (a number like \"2\", a fraction like \"1/2\", or a mixed number like \"1 1/2\"), found \"{}\"",
                chars[0]
            ),
            source_line: raw_line.to_string(),
            span: 1,
        });
    }

    let first_token: String = chars[0..i].iter().collect();
    let mut quantity = parse_number(&first_token, line_no, start_col, raw_line, file)?;
    let mut consumed = i;

    if i < chars.len() && chars[i] == '/' {
        let denom_start = i + 1;
        let mut j = denom_start;
        while j < chars.len() && chars[j].is_ascii_digit() {
            j += 1;
        }
        if j == denom_start {
            return Err(ParseError {
                file: file.to_string(),
                line: line_no,
                col: start_col + denom_start,
                message: "expected a number after '/'".to_string(),
                source_line: raw_line.to_string(),
                span: 1,
            });
        }
        let denom_text: String = chars[denom_start..j].iter().collect();
        let denominator = parse_number(&denom_text, line_no, start_col + denom_start, raw_line, file)?;
        if denominator == 0.0 {
            return Err(ParseError {
                file: file.to_string(),
                line: line_no,
                col: start_col + denom_start,
                message: "fraction denominator cannot be zero".to_string(),
                source_line: raw_line.to_string(),
                span: denom_text.chars().count().max(1),
            });
        }
        quantity /= denominator;
        consumed = j;
    } else {
        let mut j = i;
        while j < chars.len() && chars[j] == ' ' {
            j += 1;
        }
        if j > i && j < chars.len() && chars[j].is_ascii_digit() {
            let frac_num_start = j;
            let mut k = frac_num_start;
            while k < chars.len() && chars[k].is_ascii_digit() {
                k += 1;
            }
            if k < chars.len() && chars[k] == '/' {
                let numerator_text: String = chars[frac_num_start..k].iter().collect();
                let denom_start = k + 1;
                let mut m = denom_start;
                while m < chars.len() && chars[m].is_ascii_digit() {
                    m += 1;
                }
                if m == denom_start {
                    return Err(ParseError {
                        file: file.to_string(),
                        line: line_no,
                        col: start_col + denom_start,
                        message: "expected a number after '/'".to_string(),
                        source_line: raw_line.to_string(),
                        span: 1,
                    });
                }
                let denom_text: String = chars[denom_start..m].iter().collect();
                let numerator = parse_number(&numerator_text, line_no, start_col + frac_num_start, raw_line, file)?;
                let denominator = parse_number(&denom_text, line_no, start_col + denom_start, raw_line, file)?;
                if denominator == 0.0 {
                    return Err(ParseError {
                        file: file.to_string(),
                        line: line_no,
                        col: start_col + denom_start,
                        message: "fraction denominator cannot be zero".to_string(),
                        source_line: raw_line.to_string(),
                        span: denom_text.chars().count().max(1),
                    });
                }
                quantity += numerator / denominator;
                consumed = m;
            }
        }
    }

    let description = content[byte_offset(content, consumed)..].trim();
    if description.is_empty() {
        return Err(ParseError {
            file: file.to_string(),
            line: line_no,
            col: start_col + consumed,
            message: "missing ingredient name after quantity".to_string(),
            source_line: raw_line.to_string(),
            span: 1,
        });
    }

    Ok(Ingredient {
        quantity,
        description: description.to_string(),
    })
}

fn parse_number(text: &str, line: usize, col: usize, raw_line: &str, file: &str) -> Result<f64, ParseError> {
    text.parse::<f64>().map_err(|_| ParseError {
        file: file.to_string(),
        line,
        col,
        message: format!("\"{}\" is not a valid number", text),
        source_line: raw_line.to_string(),
        span: text.chars().count().max(1),
    })
}

fn first_non_ws_col(line: &str) -> usize {
    line.chars().take_while(|c| c.is_whitespace()).count() + 1
}

fn byte_offset(s: &str, char_count: usize) -> usize {
    s.char_indices().nth(char_count).map(|(b, _)| b).unwrap_or(s.len())
}
