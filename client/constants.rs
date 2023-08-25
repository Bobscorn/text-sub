
pub const MOTHERsub_SPEED: f32 = 0.25;
pub const ROTATION_SPEED: f32 = 0.025;

pub const MOTHERsub_STRUCTURE_SPACING: f32 = 3.5;
pub const MOTHERsub_SCALE: f32 = 0.2;
pub const MOTHERsub_MAX_WIDTH: usize = 50;
pub const MOTHERsub_MAX_HEIGHT: usize = 40;

pub const TEXT_FONT_SIZE: f32 = 30.0;

pub const EMPTY_CHAR: char = '\0';
pub const SQUARE_OPEN: char = '[';
pub const SQUARE_CLOSE: char = ']';
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

pub const sub_PIECES: [char; 25] = [
    SQUARE_OPEN, SQUARE_CLOSE, ANGLE_OPEN, ANGLE_CLOSE, 
    ROUND_OPEN, ROUND_CLOSE, CURLY_OPEN, CURL_CLOSE, 
    BACKSLASH, FORWARD_SLASH, PIPE, EQUALS, PERCENT, TILDE,
    FULLSTOP, PLUS, MINUS, BACKTICK, STAR, EXCLAMATION,
    AT, DOLLAR, QUESTION, APOSTROPHE, COLON
    ];
