use crate::collision::rects_intersect;
use crate::entities::{Bullet, EnemyBullet, Invader, Player, Shield};
use crate::config::{
    BULLET_HEIGHT, BULLET_WIDTH, ENEMY_BULLET_SPEED, ENEMY_SHOOT_CHANCE, ENEMY_SHOOT_DELAY,
    INVADER_COLS, INVADER_DROP_DISTANCE, INVADER_HEIGHT, INVADER_MOVE_DELAY, INVADER_ROWS,
    INVADER_SPEED, INVADER_START_X, INVADER_START_Y, INVADER_SPACING_X, INVADER_SPACING_Y,
    INVADER_WIDTH, MAX_BULLETS, MAX_ENEMY_BULLETS, PLAYER_BULLET_SPEED, PLAYER_HEIGHT,
    PLAYER_MOVE_PER_FRAME, PLAYER_SHOOT_COOLDOWN_FRAMES, PLAYER_WIDTH, SCORE_PER_INVADER,
    SCREEN_HEIGHT, SCREEN_WIDTH, SHIELD_COUNT, SHIELD_HEIGHT, SHIELD_SPACING, SHIELD_START_HEALTH,
    SHIELD_START_X, SHIELD_WIDTH, SHIELD_Y,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Playing,
    GameOver,
    GameWon,
}

pub struct GameState {
    pub player: Player,
    pub bullets: Vec<Bullet>,
    pub enemy_bullets: Vec<EnemyBullet>,
    pub shields: Vec<Shield>,
    pub mode: GameMode,
    pub score: i32,
    pub invaders: Vec<Invader>,
    pub invader_direction: f32,
    pub invader_move_counter: i32,
    pub enemy_shoot_counter: i32,
    pub player_shoot_cooldown: i32,
}

fn build_invader_grid() -> Vec<Invader> {
    let mut invaders = Vec::new();
    for row in 0..INVADER_ROWS {
        for col in 0..INVADER_COLS {
            invaders.push(Invader {
                x: INVADER_START_X + col as f32 * INVADER_SPACING_X,
                y: INVADER_START_Y + row as f32 * INVADER_SPACING_Y,
                width: INVADER_WIDTH,
                height: INVADER_HEIGHT,
                active: true,
            });
        }
    }
    invaders
}

fn build_bullet_pool() -> Vec<Bullet> {
    let mut bullets = Vec::with_capacity(MAX_BULLETS);
    for _ in 0..MAX_BULLETS {
        bullets.push(Bullet {
            x: 0.0,
            y: 0.0,
            width: BULLET_WIDTH,
            height: BULLET_HEIGHT,
            active: false,
        });
    }
    bullets
}

fn build_enemy_bullet_pool() -> Vec<EnemyBullet> {
    let mut v = Vec::with_capacity(MAX_ENEMY_BULLETS);
    for _ in 0..MAX_ENEMY_BULLETS {
        v.push(EnemyBullet {
            x: 0.0,
            y: 0.0,
            width: BULLET_WIDTH,
            height: BULLET_HEIGHT,
            active: false,
        });
    }
    v
}

fn build_shields() -> Vec<Shield> {
    let mut shields = Vec::with_capacity(SHIELD_COUNT);
    for i in 0..SHIELD_COUNT {
        shields.push(Shield {
            x: SHIELD_START_X + i as f32 * SHIELD_SPACING,
            y: SHIELD_Y,
            width: SHIELD_WIDTH,
            height: SHIELD_HEIGHT,
            health: SHIELD_START_HEALTH,
        });
    }
    shields
}

impl GameState {
    pub fn new() -> Self {
        let py = SCREEN_HEIGHT as f32 - 60.0;
        Self {
            mode: GameMode::Playing,
            player: Player {
                x: SCREEN_WIDTH as f32 / 2.0 - PLAYER_WIDTH / 2.0,
                y: py,
                width: PLAYER_WIDTH,
                height: PLAYER_HEIGHT,
            },
            bullets: build_bullet_pool(),
            enemy_bullets: build_enemy_bullet_pool(),
            shields: build_shields(),
            invaders: build_invader_grid(),
            invader_direction: 1.0,
            invader_move_counter: 0,
            enemy_shoot_counter: 0,
            player_shoot_cooldown: 0,
            score: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// pixels per frame, not dt.
    pub fn update_player_movement(&mut self, move_dir: f32) {
        if move_dir > 0.0 {
            self.player.x += PLAYER_MOVE_PER_FRAME;
        } else if move_dir < 0.0 {
            self.player.x -= PLAYER_MOVE_PER_FRAME;
        }
        if self.player.x < 0.0 {
            self.player.x = 0.0;
        }
        if self.player.x + self.player.width > SCREEN_WIDTH as f32 {
            self.player.x = SCREEN_WIDTH as f32 - self.player.width;
        }
    }

    pub fn tick_player_shoot_cooldown(&mut self) {
        if self.player_shoot_cooldown > 0 {
            self.player_shoot_cooldown -= 1;
        }
    }

    pub fn try_spawn_player_bullet(&mut self, want_shoot: bool) {
        if !want_shoot || self.player_shoot_cooldown > 0 {
            return;
        }
        for bullet in &mut self.bullets {
            if !bullet.active {
                bullet.x = self.player.x + self.player.width / 2.0 - BULLET_WIDTH / 2.0;
                bullet.y = self.player.y;
                bullet.active = true;
                self.player_shoot_cooldown = PLAYER_SHOOT_COOLDOWN_FRAMES;
                break;
            }
        }
    }

    pub fn update_player_bullets(&mut self) {
        for bullet in &mut self.bullets {
            if bullet.active {
                bullet.y -= PLAYER_BULLET_SPEED;
                if bullet.y + bullet.height < 0.0 {
                    bullet.active = false;
                }
            }
        }
    }

    pub fn update_enemy_bullets(&mut self) {
        let h = SCREEN_HEIGHT as f32;
        for b in &mut self.enemy_bullets {
            if b.active {
                b.y += ENEMY_BULLET_SPEED;
                if b.y > h {
                    b.active = false;
                }
            }
        }
    }

    pub fn resolve_bullet_invader_collisions(&mut self) {
        for bi in 0..self.bullets.len() {
            if !self.bullets[bi].active {
                continue;
            }
            let b = &self.bullets[bi];
            let (bx, by, bw, bh) = (b.x, b.y, b.width, b.height);

            for ii in 0..self.invaders.len() {
                if !self.invaders[ii].active {
                    continue;
                }
                let inv = &self.invaders[ii];
                if rects_intersect(bx, by, bw, bh, inv.x, inv.y, inv.width, inv.height) {
                    self.bullets[bi].active = false;
                    self.invaders[ii].active = false;
                    self.score += SCORE_PER_INVADER;
                    break;
                }
            }
        }
    }

    pub fn resolve_player_bullet_shield_collisions(&mut self) {
        for bi in 0..self.bullets.len() {
            if !self.bullets[bi].active {
                continue;
            }
            let b = &self.bullets[bi];
            let (bx, by, bw, bh) = (b.x, b.y, b.width, b.height);
            for sh in &mut self.shields {
                if sh.health <= 0 {
                    continue;
                }
                if rects_intersect(bx, by, bw, bh, sh.x, sh.y, sh.width, sh.height) {
                    self.bullets[bi].active = false;
                    sh.health -= 1;
                    break;
                }
            }
        }
    }

    pub fn resolve_enemy_bullet_collisions(&mut self) {
        let px = self.player.x;
        let py = self.player.y;
        let pw = self.player.width;
        let ph = self.player.height;

        for eb in &mut self.enemy_bullets {
            if !eb.active {
                continue;
            }
            let (bx, by, bw, bh) = (eb.x, eb.y, eb.width, eb.height);
            if rects_intersect(bx, by, bw, bh, px, py, pw, ph) {
                eb.active = false;
                self.mode = GameMode::GameOver;
                return;
            }
            for sh in &mut self.shields {
                if sh.health <= 0 {
                    continue;
                }
                if rects_intersect(bx, by, bw, bh, sh.x, sh.y, sh.width, sh.height) {
                    eb.active = false;
                    sh.health -= 1;
                    break;
                }
            }
        }
    }

    /// increment each frame; on interval roll per invader with random 0..100.
    pub fn step_enemy_shooting(&mut self, mut random_0_100: impl FnMut() -> i32) {
        self.enemy_shoot_counter += 1;
        if self.enemy_shoot_counter < ENEMY_SHOOT_DELAY {
            return;
        }
        self.enemy_shoot_counter = 0;

        for ii in 0..self.invaders.len() {
            if !self.invaders[ii].active {
                continue;
            }
            if random_0_100() >= ENEMY_SHOOT_CHANCE {
                continue;
            }
            let inv = &self.invaders[ii];
            let ix = inv.x + inv.width / 2.0 - BULLET_WIDTH / 2.0;
            let iy = inv.y + inv.height;
            for b in &mut self.enemy_bullets {
                if !b.active {
                    b.x = ix;
                    b.y = iy;
                    b.active = true;
                    return;
                }
            }
            return;
        }
    }

    /// every INVADER_MOVE_DELAY frames, step by INVADER_SPEED or drop.
    pub fn step_invaders(&mut self) {
        self.invader_move_counter += 1;
        if self.invader_move_counter < INVADER_MOVE_DELAY {
            return;
        }
        self.invader_move_counter = 0;

        let sw = SCREEN_WIDTH as f32;
        let mut hit_edge = false;
        for inv in &self.invaders {
            if !inv.active {
                continue;
            }
            let next_x = inv.x + INVADER_SPEED * self.invader_direction;
            if next_x < 0.0 || next_x + inv.width > sw {
                hit_edge = true;
                break;
            }
        }

        if hit_edge {
            self.invader_direction *= -1.0;
            for inv in &mut self.invaders {
                if inv.active {
                    inv.y += INVADER_DROP_DISTANCE;
                }
            }
        } else {
            let dx = INVADER_SPEED * self.invader_direction;
            for inv in &mut self.invaders {
                if inv.active {
                    inv.x += dx;
                }
            }
        }
    }

    pub fn check_invader_player_collision(&mut self) {
        let p = &self.player;
        for inv in &self.invaders {
            if !inv.active {
                continue;
            }
            if rects_intersect(
                p.x,
                p.y,
                p.width,
                p.height,
                inv.x,
                inv.y,
                inv.width,
                inv.height,
            ) {
                self.mode = GameMode::GameOver;
                return;
            }
        }
    }

    pub fn check_all_invaders_destroyed(&mut self) {
        if self.mode != GameMode::Playing {
            return;
        }
        if self.invaders.iter().all(|i| !i.active) {
            self.mode = GameMode::GameWon;
        }
    }

}
