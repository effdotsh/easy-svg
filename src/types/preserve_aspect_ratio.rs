use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreserveAspectRatio {
    None,
    XMinYMin,
    XMidYMin,
    XMaxYMin,
    XMinYMid,
    XMidYMid,
    XMaxYMid,
    XMinYMax,
    XMidYMax,
    XMaxYMax,
}

impl Display for PreserveAspectRatio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PreserveAspectRatio::None => "None".to_string(),
                PreserveAspectRatio::XMinYMin => "xMinYMin".to_string(),
                PreserveAspectRatio::XMidYMin => "xMidYMin".to_string(),
                PreserveAspectRatio::XMaxYMin => "xMaxYMin".to_string(),

                PreserveAspectRatio::XMinYMid => "xMinYMid".to_string(),
                PreserveAspectRatio::XMidYMid => "xMidYMid".to_string(),
                PreserveAspectRatio::XMaxYMid => "xMaxYMid".to_string(),

                PreserveAspectRatio::XMinYMax => "xMinYMax".to_string(),
                PreserveAspectRatio::XMidYMax => "xMidYMax".to_string(),
                PreserveAspectRatio::XMaxYMax => "xMaxYMax".to_string(),
            }
        )
    }
}
