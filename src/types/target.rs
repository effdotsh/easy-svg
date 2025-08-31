use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Target {
    _Self,
    _Parent,
    _Top,
    _Blank,
    Custom(String),
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            Target::_Self => "_self",
            Target::_Parent => "_parent",
            Target::_Top => "_top",
            Target::_Blank => "_blank",
            Target::Custom(str) => str,
        };
        write!(f, "{}", str)
    }
}
