
pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 600;

pub const PLAYER_WIDTH: f32 = 50.0;
pub const PLAYER_HEIGHT: f32 = 30.0;
/// Pixels per frame while moving
pub const PLAYER_MOVE_PER_FRAME: f32 = 5.0;

pub const BULLET_WIDTH: f32 = 4.0;
pub const BULLET_HEIGHT: f32 = 10.0;
/// Player bullet: pixels per frame upward
pub const PLAYER_BULLET_SPEED: f32 = 10.0;
/// Enemy bullet: pixels per frame downward
pub const ENEMY_BULLET_SPEED: f32 = 5.0;

pub const INVADER_ROWS: i32 = 5;
pub const INVADER_COLS: i32 = 11;
pub const INVADER_WIDTH: f32 = 40.0;
pub const INVADER_HEIGHT: f32 = 30.0;
pub const INVADER_START_X: f32 = 100.0;
pub const INVADER_START_Y: f32 = 30.0;
pub const INVADER_SPACING_X: f32 = 60.0;
pub const INVADER_SPACING_Y: f32 = 40.0;
/// Horizontal step in pixels when the fleet moves
pub const INVADER_SPEED: f32 = 5.0;
/// Frames between invader steps
pub const INVADER_MOVE_DELAY: i32 = 30;
pub const INVADER_DROP_DISTANCE: f32 = 20.0;

pub const MAX_BULLETS: usize = 10;
pub const MAX_ENEMY_BULLETS: usize = 20;
/// Frames between enemy shoot attempts
pub const ENEMY_SHOOT_DELAY: i32 = 60;
/// Chance out of 100 each attempt
pub const ENEMY_SHOOT_CHANCE: i32 = 5;

pub const SHIELD_COUNT: usize = 4;
pub const SHIELD_WIDTH: f32 = 80.0;
pub const SHIELD_HEIGHT: f32 = 60.0;
pub const SHIELD_START_X: f32 = 150.0;
pub const SHIELD_Y: f32 = 450.0;
pub const SHIELD_SPACING: f32 = 150.0;
pub const SHIELD_START_HEALTH: i32 = 10;

pub const SCORE_PER_INVADER: i32 = 10;
pub const PLAYER_SHOOT_COOLDOWN_FRAMES: i32 = 8;
