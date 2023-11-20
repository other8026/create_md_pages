use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StringOrNone(pub Option<String>);

impl StringOrNone {
    pub fn replace_double_quotes_with_single_quotes(&self) -> Self {
        if let Self(Some(text)) = self {
            Self(Some(text.replace("\"", "'")))
        } else {
            Self(None)
        }
    }

    pub fn replace_new_lines_with_p_tags(&self) -> Self {
        if let Self(Some(text)) = self {
            let text = text.trim();

            Self(Some(
                text.split("\n")
                    .map(|s| format!("<p>{}</p>", s))
                    .collect::<Vec<String>>()
                    .join(""),
            ))
        } else {
            Self(None)
        }
    }
}

impl Display for StringOrNone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(r) = &self.0 {
            write!(f, "{}", r)
        } else {
            write!(f, "")
        }
    }
}
