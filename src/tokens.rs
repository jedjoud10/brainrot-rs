use strum_macros::EnumIter;

pub enum Value {
    Int(u32),
    Float(f32),
    None,
}

#[derive(EnumIter, Debug, Clone, Copy)]
#[repr(u32)]
pub enum Register {
    Zawg,
    Blud,

    Vro,
    Cro,
    Zro,
    Tro,


    // Cat reference??
    //Mohsin,
    //Belahsoun,
}

#[derive(EnumIter,Debug, Clone, Copy)]
pub enum Type {
    Int,
    Float
}

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum ParsedType {
    Int(i32),
    Float(f32)
}

impl Default for ParsedType {
    fn default() -> Self {
        Self::Float(0f32)
    }
}

impl ParsedType {
    pub fn of_type(self, _type: Type) -> bool {
        match (_type, self) {
            (Type::Int, ParsedType::Int(_)) => true,
            (Type::Float, ParsedType::Float(_)) => true,
            _ => false,
        }
    }
}

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum Moment {
    Instant,
    Later
}

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum Slop {
    Be,
    This,
    To,
    On,
}

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum IoDirection {
    Listen, 
    Talk,
}

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum Marker {
    LineEnd,
    LineStart,
    FileEnd,
}

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum Special {
    Comma,
    Colon,
    Question,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Assignment,
    Register(Register),
    WriteBitwise,
    BitwiseRegister(usize),
    Type(Type),
    Slop(Slop),
    Moment(Moment),
    Comment,
    Special(Special),
    Parsed(ParsedType),
    Marker(Marker),
    Io,
    Debug,
    Conditional,
    LessThanEqualCheck,
    IoDirection(IoDirection),
    Unrecognized,
}

pub struct TokenExtra {
    pub line: u32,
}