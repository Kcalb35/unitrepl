use crate::constants::*;
use crate::parse::{ConversionExpr, UnitTarget};
use crate::units::Dim;

fn au_to_si(dim: Dim) -> f64 {
    AU_LENGTH.powi(dim.len as i32)
        * AU_MASS.powi(dim.mass as i32)
        * AU_TIME.powi(dim.time as i32)
        * AU_TEMPERATURE.powi(dim.temp as i32)
        * AU_CURRENT.powi(dim.curr as i32)
}

pub fn convert(expr: &ConversionExpr) -> String {
    let mut buffer = ryu::Buffer::new();
    let (from_factor, to_factor, symbol) = match (&expr.from, &expr.to) {
        (UnitTarget::Au, UnitTarget::Au) => unreachable!(),
        (UnitTarget::Au, UnitTarget::Unit(to)) => (au_to_si(to.dim), to.factor, to.symbol.as_str()),
        (UnitTarget::Unit(from), UnitTarget::Au) => (from.factor, au_to_si(from.dim), "au"),
        (UnitTarget::Unit(from), UnitTarget::Unit(to)) => {
            (from.factor, to.factor, to.symbol.as_str())
        }
    };
    let value = expr.value * from_factor / to_factor;
    format!("{} {}", buffer.format(value), symbol)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_expr;

    fn approx_eq(a: f64, b: f64, rel: f64, abs: f64) -> bool {
        let d = (a - b).abs();
        d <= abs || d <= rel * b.abs().max(1.0)
    }

    fn parse_value_and_unit(out: &str) -> (f64, &str) {
        let mut it = out.split_whitespace();
        let v = it.next().unwrap().parse::<f64>().unwrap();
        let u = it.next().unwrap();
        (v, u)
    }

    fn assert_conv(line: &str, expected: f64, expected_unit: &str) {
        let expr = parse_expr(line).unwrap();
        let out = convert(&expr);
        let (v, u) = parse_value_and_unit(&out);
        assert_eq!(u, expected_unit);
        assert!(
            approx_eq(v, expected, 1e-12, 1e-12),
            "line={line} out={out} expected={expected} {expected_unit}"
        );
    }

    #[test]
    fn basic_length() {
        assert_conv("10 km to m", 10_000.0, "m");
        assert_conv("1 m to cm", 100.0, "cm");
        assert_conv("250 cm to m", 2.5, "m");
        assert_conv("1 angstrom to m", 1e-10, "m");
    }

    #[test]
    fn basic_time() {
        assert_conv("2 hour to s", 7200.0, "s");
        assert_conv("90 min to hour", 1.5, "hour");
        assert_conv("1000 ms to s", 1.0, "s");
        assert_conv("1 day to hour", 24.0, "hour");
    }

    #[test]
    fn energy_and_charge() {
        assert_conv("1 kWh to J", 3.6e6, "J");
        assert_conv("500 mAh to C", 1800.0, "C");
        assert_conv("1 Ah to C", 3600.0, "C");
        assert_conv("2 kcal to J", 8368.0, "J");
    }

    #[test]
    fn force_and_pressure() {
        assert_conv("10 kN to N", 10_000.0, "N");
        assert_conv("1 bar to Pa", 100_000.0, "Pa");
        assert_conv("101325 Pa to atm", 1.0, "atm");
    }

    #[test]
    fn composite_units_mul_div_pow() {
        assert_conv("1 m/s to km/hour", 3.6, "km/hour");
        assert_conv("1 m^2 to cm^2", 10_000.0, "cm^2");
        assert_conv("1 m/s^2 to m/s^2", 1.0, "m/s^2");
        assert_conv("1000 N/m^2 to kPa", 1.0, "kPa");
    }

    #[test]
    fn au_conversions() {
        assert_conv("1 au to m", 5.291772108e-11, "m");
        assert_conv("1 m to au", 1.0 / 5.291772108e-11, "au");
        assert_conv("1 au to s", 2.418884326505e-17, "s");
        assert_conv("1 s to au", 1.0 / 2.418884326505e-17, "au");
    }

    #[test]
    fn number_formats() {
        assert_conv("1e3 m to km", 1.0, "km");
        assert_conv("+2.5e2 cm to m", 2.5, "m");
        assert_conv("-1.5e3 m to km", -1.5, "km");
    }
}