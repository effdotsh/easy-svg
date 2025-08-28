#[derive(Debug, Clone)]
pub enum LineCap {
    /* keyword values */
    Butt,
    Round,
    Square,

    /* Global values */
    Inherit,
    Initial,
    Revert,
    RevertLayer,
    Unset,
}

impl From<LineCap> for String {
    fn from(line_cap: LineCap) -> String {
        match line_cap {
            LineCap::Butt => "butt".to_string(),
            LineCap::Round => "round".to_string(),
            LineCap::Square => "square".to_string(),
            LineCap::Inherit => "inherit".to_string(),
            LineCap::Initial => "initial".to_string(),
            LineCap::Revert => "revert".to_string(),
            LineCap::RevertLayer => "revert-layer".to_string(),
            LineCap::Unset => "unset".to_string(),
        }
    }
}
