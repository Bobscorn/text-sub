use crate::resources::SubPiece;


pub const SUB_SPEED: f32 = 0.25;
pub const ROTATION_SPEED: f32 = 0.025;

pub const SUB_STRUCTURE_SPACING: f32 = 3.5;
pub const SUB_SCALE: f32 = 0.2;
pub const SUB_MAX_WIDTH: usize = 50;
pub const SUB_MAX_HEIGHT: usize = 40;

pub const TEXT_FONT_SIZE: f32 = 30.0;
pub const EMPTY_CHAR: char = '\0';

//armour
pub const ARMOUR_LABEL: &'static str = "Hull Panel";
pub const LIGHT_ARMOUR_DESC: &'static str = "Submarine construction part. Suitable for pressurised environments.";
pub const LIGHT_ARMOUR_DESCRIPTION: &'static str = "";
pub const SQUARE_OPEN: char = '[';
pub const SQUARE_CLOSE: char = ']';

pub static LIGHT_ARMOUR1: SubPiece = SubPiece {
    symbol: SQUARE_OPEN,
    label: String::from("Light Armour"),
    description: String::from("")
};

pub static LIGHT_ARMOUR2: SubPiece = SubPiece {
    symbol: SQUARE_CLOSE,
    label: String::from(""),
    description: String::from("")
};


pub const ANGLE_OPEN: char = '<';
pub const ANGLE_CLOSE: char = '>';
pub const ROUND_OPEN: char = '(';
pub const ROUND_CLOSE: char = ')';
pub const CURLY_OPEN: char = '{';
pub const CURL_CLOSE: char = '}';

pub const BACKSLASH: char = '\\';
pub const FORWARD_SLASH: char = '/';
pub const PIPE: char = '|';
pub const EQUALS: char = '=';
pub const PERCENT: char = '%';
pub const TILDE: char = '~';

pub const FULLSTOP: char = '.';
pub const PLUS: char = '+';
pub const MINUS: char = '-';
pub const BACKTICK: char = '`';

pub const STAR: char = '*';
pub const EXCLAMATION: char = '!';
pub const AT: char = '@';
pub const DOLLAR: char = '$';
pub const QUESTION: char = '?';
pub const APOSTROPHE: char = '\'';
pub const COLON: char = ':';

pub const SUB_PIECES: [char; 25] = [
    SQUARE_OPEN, SQUARE_CLOSE, ANGLE_OPEN, ANGLE_CLOSE, 
    ROUND_OPEN, ROUND_CLOSE, CURLY_OPEN, CURL_CLOSE, 
    BACKSLASH, FORWARD_SLASH, PIPE, EQUALS, PERCENT, TILDE,
    FULLSTOP, PLUS, MINUS, BACKTICK, STAR, EXCLAMATION,
    AT, DOLLAR, QUESTION, APOSTROPHE, COLON
    ];
