use std::fmt;
use std::str::FromStr;

pub enum Position {
    C,
    RW,
    LW,
    D,
    G
}

pub enum PositionType {
    Forward,
    Defenseman,
    Goalie
}

pub struct Player {
    pub name: String,
    pub number: u64,
    pub id: u64,
    pub position: Position,
    pub position_type: PositionType
}

impl FromStr for Position {
    // TODO Look up what this does
    type Err = ();

    fn from_str(input: &str) -> Result<Position, Self::Err> {
        match input {
            "C" => Ok(Position::C),
            "RW" => Ok(Position::RW),
            "LW" => Ok(Position::LW),
            "D" => Ok(Position::D),
            "G" => Ok(Position::G),
            _ => Err(())
        }
    }
}

impl FromStr for PositionType {
    type Err = ();

    fn from_str(input: &str) -> Result<PositionType, Self::Err> {
        match input {
            "Forward" => Ok(PositionType::Forward),
            "Defenseman" => Ok(PositionType::Defenseman),
            "Goalie" => Ok(PositionType::Goalie),
            _ => Err(())
        }
    }
}

impl ToString for Position {
    fn to_string(&self) -> String {
        match self {
            Position::C => "C".to_string(),
            Position::RW => "RW".to_string(),
            Position::LW => "LW".to_string(),
            Position::D => "D".to_string(),
            Position::G => "G".to_string(),
        }
    }
}

impl ToString for PositionType {
    fn to_string(&self) -> String {
        match self {
            PositionType::Forward => "Forward".to_string(),
            PositionType::Defenseman => "Defenseman".to_string(),
            PositionType::Goalie => "Goalie".to_string(),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pos = self.position.to_string();
        let pos_type = self.position_type.to_string();
        // No semicolon so it's being reutrned
        write!(f, "({} #{}, {}, {} [{}])", self.name, self.number, pos, pos_type, self.id)
    }
}