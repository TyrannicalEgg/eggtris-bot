pub const BOARD_WIDTH: u16 = 10;
pub const BOARD_HIGHT: u16 = 21;
pub const GARBAGE_MESSINESS: f32 = 0.05;
pub const ATTACK_TABLE: [(&'static str, usize); 9] = 
[
    ("single", 0),
    ("double", 1),
    ("triple", 2),
    ("quad", 4),
    ("asd", 4),
    ("ass", 4),
    ("ast", 6),
    ("pc", 10),
    ("b2b", 1),
];
pub const COMBO_TABLE: [u8; 10] = 
    [0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
