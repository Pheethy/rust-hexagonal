use anyhow::Result;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage {
    Local,
    Development,
    Production,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stage::Local => write!(f, "Local"),
            Stage::Development => write!(f, "Development"),
            Stage::Production => write!(f, "Production"),
        }
    }
}

impl Stage {
    pub fn try_form(stage: &str) -> Result<Self> {
        match stage {
            "Local" => Ok(Stage::Local),
            "Development" => Ok(Stage::Development),
            "Production" => Ok(Stage::Production),
            _ => Err(anyhow::anyhow!("Invalid stage")),
        }
    }
}
