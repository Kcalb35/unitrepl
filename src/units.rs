use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dim {
    pub len: i64,
    pub mass: i64,
    pub time: i64,
    pub temp: i64,
    pub curr: i64,
}

pub struct UnitDef {
    pub symbol: &'static str,
    pub dim: Dim,
    pub factor: f64,
}
impl Display for Dim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // (symbol, exponent)
        let dims = [
            ("L", self.len),
            ("M", self.mass),
            ("T", self.time),
            ("Θ", self.temp),
            ("I", self.curr),
        ];

        let mut first = true;

        for (sym, exp) in dims {
            if exp == 0 {
                continue;
            }

            if !first {
                write!(f, "_")?;
            }
            first = false;

            if exp == 1 {
                write!(f, "{sym}")?;
            } else {
                write!(f, "{sym}^{exp}")?;
            }
        }

        if first {
            // all zero => dimensionless
            write!(f, "1")?;
        }

        Ok(())
    }
}

impl Dim {
    pub const fn zero() -> Dim {
        Dim::new(0, 0, 0, 0, 0)
    }

    pub const fn new(len: i64, mass: i64, time: i64, temp: i64, curr: i64) -> Dim {
        Dim {
            len,
            mass,
            time,
            temp,
            curr,
        }
    }

    pub const fn len(p: i64) -> Dim {
        Dim {
            len: p,
            ..Dim::zero()
        }
    }

    pub const fn mass(p: i64) -> Dim {
        Dim {
            mass: p,
            ..Dim::zero()
        }
    }

    pub const fn time(p: i64) -> Dim {
        Dim {
            time: p,
            ..Dim::zero()
        }
    }

    pub const fn temp(p: i64) -> Dim {
        Dim {
            temp: p,
            ..Dim::zero()
        }
    }

    pub const fn curr(p: i64) -> Dim {
        Dim {
            curr: p,
            ..Dim::zero()
        }
    }

    pub const fn mul(self, other: Dim) -> Dim {
        Dim {
            len: self.len + other.len,
            mass: self.mass + other.mass,
            time: self.time + other.time,
            temp: self.temp + other.temp,
            curr: self.curr + other.curr,
        }
    }

    pub const fn div(self, other: Dim) -> Dim {
        Dim {
            len: self.len - other.len,
            mass: self.mass - other.mass,
            time: self.time - other.time,
            temp: self.temp - other.temp,
            curr: self.curr - other.curr,
        }
    }
    pub const fn pow(self, k: i64) -> Dim {
        Dim {
            len: self.len * k,
            mass: self.mass * k,
            time: self.time * k,
            temp: self.temp * k,
            curr: self.curr * k,
        }
    }
}
