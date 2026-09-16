mod fraction;
mod parser;
mod scale;

use fraction::Fraction;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.iter().any(|a| a == "-h" || a == "--help") {
        print_usage(&args[0]);
        if args.len() < 2 {
            process::exit(1);
        }
        return;
    }

    let path = &args[1];
    let mut target_servings: Option<Fraction> = None;
    let mut factor: Option<Fraction> = None;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--servings" => {
                let value = args.get(i + 1).unwrap_or_else(|| {
                    eprintln!("error: --servings requires a value");
                    process::exit(1);
                });
                target_servings = Some(fraction::parse_decimal(value).unwrap_or_else(|| {
                    eprintln!("error: --servings value \"{}\" is not a number", value);
                    process::exit(1);
                }));
                i += 2;
            }
            "--factor" => {
                let value = args.get(i + 1).unwrap_or_else(|| {
                    eprintln!("error: --factor requires a value");
                    process::exit(1);
                });
                factor = Some(fraction::parse_decimal(value).unwrap_or_else(|| {
                    eprintln!("error: --factor value \"{}\" is not a number", value);
                    process::exit(1);
                }));
                i += 2;
            }
            other => {
                eprintln!("error: unrecognized argument \"{}\"", other);
                process::exit(1);
            }
        }
    }

    if target_servings.is_some() && factor.is_some() {
        eprintln!("error: pass either --servings or --factor, not both");
        process::exit(1);
    }

    let source = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("error: could not read \"{}\": {}", path, err);
        process::exit(1);
    });

    let recipe = match parser::parse(&source, path) {
        Ok(recipe) => recipe,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let scale_factor = match (target_servings, factor) {
        (Some(target), None) => target.div(recipe.servings),
        (None, Some(f)) => f,
        (None, None) => {
            eprintln!("error: specify --servings <n> or --factor <n>");
            process::exit(1);
        }
        (Some(_), Some(_)) => unreachable!(),
    };

    if !scale_factor.is_positive() {
        eprintln!("error: scale factor must be greater than zero");
        process::exit(1);
    }

    if let Some(title) = &recipe.title {
        println!("{}", title);
    }
    let new_servings = recipe.servings.mul(scale_factor);
    println!("servings: {}", scale::format_quantity(new_servings));
    println!();

    for ingredient in &recipe.ingredients {
        let scaled = ingredient.quantity.mul(scale_factor);
        println!("{} {}", scale::format_quantity(scaled), ingredient.description);
    }
}

fn print_usage(program: &str) {
    eprintln!("usage: {} <recipe-file> (--servings <n> | --factor <n>)", program);
    eprintln!();
    eprintln!("examples:");
    eprintln!("  {} examples/pancakes.recipe --servings 12", program);
    eprintln!("  {} examples/pancakes.recipe --factor 1.5", program);
}
