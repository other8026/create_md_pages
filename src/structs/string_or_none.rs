use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StringOrNone(pub Option<String>);

impl Display for StringOrNone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(r) = &self.0 {
            write!(f, "{}", r)
        } else {
            write!(f, "")
        }
    }
}