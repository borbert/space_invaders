mod collision;
mod config;
mod entities;
mod state;

use raylib::prelude::*;
use state::GameMode;
use state::GameState;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(config::SCREEN_WIDTH, config::SCREEN_HEIGHT)
        .title("Rust Invaders")
        .build();

    rl.set_target_fps(60);

    let mut game = GameState::new();

    while !rl.window_should_close() {
        let shoot = rl.is_key_pressed(KeyboardKey::KEY_SPACE)
            || rl.is_key_down(KeyboardKey::KEY_SPACE);

        let mut move_dir = 0.0;
        if rl.is_key_down(KeyboardKey::KEY_RIGHT)
            || rl.is_key_down(KeyboardKey::KEY_D)
            || rl.is_key_down(KeyboardKey::KEY_L)
        {
            move_dir += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT)
            || rl.is_key_down(KeyboardKey::KEY_A)
            || rl.is_key_down(KeyboardKey::KEY_H)
        {
            move_dir -= 1.0;
        }

        // --- overlays--
        if game.mode == GameMode::GameOver || game.mode == GameMode::GameWon {
            if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                game.reset();
            }

            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            match game.mode {
                GameMode::GameOver => {
                    d.draw_text("Game Over", 270, 250, 40, Color::RED);
                }
                GameMode::GameWon => {
                    d.draw_text("You Won!", 320, 250, 40, Color::GOLD);
                }
                GameMode::Playing => {}
            }
            let score_line = format!("Score: {}", game.score);
            d.draw_text(&score_line, 285, 310, 30, Color::WHITE);
            d.draw_text(
                "Press Enter to Restart, ESC to quit",
                180,
                360,
                20,
                Color::WHITE,
            );
            continue;
        }

        // --- playing: ---
        game.tick_player_shoot_cooldown();
        game.update_player_movement(move_dir);
        game.try_spawn_player_bullet(shoot);

        game.update_player_bullets();
        game.update_enemy_bullets();

        game.resolve_bullet_invader_collisions();
        game.resolve_player_bullet_shield_collisions();
        game.resolve_enemy_bullet_collisions();

        game.step_enemy_shooting(|| rl.get_random_value(0..100));

        game.step_invaders();

        game.check_invader_player_collision();
        game.check_all_invaders_destroyed();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for sh in &game.shields {
            if sh.health > 0 {
                let alpha: u8 = (sh.health * 25).min(255) as u8;
                d.draw_rectangle(
                    sh.x as i32,
                    sh.y as i32,
                    sh.width as i32,
                    sh.height as i32,
                    Color::new(0, 255, 255, alpha),
                );
            }
        }

        d.draw_rectangle(
            game.player.x as i32,
            game.player.y as i32,
            game.player.width as i32,
            game.player.height as i32,
            Color::BLUE,
        );

        for bullet in &game.bullets {
            if bullet.active {
                d.draw_rectangle(
                    bullet.x as i32,
                    bullet.y as i32,
                    bullet.width as i32,
                    bullet.height as i32,
                    Color::RED,
                );
            }
        }

        for invader in &game.invaders {
            if invader.active {
                d.draw_rectangle(
                    invader.x as i32,
                    invader.y as i32,
                    invader.width as i32,
                    invader.height as i32,
                    Color::GREEN,
                );
            }
        }

        for eb in &game.enemy_bullets {
            if eb.active {
                d.draw_rectangle(
                    eb.x as i32,
                    eb.y as i32,
                    eb.width as i32,
                    eb.height as i32,
                    Color::YELLOW,
                );
            }
        }

        let score_text = format!("Score: {}", game.score);
        d.draw_text(&score_text, 20, 20, 20, Color::WHITE);
        d.draw_text("SPACE to shoot — Enter restarts after game ends", 20, 40, 20, Color::WHITE);
    }
}
