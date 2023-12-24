use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(u8)]
pub enum WidgetSize {
    Small,
    Medium,
    Large,
}

impl WidgetSize {
    pub const fn size_class(&self) -> &str {
        match self {
            Self::Small => "w-20 h-20",
            Self::Medium => "w-40 h-20",
            Self::Large => "w-40 h-40",
        }
    }

    pub const fn class(&self) -> &str {
        match self {
            Self::Small => "row-span-1 col-span-1",
            Self::Medium => "row-span-1 col-span-2",
            Self::Large => "row-span-2 col-span-2",
        }
    }

    pub const fn row_span(&self) -> usize {
        match self {
            Self::Small => 1,
            Self::Medium => 1,
            Self::Large => 2,
        }
    }

    pub const fn col_span(&self) -> usize {
        match self {
            Self::Small => 1,
            Self::Medium => 2,
            Self::Large => 2,
        }
    }
}

impl From<usize> for WidgetSize {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Small,
            1 => Self::Medium,
            2 => Self::Large,
            _ => unreachable!(),
        }
    }
}
