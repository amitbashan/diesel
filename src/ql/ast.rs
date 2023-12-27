use std::fmt;

use chrono::{Datelike, Month, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

use super::*;

pub enum Type {
    Number,
    Placeholder,
    Weekday,
    Month,
    Date,
}

pub enum TypeError {
    Mismatch,
}

pub type TypeResult<T> = Result<T, TypeError>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaceholderUnit {
    Weekday,
    Monthday,
    Month,
    Year,
    Date,
}

impl fmt::Display for PlaceholderUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Weekday => "wd",
                Self::Monthday => "md",
                Self::Month => "mo",
                Self::Year => "y",
                Self::Date => "date",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expression {
    Boolean(bool),
    Number(u32),
    Placeholder(PlaceholderUnit),
    Weekday(Weekday),
    Month(Month),
    Date(NaiveDate),
    Predicate(Box<Predicate>),
    Arithmetic(Box<Arithmetic>),
}

impl Expression {
    pub fn as_predicate(self) -> Option<Predicate> {
        match self {
            Self::Predicate(p) => {
                log::info!("{p:#?}");
                Some(*p)
            }
            _ => None,
        }
    }
}

impl Evaluate<TypeResult<Self>> for Expression {
    fn evaluate(&self, context: &Context) -> TypeResult<Self> {
        match self {
            Self::Placeholder(e) => Ok(match e {
                PlaceholderUnit::Weekday => Self::Weekday(context.date.weekday()),
                PlaceholderUnit::Monthday => Self::Number(context.date.day()),
                PlaceholderUnit::Month => {
                    Self::Month(Month::try_from(context.date.month() as u8).unwrap())
                }
                PlaceholderUnit::Year => Self::Number(context.date.year_ce().1),
                PlaceholderUnit::Date => Self::Date(context.date),
            }),
            Self::Predicate(e) => e.evaluate(context).map(Self::Boolean),
            Self::Arithmetic(e) => e.evaluate(context),
            _ => Ok(self.clone()),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Boolean(e) => e.to_string(),
                Self::Number(e) => e.to_string(),
                Self::Placeholder(e) => e.to_string(),
                Self::Weekday(e) => e.to_string().to_lowercase(),
                Self::Month(e) => {
                    let mut e = format!("{e:?}");
                    e.truncate(3);
                    e.make_ascii_lowercase();
                    e
                }
                Self::Date(e) => e.to_string(),
                Self::Predicate(e) => e.to_string(),
                Self::Arithmetic(e) => e.to_string(),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Predicate {
    Equality(Expression, Expression),
    And(Expression, Expression),
    Or(Expression, Expression),
    Not(Expression),
}

impl Evaluate<TypeResult<bool>> for Predicate {
    fn evaluate(&self, context: &Context) -> TypeResult<bool> {
        match self {
            Self::Equality(l, r) => Ok(l.evaluate(context)? == r.evaluate(context)?),
            Self::And(l, r) => {
                let l = l.evaluate(context)?;
                let r = r.evaluate(context)?;
                match (l, r) {
                    (Expression::Boolean(l), Expression::Boolean(r)) => Ok(l && r),
                    _ => Err(TypeError::Mismatch),
                }
            }
            Self::Or(l, r) => {
                let l = l.evaluate(context)?;
                let r = r.evaluate(context)?;
                match (l, r) {
                    (Expression::Boolean(l), Expression::Boolean(r)) => Ok(l || r),
                    _ => Err(TypeError::Mismatch),
                }
            }
            Self::Not(e) => {
                let e = e.evaluate(context)?;
                match e {
                    Expression::Boolean(e) => Ok(!e),
                    _ => Err(TypeError::Mismatch),
                }
            }
        }
    }
}

impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Equality(l, r) => format!("{l} = {r}"),
                Self::And(l, r) => format!("{l} & {r}"),
                Self::Or(l, r) => format!("{l} | {r}"),
                Self::Not(e) => format!("!{e}"),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Arithmetic {
    Modulo(Expression, Expression),
}

impl Evaluate<TypeResult<Expression>> for Arithmetic {
    fn evaluate(&self, context: &Context) -> TypeResult<Expression> {
        match self {
            Arithmetic::Modulo(l, r) => {
                let l = l.evaluate(context)?;
                let r = r.evaluate(context)?;
                match (l, r) {
                    (Expression::Number(l), Expression::Number(r)) => Ok(Expression::Number(l % r)),
                    _ => Err(TypeError::Mismatch),
                }
            }
        }
    }
}

impl fmt::Display for Arithmetic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Modulo(l, r) => format!("{l} % {r}"),
            }
        )
    }
}
