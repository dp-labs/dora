/// Represents different levels of optimization.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OptimizationLevel {
    /// No optimizations.
    None,
    /// Less optimizations.
    Less,
    /// Default optimizations.
    #[default]
    Default,
    /// Aggressive optimizations.
    Aggressive,
}

impl std::str::FromStr for OptimizationLevel {
    type Err = String;

    /// Parses a string into an `OptimizationLevel`.
    ///
    /// Accepts either numerical representations ("0", "1", "2", "3") or named levels
    /// ("none", "less", "default", "aggressive"). Returns an error if the input does not match.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | "none" => Ok(Self::None),
            "1" | "less" => Ok(Self::Less),
            "2" | "default" => Ok(Self::Default),
            "3" | "aggressive" => Ok(Self::Aggressive),
            _ => Err(format!("unknown optimization level: {s}")),
        }
    }
}
