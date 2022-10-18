use std::fmt;

// Now use crate::player to tell it to  look for the player object locally
use crate::player;

pub enum Division {
    Atlantic,
    Metropolitan,
    Central,
    Pacific
}

pub const NUM_TEAMS:usize = 32;

pub struct Team {
    pub name: String,
    pub abbrev: String,
    pub division: Division,
    pub id: usize,
    pub players: Vec<player::Player>
}

pub fn division_from_string(division_name: &str) -> Result<Division, String> {
    match division_name{
        "Atlantic" => Ok(Division::Atlantic),
        "Metropolitan" => Ok(Division::Metropolitan),
        "Central" => Ok(Division::Central),
        "Pacific" => Ok(Division::Pacific),
        _ => Err("INVALID DIVISION NAME".to_string())
    }
}

impl Default for Team {
    fn default() -> Self {
        Team {
            name: "N/A".to_string(),
            abbrev: "N/A".to_string(),
            division: Division::Atlantic,
            id: 0,
            players: Vec::new()
        }
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let div: String = match self.division {
            Division::Atlantic => "Atlantic".to_string(),
            Division::Metropolitan => "Metropolitan".to_string(),
            Division::Central => "Central".to_string(),
            Division::Pacific => "Pacific".to_string(),
        };
        // No semicolon so it's being reutrned
        write!(f, "({},{},{})", self.name, self.abbrev, div)
    }
}