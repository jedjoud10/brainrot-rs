pub enum Value {
    Int(u32),
    Float(f32),
    None,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Int,
    Float
}

#[derive(Debug, Clone, Copy)]
pub enum ParsedType {
    Int(i32),
    Float(f32)
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

#[derive(Debug, Clone, Copy)]
pub enum Moment {
    Instant,
    Later
}

#[derive(Debug, Clone, Copy)]
pub enum Slop {
    Be,
    Is,
    This,
    To,
}

#[derive(Debug, Clone, Copy)]
pub enum IoDirection {
    Listen, 
    Talk,
}

#[derive(Debug, Clone, Copy)]
pub enum Marker {
    LineEnd,
    LineStart,
    FileEnd,
}

#[derive(Debug, Clone, Copy)]
pub enum Special {
    Comma,
    Colon,
    Question,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Assignment,
    Register(Register),
    Type(Type),
    Slop(Slop),
    Moment(Moment),
    Comment,
    Special(Special),
    Parsed(ParsedType),
    Marker(Marker),
    Io,
    IoDirection(IoDirection),
}