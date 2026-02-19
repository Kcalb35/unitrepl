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
        (UnitTarget::Au, UnitTarget::Unit(to)) => (au_to_si(to.dim), to.factor, to.symbol),
        (UnitTarget::Unit(from), UnitTarget::Au) => (from.factor, au_to_si(from.dim),"au"),
        (UnitTarget::Unit(from), UnitTarget::Unit(to)) => (from.factor, to.factor, to.symbol),
    };
    let value = expr.value * from_factor / to_factor;
    format!("{} {}",buffer.format(value),symbol)
}
