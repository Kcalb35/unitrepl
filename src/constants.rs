use std::{collections::HashMap, sync::LazyLock};

use crate::units::{Dim, UnitDef};

pub const AU_MASS: f64 = 9.1093826e-31;
pub const AU_LENGTH: f64 = 5.291772108e-11;
pub const AU_TIME: f64 = 2.418884326505e-17;
pub const AU_CURRENT: f64 = 6.62361782e-3;
pub const AU_TEMPERATURE: f64 = 3.1577464e5;
pub const E: f64 = 1.60217653e-19;

type UnitsType = &'static [(&'static str, f64)];

pub static LENGTH_UNITS: UnitsType = &[
    ("m", 1.0),
    ("km", 1e3),
    ("dm", 1e-1),
    ("cm", 1e-2),
    ("mm", 1e-3),
    ("um", 1e-6),
    ("nm", 1e-9),
    ("angstrom", 1e-10),
    ("ang", 1e-10),
    ("pm", 1e-12),
    ("fm", 1e-15),
];

pub static TIME_UNITS: UnitsType = &[
    ("day", 86400.0), // 24 * 3600
    ("hour", 3600.0),
    ("minute", 60.0),
    ("min", 60.0),
    ("s", 1.0),
    ("ms", 1e-3),
    ("us", 1e-6),
    ("ns", 1e-9),
    ("ps", 1e-12),
    ("fs", 1e-15),
];

pub static TEMP_UNITS: UnitsType = &[("K", 1.0)];

pub static CURRENT_UNITS: UnitsType = &[
    ("A", 1.0),
    ("kA", 1e3),
    ("MA", 1e6),
    ("mA", 1e-3),
    ("uA", 1e-6),
    ("nA", 1e-9),
    ("pA", 1e-12),
    ("fA", 1e-15),
];

pub static MASS_UNITS: UnitsType = &[
    ("kg", 1.0),
    ("g", 1e-3),
    ("mg", 1e-6),
    ("ug", 1e-9),
    ("ng", 1e-12),
    ("pg", 1e-15),
    ("t", 1e3),
    ("amu", 1.66053904e-27),
    ("Da", 1.66053904e-27),
];
pub static ENERGY_UNITS: UnitsType = &[
    // SI joule family
    ("J", 1.0),
    ("kJ", 1e3),
    ("MJ", 1e6),
    ("GJ", 1e9),
    ("mJ", 1e-3),
    ("uJ", 1e-6),
    ("nJ", 1e-9),
    ("pJ", 1e-12),
    // electron-volt (exact, since e is exact)
    ("eV", E),
    ("keV", 1e3 * E),
    ("MeV", 1e6 * E),
    ("GeV", 1e9 * E),
    ("cal", 4.184),
    ("kcal", 4184.0),
    ("Cal", 4184.0), // food Calorie = kilocalorie
    ("Wh", 3600.0),
    ("kWh", 3.6e6),
    ("MWh", 3.6e9),
];

pub static CHARGE_UNITS: UnitsType = &[
    ("C", 1.0),
    ("kC", 1e3),
    ("mC", 1e-3),
    ("uC", 1e-6),
    ("nC", 1e-9),
    ("pC", 1e-12),
    ("Ah", 3600.0),
    ("mAh", 3.6),
];

pub static FORCE_UNITS: UnitsType = &[
    ("N", 1.0),
    ("kN", 1e3),
    ("MN", 1e6),
    ("mN", 1e-3),
    ("uN", 1e-6),
    ("dyn", 1e-5), // dyne
];

pub static PRESSURE_UNITS: UnitsType = &[
    ("Pa", 1.0),
    ("hPa", 100.0),
    ("kPa", 1e3),
    ("MPa", 1e6),
    ("GPa", 1e9),
    ("bar", 1e5),
    ("mbar", 1e2),
    ("atm", 101_325.0),
    ("Torr", 133.32236842105263),
    ("mmHg", 133.322387415),
    ("psi", 6894.757293168),
    ("barye", 0.1), // 1 Ba = 0.1 Pa
];
pub static UNIT_GROUP_MAP: LazyLock<HashMap<&'static str, UnitsType>> = LazyLock::new(|| {
    HashMap::from([
        ("length", LENGTH_UNITS as UnitsType),
        ("time", TIME_UNITS as UnitsType),
        ("temperature", TEMP_UNITS as UnitsType),
        ("current", CURRENT_UNITS as UnitsType),
        ("mass", MASS_UNITS as UnitsType),
        ("energy", ENERGY_UNITS as UnitsType),
        ("charge", CHARGE_UNITS as UnitsType),
        ("force", FORCE_UNITS as UnitsType),
        ("pressure", PRESSURE_UNITS as UnitsType),
    ])
});

fn to_entry(name: &'static str, dim: Dim, factor_to_si: f64) -> (&'static str, UnitDef) {
    (
        name,
        UnitDef {
            symbol: name,
            dim,
            factor: factor_to_si,
        },
    )
}

pub static UNIT_DEF_MAP: LazyLock<HashMap<&'static str, UnitDef>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.extend(
        LENGTH_UNITS
            .iter()
            .map(|&(name, f)| to_entry(name, Dim::len(1), f)),
    );

    map.extend(
        TIME_UNITS
            .iter()
            .map(|&(name, f)| to_entry(name, Dim::time(1), f)),
    );

    map.extend(
        TEMP_UNITS
            .iter()
            .map(|&(name, f)| to_entry(name, Dim::temp(1), f)),
    );

    map.extend(
        CURRENT_UNITS
            .iter()
            .map(|&(name, f)| to_entry(name, Dim::curr(1), f)),
    );

    map.extend(
        MASS_UNITS
            .iter()
            .map(|&(name, f)| to_entry(name, Dim::mass(1), f)),
    );
    map.extend(
        ENERGY_UNITS
            .iter()
            .map(|&(name, f)| to_entry(name, Dim::new(2, 1, -2, 0, 0), f)),
    );
    map.extend(
        CHARGE_UNITS
            .iter()
            .map(|&(name, f)| to_entry(name, Dim::new(0, 0, 1, 0, 1), f)),
    );

    map.extend(
        FORCE_UNITS
            .iter()
            .map(|&(name, f)| to_entry(name, Dim::new(1, 1, -2, 0, 0), f)),
    );

    map.extend(
        PRESSURE_UNITS
            .iter()
            .map(|&(name, f)| to_entry(name, Dim::new(-1, 1, -2, 0, 0), f)),
    );
    map
});
