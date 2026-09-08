use std::fmt;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum Game {
    THPS4,
    THUG,
    THUG2,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Game::THPS4 => "THPS4",
                Game::THUG => "THUG",
                Game::THUG2 => "THUG2",
            }
        )
    }
}
