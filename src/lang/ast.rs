use chrono::{Datelike, Duration, Month, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::*;

pub enum Type {
    Number,
    Placeholder,
    Weekday,
    Month,
    Date,
}

#[derive(Debug)]
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
    Function(Box<Function>),
}

impl Expression {
    pub fn as_predicate(self) -> Option<Predicate> {
        match self {
            Self::Predicate(p) => Some(*p),
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
            Self::Function(e) => e.evaluate(context),
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
                Self::Function(e) => e.to_string(),
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
    Comparison {
        greater_than: bool,
        or_equal: bool,
        left: Expression,
        right: Expression,
    },
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
            Self::Comparison {
                greater_than,
                or_equal,
                left,
                right,
            } => {
                let left = left.evaluate(context)?;
                let right = right.evaluate(context)?;

                match (left, right) {
                    (Expression::Number(l), Expression::Number(r)) => {
                        let c = if *greater_than { l > r } else { l < r };
                        Ok(c || or_equal.then_some(l == r).unwrap_or(false))
                    }
                    (Expression::Date(l), Expression::Date(r)) => {
                        let c = if *greater_than { l > r } else { l < r };
                        Ok(c || or_equal.then_some(l == r).unwrap_or(false))
                    }
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
                Self::Comparison {
                    greater_than,
                    or_equal,
                    left,
                    right,
                } => format!(
                    "{left} {}{} {right}",
                    if *greater_than { ">" } else { "<" },
                    or_equal.then_some("=").unwrap_or_default()
                ),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Arithmetic {
    Addition(Expression, Expression),
    Modulo(Expression, Expression),
}

impl Evaluate<TypeResult<Expression>> for Arithmetic {
    fn evaluate(&self, context: &Context) -> TypeResult<Expression> {
        match self {
            Self::Addition(l, r) => {
                let l = l.evaluate(context)?;
                let r = r.evaluate(context)?;
                match (l, r) {
                    (Expression::Date(date), Expression::Number(days))
                    | (Expression::Number(days), Expression::Date(date)) => {
                        Ok(Expression::Date(date + Duration::days(days as i64)))
                    }
                    _ => Err(TypeError::Mismatch),
                }
            }
            Self::Modulo(l, r) => {
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
                Self::Addition(l, r) => format!("{l} + {r}"),
                Self::Modulo(l, r) => format!("{l} % {r}"),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Function {
    Monthday(Expression),
    Weekday(Expression),
    WeekdayPredecessor {
        weekday: Expression,
        date: Expression,
    },
    NumberOfWeeks {
        low: Expression,
        high: Expression,
    },
}

impl Evaluate<TypeResult<Expression>> for Function {
    fn evaluate(&self, context: &Context) -> TypeResult<Expression> {
        match self {
            Self::Monthday(e) => match e.evaluate(context)? {
                Expression::Date(d) => Ok(Expression::Number(d.day())),
                _ => Err(TypeError::Mismatch),
            },
            Self::Weekday(e) => match e.evaluate(context)? {
                Expression::Date(d) => Ok(Expression::Weekday(d.weekday())),
                _ => Err(TypeError::Mismatch),
            },
            Self::WeekdayPredecessor { weekday, date } => {
                let weekday = weekday.evaluate(context)?;
                let date = date.evaluate(context)?;

                match (weekday, date) {
                    (Expression::Weekday(wd), Expression::Date(date)) => {
                        let result =
                            date - Duration::days(match context.date.weekday() {
                                Weekday::Mon => 1,
                                Weekday::Tue => 2,
                                Weekday::Wed => 3,
                                Weekday::Thu => 4,
                                Weekday::Fri => 5,
                                Weekday::Sat => 6,
                                Weekday::Sun => 0,
                            }) - Duration::days(6)
                                + Duration::days(wd as i64);
                        Ok(Expression::Date(result))
                    }
                    _ => Err(TypeError::Mismatch),
                }
            }
            Self::NumberOfWeeks { low, high } => {
                let low = low.evaluate(context)?;
                let high = high.evaluate(context)?;

                match (low, high) {
                    (Expression::Date(low), Expression::Date(high)) => {
                        let low = low.min(high);
                        let high = low.max(high);
                        let result = (high - low).num_weeks() as u32;
                        Ok(Expression::Number(result as u32))
                    }
                    _ => Err(TypeError::Mismatch),
                }
            }
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Monthday(e) => format!("md({e})"),
                Self::Weekday(e) => format!("wd({e})"),
                Self::WeekdayPredecessor { weekday, date } => format!("wdp({weekday}, {date})"),
                Self::NumberOfWeeks { low, high } => format!("nw({low}, {high})"),
            }
        )
    }
}
