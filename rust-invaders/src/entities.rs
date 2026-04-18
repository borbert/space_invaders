#[derive(Debug, Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct EnemyBullet {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct Invader {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct Shield {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub health: i32,
}
