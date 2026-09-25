use crate::fraction::Fraction;

// Recognizes the handful of kitchen units worth converting between, and
// picks a nicer unit to print once scaling has been applied. Scaling "1 tsp"
// by 12 gives "12 tsp", but nobody measures baking powder that way once it
// divides evenly into tablespoons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    Volume,
    Mass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unit {
    pub family: Family,
    pub base_per_unit: i64,
    pub singular: &'static str,
    pub plural: &'static str,
    aliases: &'static [&'static str],
}

const TEASPOON: Unit = Unit {
    family: Family::Volume,
    base_per_unit: 1,
    singular: "tsp",
    plural: "tsp",
    aliases: &["tsp", "tsps", "teaspoon", "teaspoons"],
};

const TABLESPOON: Unit = Unit {
    family: Family::Volume,
    base_per_unit: 3,
    singular: "tbsp",
    plural: "tbsp",
    aliases: &["tbsp", "tbsps", "tablespoon", "tablespoons"],
};

const CUP: Unit = Unit {
    family: Family::Volume,
    base_per_unit: 48,
    singular: "cup",
    plural: "cups",
    aliases: &["cup", "cups"],
};

const GRAM: Unit = Unit {
    family: Family::Mass,
    base_per_unit: 1,
    singular: "g",
    plural: "g",
    aliases: &["g", "gram", "grams"],
};

const KILOGRAM: Unit = Unit {
    family: Family::Mass,
    base_per_unit: 1000,
    singular: "kg",
    plural: "kg",
    aliases: &["kg", "kilogram", "kilograms"],
};

// Largest to smallest within each family, since best_display walks this
// looking for the biggest unit that divides the quantity evenly.
const VOLUME_UNITS: [Unit; 3] = [CUP, TABLESPOON, TEASPOON];
const MASS_UNITS: [Unit; 2] = [KILOGRAM, GRAM];
const ALL_UNITS: [Unit; 5] = [TEASPOON, TABLESPOON, CUP, GRAM, KILOGRAM];

pub fn recognize(word: &str) -> Option<Unit> {
    let lower = word.to_ascii_lowercase();
    ALL_UNITS.iter().copied().find(|u| u.aliases.contains(&lower.as_str()))
}

// Converts to the largest unit in the same family, no smaller than the
// ingredient's original unit, whose value comes out to a whole number.
// Never downgrades to a smaller unit: a recipe author who wrote "cups"
// meant cups, and a fraction like "2 1/2 cups" is already how a cook
// would read a measuring cup, so it is left alone.
pub fn best_display(unit: Unit, quantity: Fraction) -> (Fraction, Unit) {
    let base_amount = quantity.mul(Fraction::whole(unit.base_per_unit));
    let family_units: &[Unit] = match unit.family {
        Family::Volume => &VOLUME_UNITS,
        Family::Mass => &MASS_UNITS,
    };

    for &candidate in family_units.iter().filter(|c| c.base_per_unit >= unit.base_per_unit) {
        let value = base_amount.div(Fraction::whole(candidate.base_per_unit));
        if value.den == 1 {
            return (value, candidate);
        }
    }

    (quantity, unit)
}

pub fn display_name(unit: Unit, quantity: Fraction) -> &'static str {
    if quantity.num == quantity.den {
        unit.singular
    } else {
        unit.plural
    }
}
