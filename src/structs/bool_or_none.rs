use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct BoolOrNone(pub Option<bool>);

impl Display for BoolOrNone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let r = if let Some(b) = self.0 {
            match b {
                true => "yes",
                false => "no",
            }
        } else {
            "?"
        };
        write!(f, "{}", r)
    }
}
