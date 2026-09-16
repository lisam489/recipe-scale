# recipe-scale

Recipes are written for whatever batch size the author happened to cook.
Doubling one is easy until the ingredient list has a mixed number like
"1 1/2 cups" in it, and it gets worse the moment you need to go from 4
servings to 11. `recipe-scale` reads a plain text recipe file and reprints
it with every quantity rescaled, either to a target number of servings or
by an arbitrary factor.

## Recipe file format

```
title: Buttermilk Pancakes
servings: 4

1 1/2 cups flour
3 1/2 tsp baking powder
1 tsp salt
1 tbsp sugar
1 1/4 cups milk
1 egg
3 tbsp butter, melted
```

- `title:` is optional.
- `servings:` is required and must come before the ingredient lines.
- Each ingredient line starts with a quantity (a whole number, a decimal,
  a simple fraction like `1/2`, or a mixed number like `1 1/2`) followed
  by the rest of the line as free text.
- Blank lines and lines starting with `#` are ignored.

See `examples/pancakes.recipe` for a complete file.

## Usage

```
recipe-scale examples/pancakes.recipe --servings 8
```

```
Buttermilk Pancakes
servings: 8

3 cups flour
7 tsp baking powder
2 tsp salt
2 tbsp sugar
2 1/2 cups milk
2 egg
6 tbsp butter, melted
```

Or scale by a plain multiplier instead of a target serving count:

```
recipe-scale examples/pancakes.recipe --factor 1.5
```

## Error messages

Recipe files are hand-edited, so the most common mistake is a stray
character in a quantity, or an ingredient line with no quantity at all.
Errors point at the exact line and column rather than just naming the
problem:

```
error: expected a quantity (a number like "2", a fraction like "1/2", or a mixed number like "1 1/2"), found "a"
  --> soup.recipe:5:1
 |
5 | a pinch of salt
 | ^
```

## Building

No external crates are used, so a plain `cargo build --release` is all
that's needed. The binary ends up at `target/release/recipe-scale`.

## Status

Early skeleton. Quantities are scaled with exact fraction arithmetic, so
scaling never drifts the way repeated floating point multiplication would.
Sharper edges (unit conversion, plural ingredient names) are still open.

## License

MIT, see `LICENSE`.
