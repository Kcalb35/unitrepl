use thiserror::Error;

use crate::constants::UNIT_DEF_MAP;
use crate::units::Dim;
#[derive(Debug, Error)]
pub enum ParseError<'a> {
    #[error("Empty expr")]
    Empty,
    #[error("usage: <number> <unit> to <unit>")]
    BadSyntax,
    #[error("Invalid number: {0}")]
    InvalidNumber(&'a str),
    #[error("Unknown unit: {0}")]
    UnknownUnit(&'a str),
    #[error("Missing 'to'")]
    ExpectTo,
    #[error("Incompatible dimension {0} to {1}")]
    IncompatibleDim(Dim, Dim),
    #[error("Au to Au no supported")]
    AuToAu,
}

pub struct UnitExpr<'a> {
    pub symbol: &'a str,
    pub dim: Dim,
    pub factor: f64,
}

pub enum UnitTarget<'a> {
    Au,
    Unit(UnitExpr<'a>),
}

pub struct ConversionExpr<'a> {
    pub value: f64,
    pub from: UnitTarget<'a>,
    pub to: UnitTarget<'a>,
}
pub fn parse_target(s: &str) -> Result<UnitTarget<'_>, ParseError<'_>> {
    if s.eq_ignore_ascii_case("au") || s.eq_ignore_ascii_case("a.u.") {
        return Ok(UnitTarget::Au);
    }
    let unit = UNIT_DEF_MAP.get(s).ok_or(ParseError::UnknownUnit(s))?;
    let unit = UnitExpr {
        dim: unit.dim,
        factor: unit.factor,
        symbol: unit.symbol,
    };
    Ok(UnitTarget::Unit(unit))
}

pub fn parse_expr(s: &str) -> Result<ConversionExpr<'_>, ParseError<'_>> {
    let s = s.trim();
    if s.is_empty() {
        return Err(ParseError::Empty);
    }
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 4 {
        return Err(ParseError::BadSyntax);
    }
    if parts[2] != "to" {
        return Err(ParseError::ExpectTo);
    }
    let value: f64 = parts[0]
        .parse()
        .map_err(|_| ParseError::InvalidNumber(parts[0]))?;
    let from = parse_target(parts[1])?;
    let to = parse_target(parts[3])?;
    if let UnitTarget::Unit(expr1) = &from
        && let UnitTarget::Unit(expr2) = &to
    {
        if expr1.dim != expr2.dim {
            return Err(ParseError::IncompatibleDim(expr1.dim, expr2.dim));
        }
    }
    if let UnitTarget::Au = &from
        && let UnitTarget::Au = &to
    {
        return Err(ParseError::AuToAu);
    }
    Ok(ConversionExpr { value, from, to })
}
