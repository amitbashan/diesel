use std::fmt;

use chrono::{Datelike, Month, NaiveDate, Weekday};

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

#[derive(Copy, Clone, PartialEq, Eq)]
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Expression {
    Number(u32),
    Placeholder(PlaceholderUnit),
    Weekday(Weekday),
    Month(Month),
    Date(NaiveDate),
}

impl Evaluate<Expression> for Expression {
    fn evaluate(&self, context: &Context) -> Self {
        match self {
            Self::Placeholder(ph) => match ph {
                PlaceholderUnit::Weekday => Self::Weekday(context.date.weekday()),
                PlaceholderUnit::Monthday => Self::Number(context.date.day()),
                PlaceholderUnit::Month => {
                    Self::Month(Month::try_from(context.date.month() as u8).unwrap())
                }
                PlaceholderUnit::Year => Self::Number(context.date.year_ce().1),
                PlaceholderUnit::Date => Self::Date(context.date),
            },
            _ => self.clone(),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
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
            }
        )
    }
}

#[derive(Copy, Clone)]
pub enum Predicate {
    Equality(Expression, Expression),
}

impl Evaluate<bool> for Predicate {
    fn evaluate(&self, context: &Context) -> bool {
        match self {
            Self::Equality(l, r) => l.evaluate(context) == r.evaluate(context),
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
            }
        )
    }
}
