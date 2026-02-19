use thiserror::Error;

use crate::constants::UNIT_DEF_MAP;
use crate::units::Dim;

#[derive(Debug, Error)]
pub enum ParseErrorKind<'a> {
    #[error("Line is empty")]
    Empty,

    #[error("Invalid number")]
    InvalidNumber,

    #[error("Invalid exponent")]
    InvalidExponent,

    #[error("Missing 'to'")]
    MissingTo,

    #[error("Bad syntax {0}")]
    BadSyntax(&'a str),

    #[error("Unknown unit: {0}")]
    UnknownUnit(&'a str),

    #[error("Incompatible dimension from:{0} to:{1}")]
    IncompatibleDim(Dim, Dim),

    #[error("Au to Au not supported")]
    AuToAu,

    #[error("Au must be single")]
    AuMustSingle,

    #[error("Unexpected Number")]
    UnexpectedNumber,

    #[error("Unexpected Char {0}")]
    UnexpectedChar(char),
}

#[derive(Debug)]
pub struct ParseError<'a> {
    pub line: &'a str,
    pub pos: Option<usize>,
    pub kind: ParseErrorKind<'a>,
}

#[derive(Debug)]
pub struct UnitExpr {
    pub symbol: String,
    pub dim: Dim,
    pub factor: f64,
}

#[derive(Debug)]
pub enum UnitTarget {
    Au,
    Unit(UnitExpr),
}

impl<'a> ParseError<'a> {
    pub fn new(line: &'a str, pos: Option<usize>, kind: ParseErrorKind<'a>) -> Self {
        Self { line, pos, kind }
    }
}
impl std::fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self.line.trim_end_matches(['\n', '\r']);
        writeln!(f, "{line}")?;
        if let Some(pos) = self.pos {
            writeln!(f, "{}^---{}", " ".repeat(pos), self.kind)?;
        } else {
            writeln!(f, "{}", self.kind)?;
        }
        Ok(())
    }
}
impl ParseError<'_> {
    pub fn format_repl(&self) -> String {
        if let Some(pos) = self.pos {
            format!("      {}^---{}", " ".repeat(pos), self.kind)
        } else {
            format!("{}", self.kind)
        }
    }
}

struct Lexer<'a> {
    s: &'a str,
    pos: usize,
}
impl<'a> Lexer<'a> {
    fn new(s: &'a str) -> Self {
        Self { s, pos: 0 }
    }
    fn err<T>(&self, kind: ParseErrorKind<'a>) -> Result<T, ParseError<'a>> {
        Err(ParseError::new(self.s, Some(self.pos), kind))
    }
    fn peek(&self) -> Option<char> {
        self.s[self.pos..].chars().next()
    }
    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }
    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.bump();
        }
    }
    fn parse_number(&mut self) -> Result<f64, ParseError<'a>> {
        self.skip_ws();
        let start = self.pos;
        if matches!(self.peek(), Some('+' | '-')) {
            self.bump();
        }
        let mut saw_digit = false;
        let mut saw_dot = false;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                self.bump();
                saw_digit = true;
            } else if ch == '.' && !saw_dot {
                self.bump();
                saw_dot = true;
            } else {
                break;
            }
        }
        if !saw_digit {
            return self.err(ParseErrorKind::InvalidNumber);
        }
        if matches!(self.peek(), Some('e' | 'E')) {
            self.bump();

            if matches!(self.peek(), Some('+' | '-')) {
                self.bump();
            }
            let mut saw_exp_digit = false;
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    self.bump();
                    saw_exp_digit = true;
                } else {
                    break;
                }
            }
            if !saw_exp_digit {
                return self.err(ParseErrorKind::InvalidExponent);
            }
            if let Some(c) = self.peek()
                && !c.is_ascii_whitespace()
            {
                return self.err(ParseErrorKind::InvalidExponent);
            }
        }
        if let Some(c) = self.peek()
            && !c.is_ascii_whitespace()
        {
            return self.err(ParseErrorKind::InvalidNumber);
        }
        let num_str = &self.s[start..self.pos];
        match num_str.parse::<f64>() {
            Ok(v) => Ok(v),
            Err(_) => self.err(ParseErrorKind::InvalidNumber),
        }
    }
    fn parse_unit_expr(&mut self) -> Result<UnitTarget, ParseError<'a>> {
        self.skip_ws();
        let mut lhs = self.parse_unit_term()?;

        loop {
            self.skip_ws();
            let start = self.pos;
            match self.peek() {
                Some('*') => {
                    self.bump();
                    let rhs = self.parse_unit_term()?;
                    if let UnitTarget::Unit(l_expr) = &mut lhs
                        && let UnitTarget::Unit(r_expr) = &rhs
                    {
                        l_expr.dim.mul(r_expr.dim);
                        l_expr.factor *= r_expr.factor;
                        l_expr
                            .symbol
                            .push_str(format!("*{}", r_expr.symbol).as_str());
                    } else {
                        self.pos = start;
                        return self.err(ParseErrorKind::AuMustSingle);
                    }
                }
                Some('/') => {
                    self.bump();
                    let rhs = self.parse_unit_term()?;
                    if let UnitTarget::Unit(l_expr) = &mut lhs
                        && let UnitTarget::Unit(r_expr) = &rhs
                    {
                        l_expr.dim.div(r_expr.dim);
                        l_expr.factor /= r_expr.factor;
                        l_expr
                            .symbol
                            .push_str(format!("/{}", r_expr.symbol).as_str());
                    } else {
                        self.pos = start;
                        return self.err(ParseErrorKind::AuMustSingle);
                    }
                }
                _ => break,
            }
        }
        Ok(lhs)
    }
    fn parse_unit_term(&mut self) -> Result<UnitTarget, ParseError<'a>> {
        self.skip_ws();
        let mut atom = self.parse_unit_atom()?;
        self.skip_ws();
        if matches!(self.peek(), Some('^')) {
            self.bump();
            let k = self.parse_signed_int()?;
            if let UnitTarget::Unit(expr) = &mut atom {
                expr.dim.pow(k);
                expr.factor = expr.factor.powi(k as i32);
                expr.symbol.push_str(format!("^{}", k).as_str());
            } else {
                return self.err(ParseErrorKind::AuMustSingle);
            }
        }
        Ok(atom)
    }
    fn parse_unit_atom(&mut self) -> Result<UnitTarget, ParseError<'a>> {
        self.skip_ws();
        let start = self.pos;
        if matches!(self.peek(), Some(c) if c.is_ascii_digit() || c=='+' || c== '-') {
            return self.err(ParseErrorKind::UnexpectedNumber);
        }
        let name = self.read_ident_token()?;
        let unit_str = &self.s[start..self.pos];
        if unit_str == "au" {
            Ok(UnitTarget::Au)
        } else if let Some(e) = UNIT_DEF_MAP.get(name) {
            Ok(UnitTarget::Unit(UnitExpr {
                symbol: String::from(unit_str),
                dim: e.dim,
                factor: e.factor,
            }))
        } else {
            let ident_str = &self.s[start..self.pos];
            self.pos = start;
            self.err(ParseErrorKind::UnknownUnit(ident_str))
        }
    }
    // 字母开头，后面可以接数字或者点
    fn read_ident_token(&mut self) -> Result<&'a str, ParseError<'a>> {
        self.skip_ws();
        let start = self.pos;

        let first = self.peek().ok_or(ParseError {
            kind: ParseErrorKind::BadSyntax("expect token"),
            line: self.s,
            pos: Some(self.pos),
        })?;
        if !first.is_ascii_alphabetic() {
            return self.err(ParseErrorKind::UnexpectedChar(first));
        }
        while matches!(self.peek(), Some(c) if c.is_alphanumeric() || c=='.') {
            self.bump();
        }

        Ok(&self.s[start..self.pos])
    }
    fn parse_signed_int(&mut self) -> Result<i64, ParseError<'a>> {
        self.skip_ws();
        let start = self.pos;
        if matches!(self.peek(), Some('+' | '-')) {
            self.bump();
        }
        let mut saw_digit = false;
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            saw_digit = true;
            self.bump();
        }
        if !saw_digit {
            self.pos = start;
            return self.err(ParseErrorKind::InvalidNumber);
        }
        let num_str = &self.s[start..self.pos];
        match num_str.parse::<i64>() {
            Ok(v) => Ok(v),
            Err(_) => self.err(ParseErrorKind::InvalidExponent),
        }
    }
    fn consume_kw_to(&mut self) -> Result<(), ParseError<'a>> {
        self.skip_ws();
        let start = self.pos;
        let ident = self.read_ident_token()?;
        if ident == "to" {
            Ok(())
        } else {
            self.pos = start;
            self.err(ParseErrorKind::MissingTo)
        }
    }
    fn ensure_eof(&mut self) -> Result<(), ParseError<'a>> {
        self.skip_ws();
        if self.peek() == None {
            Ok(())
        } else {
            self.err(ParseErrorKind::BadSyntax("trailing input"))
        }
    }
}

#[derive(Debug)]
pub struct ConversionExpr {
    pub value: f64,
    pub from: UnitTarget,
    pub to: UnitTarget,
}

pub fn parse_expr(line: &str) -> Result<ConversionExpr, ParseError<'_>> {
    let mut lexer = Lexer::new(line);
    if let None = lexer.peek() {
        return Err(ParseError::new(line, None, ParseErrorKind::Empty));
    }

    let value = lexer.parse_number()?;
    let from = lexer.parse_unit_expr()?;
    lexer.consume_kw_to()?;
    let to = lexer.parse_unit_expr()?;
    lexer.ensure_eof()?;

    if let UnitTarget::Unit(expr1) = &from
        && let UnitTarget::Unit(expr2) = &to
    {
        if expr1.dim != expr2.dim {
            return Err(ParseError::new(
                line,
                None,
                ParseErrorKind::IncompatibleDim(expr1.dim, expr2.dim),
            ));
        }
    }
    if let UnitTarget::Au = &from
        && let UnitTarget::Au = &to
    {
        return Err(ParseError::new(line, None, ParseErrorKind::AuToAu));
    }
    Ok(ConversionExpr { value, from, to })
}


#[cfg(test)]
mod tests {
    use super::*;

    fn assert_err<'a>(
        line: &'a str,
        expected_pos: Option<usize>,
        check: impl FnOnce(&ParseErrorKind<'a>),
    ) {
        let err = parse_expr(line).expect_err("expected error");
        assert_eq!(err.pos, expected_pos);
        check(&err.kind);
    }

    #[test]
    fn empty_line() {
        assert_err("", None, |k| assert!(matches!(k, ParseErrorKind::Empty)));
    }

    #[test]
    fn whitespace_only_is_invalid_number() {
        assert_err("   \n", Some(4), |k| assert!(matches!(k, ParseErrorKind::InvalidNumber)));
    }

    #[test]
    fn invalid_number_no_digits() {
        assert_err("-", Some(1), |k| assert!(matches!(k, ParseErrorKind::InvalidNumber)));
        assert_err("+", Some(1), |k| assert!(matches!(k, ParseErrorKind::InvalidNumber)));
        assert_err(".", Some(1), |k| assert!(matches!(k, ParseErrorKind::InvalidNumber)));
    }

    #[test]
    fn invalid_number_trailing_garbage() {
        assert_err("1x km to m", Some(1), |k| assert!(matches!(k, ParseErrorKind::InvalidNumber)));
        assert_err(
            "1.2.3 km to m",
            Some(3),
            |k| assert!(matches!(k, ParseErrorKind::InvalidNumber)),
        );
    }

    #[test]
    fn invalid_exponent() {
        assert_err("1e km to m", Some(2), |k| assert!(matches!(k, ParseErrorKind::InvalidExponent)));
        assert_err(
            "1e+ km to m",
            Some(3),
            |k| assert!(matches!(k, ParseErrorKind::InvalidExponent)),
        );
        assert_err(
            "1e2x km to m",
            Some(3),
            |k| assert!(matches!(k, ParseErrorKind::InvalidExponent)),
        );
    }

    #[test]
    fn missing_to_keyword() {
        assert_err("1 km m", Some(5), |k| assert!(matches!(k, ParseErrorKind::MissingTo)));
        assert_err("1 km too m", Some(5), |k| assert!(matches!(k, ParseErrorKind::MissingTo)));
    }

    #[test]
    fn unknown_unit() {
        assert_err("1 xyz to m", Some(2), |k| match k {
            ParseErrorKind::UnknownUnit(u) => assert_eq!(*u, "xyz"),
            _ => panic!("unexpected kind: {k:?}"),
        });
    }

    #[test]
    fn unexpected_char_in_unit() {
        assert_err("1 *m to m", Some(2), |k| match k {
            ParseErrorKind::UnexpectedChar('*') => {}
            _ => panic!("unexpected kind: {k:?}"),
        });
        assert_err("1 (m to m", Some(2), |k| match k {
            ParseErrorKind::UnexpectedChar('(') => {}
            _ => panic!("unexpected kind: {k:?}"),
        });
    }

    #[test]
    fn unexpected_number_in_unit() {
        assert_err("1 2m to m", Some(2), |k| assert!(matches!(k, ParseErrorKind::UnexpectedNumber)));
        assert_err("1 -m to m", Some(2), |k| assert!(matches!(k, ParseErrorKind::UnexpectedNumber)));
    }

    #[test]
    fn trailing_input() {
        assert_err("1 km to m foo", Some(10), |k| match k {
            ParseErrorKind::BadSyntax(msg) => assert_eq!(*msg, "trailing input"),
            _ => panic!("unexpected kind: {k:?}"),
        });
    }

    #[test]
    fn bad_syntax_expect_token() {
        assert_err("1 ", Some(2), |k| match k {
            ParseErrorKind::BadSyntax(msg) => assert_eq!(*msg, "expect token"),
            _ => panic!("unexpected kind: {k:?}"),
        });
    }

    #[test]
    fn invalid_exponent_in_pow() {
        assert_err("1 m^ to m", Some(5), |k| assert!(matches!(k, ParseErrorKind::InvalidNumber)));
        assert_err("1 m^+ to m", Some(4), |k| assert!(matches!(k, ParseErrorKind::InvalidNumber)));
    }

    #[test]
    fn au_to_au_not_supported() {
        assert_err("1 au to au", None, |k| assert!(matches!(k, ParseErrorKind::AuToAu)));
    }

    #[test]
    fn au_must_single_for_mul_div_pow() {
        assert_err("1 au*m to m", Some(4), |k| assert!(matches!(k, ParseErrorKind::AuMustSingle)));
        assert_err("1 m/au to m", Some(3), |k| assert!(matches!(k, ParseErrorKind::AuMustSingle)));
        assert_err("1 au^2 to m", Some(6), |k| assert!(matches!(k, ParseErrorKind::AuMustSingle)));
    }

    #[test]
    fn incompatible_dimension() {
        assert_err("1 km to s", None, |k| assert!(matches!(k, ParseErrorKind::IncompatibleDim(_, _))));
    }
}