use crate::resources::SubPiece;


pub const SUB_SPEED: f32 = 0.25;
pub const ROTATION_SPEED: f32 = 0.025;

pub const SUB_STRUCTURE_SPACING: f32 = 3.5;
pub const SUB_SCALE: f32 = 0.2;
pub const SUB_MAX_WIDTH: usize = 100;
pub const SUB_MAX_HEIGHT: usize = 100;

pub const TEXT_FONT_SIZE: f32 = 30.0;
pub const EMPTY_CHAR: char = '\0';

//armour
pub const ARMOUR_LABEL: &'static str = "Hull Panel";
pub const ARMOUR_DESC: &'static str = "Submarine construction part. Suitable for pressurised environments.";

pub const SQUARE_OPEN: char = '[';
pub const SQUARE_CLOSE: char = ']';

pub static ARMOUR1: SubPiece = SubPiece {
    symbol: SQUARE_OPEN,
    label: ARMOUR_LABEL,
    description: ARMOUR_DESC
};

pub static ARMOUR2: SubPiece = SubPiece {
    symbol: SQUARE_CLOSE,
    label: ARMOUR_LABEL,
    description: ARMOUR_DESC
};

pub const ROUND_OPEN: char = '(';
pub const ROUND_CLOSE: char = ')';

pub static ARMOUR3: SubPiece = SubPiece {
    symbol: ROUND_OPEN,
    label: ARMOUR_LABEL,
    description: ARMOUR_DESC
};

pub static ARMOUR4: SubPiece = SubPiece {
    symbol: ROUND_CLOSE,
    label: ARMOUR_LABEL,
    description: ARMOUR_DESC
};

pub const CURLY_OPEN: char = '{';
pub const CURLY_CLOSE: char = '}';

pub static ARMOUR5: SubPiece = SubPiece {
    symbol: CURLY_OPEN,
    label: ARMOUR_LABEL,
    description: ARMOUR_DESC
};

pub static ARMOUR6: SubPiece = SubPiece {
    symbol: CURLY_CLOSE,
    label: ARMOUR_LABEL,
    description: ARMOUR_DESC
};

pub const BACKSLASH: char = '\\';
pub const FORWARD_SLASH: char = '/';
pub const PIPE: char = '|';

pub static ARMOUR7: SubPiece = SubPiece {
    symbol: BACKSLASH,
    label: ARMOUR_LABEL,
    description: ARMOUR_DESC
};

pub static ARMOUR8: SubPiece = SubPiece {
    symbol: FORWARD_SLASH,
    label: ARMOUR_LABEL,
    description: ARMOUR_DESC
};

pub static ARMOUR9: SubPiece = SubPiece {
    symbol: PIPE,
    label: ARMOUR_LABEL,
    description: ARMOUR_DESC
};

//utilities
pub static PROPELLER_LABEL: &'static str = "Propeller";
pub static PROPELLER_DESCRIPTION: &'static str = "Used for propulsion in the watery depths.";
pub const PLUS: char = '+';
pub const MINUS: char = '-';

pub static PROPELLER1: SubPiece = SubPiece {
    symbol: PLUS,
    label: PROPELLER_LABEL,
    description: PROPELLER_DESCRIPTION
};

pub static PROPELLER2: SubPiece = SubPiece {
    symbol: MINUS,
    label: PROPELLER_LABEL,
    description: PROPELLER_DESCRIPTION
};

pub static TORPEDO_LABEL: &'static str = "Torpedo";
pub static TORPEDO_DESC: &'static str = "Explodey thing";
pub const EXCLAMATION: char = '!';

pub static TORPEDO: SubPiece = SubPiece {
    symbol: EXCLAMATION,
    label: TORPEDO_LABEL,
    description: TORPEDO_DESC
};

pub static TORPEDO_LAUNCHER_LABEL: &'static str = "Torpedo Launcher";
pub static TORPEDO_LAUNCHER_DESC: &'static str = "Throws explodey things.";
pub const EQUALS: char = '=';

pub static TORPEDO_LAUNCHER: SubPiece = SubPiece {
    symbol: EQUALS,
    label: TORPEDO_LAUNCHER_LABEL,
    description: TORPEDO_LAUNCHER_DESC
};

pub static REACTOR_LABEL: &'static str = "Reactor";
pub static REACTOR_DESC: &'static str = "Generates power using unholy science.";
pub const AT: char = '@';

pub static REACTOR: SubPiece = SubPiece {
    symbol: AT,
    label: REACTOR_LABEL,
    description: REACTOR_DESC
};

//debris
pub const TILDE: char = '~';
pub const FULLSTOP: char = '.';
pub const BUBBLE: char = 'o';

pub static SUB_PARTS: [&SubPiece; 12] = [
    &ARMOUR1, &ARMOUR2, &ARMOUR3, &ARMOUR4, &ARMOUR5, &ARMOUR6, &ARMOUR7, &ARMOUR8, &ARMOUR9,
    &PROPELLER1, &TORPEDO_LAUNCHER, &REACTOR
];

pub const TEAM_KEY: &'static str = "amogus";
pub const GAME_NAME: &'static str = "Text Sub";
pub const CACHED_KEY: &'static str = "saved_build";
