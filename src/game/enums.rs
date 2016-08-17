#[derive(Clone,Debug)]
pub struct DecodingError;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum GameVariant
{
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct GameMode
{
    pub variant: GameVariant,
    pub is_hardcore: bool,
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Dimension
{
    Nether = -1,
    Overworld = 0,
    End = 1,
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Difficulty
{
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

impl GameVariant
{
    pub fn encode(self) -> u8 { self as u8 }
    pub fn decode(val: u8) -> Result<Self, DecodingError> {
        match val {
            0 => Ok(GameVariant::Survival),
            1 => Ok(GameVariant::Creative),
            2 => Ok(GameVariant::Adventure),
            3 => Ok(GameVariant::Spectator),
            _ => Err(DecodingError),
        }
    }
}

impl GameMode
{
    pub fn encode(self) -> u8 {
        let hardcore_bit = if self.is_hardcore { 0b1000 } else { 0 };
        self.variant.encode() | hardcore_bit
    }

    pub fn decode(mut val: u8) -> Result<Self, DecodingError> {
        let is_hardcore = if val & 0b1000 == 0b1000 {
            // Remove the hardcore bit from the value.
            val &= !0b1000;
            true
        } else {
            false
        };

        if let Ok(variant) = GameVariant::decode(val) {
            Ok(GameMode { variant: variant, is_hardcore: is_hardcore })
        } else {
            Err(DecodingError)
        }
    }
}

impl Dimension
{
    pub fn encode(self) -> i32 { self as i32 }
    pub fn decode(val: i32) -> Result<Self, DecodingError> {
        match val {
            -1 => Ok(Dimension::Nether),
            0 => Ok(Dimension::Overworld),
            1 => Ok(Dimension::End),
            _ => Err(DecodingError),
        }
    }
}

impl Difficulty
{
    pub fn encode(self) -> u8 { self as u8 }
    pub fn decode(val: u8) -> Result<Self, DecodingError> {
        match val {
            0 => Ok(Difficulty::Peaceful),
            1 => Ok(Difficulty::Easy),
            2 => Ok(Difficulty::Normal),
            3 => Ok(Difficulty::Hard),
            _ => Err(DecodingError),
        }
    }
}

