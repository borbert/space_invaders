package main

import (
	"fmt"
	rl "github.com/gen2brain/raylib-go/raylib"
)

func resetGame(
	player *Player,
	bullets []Bullet,
	enemyBullets []EnemyBullet,
	shields []Shield,
	invaders *[invaderRows][invaderCols]Invader,
	invaderDirection *float32,
	score *int32,
	cfg GameConfig,
) {
	*score = 0
	*player = NewPlayer(
		float32(cfg.ScreenWidth)/2.0-cfg.PlayerWidth/2.0,
		cfg.PlayerStartY,
		cfg.PlayerWidth,
		cfg.PlayerHeight,
	)
	for i := range bullets {
		bullets[i].Active = false
	}
	for i := range enemyBullets {
		enemyBullets[i].Active = false
	}
	for i := range shields {
		x := cfg.SheildStartX + float32(i)*cfg.ShieldSpacing
		shields[i] = NewShield(x, cfg.ShieldY, cfg.ShieldWidth, cfg.ShieldHeight)
	}
	for i := range invaders {
		for j := range invaders[i] {
			x := cfg.InvaderStartX + float32(j)*cfg.InvaderSpacingX
			y := cfg.InvaderStartY + float32(i)*cfg.InvaderSpacingY
			if y < 0 {
				y = 0
			}
			invaders[i][j] = NewInvader(x, y, cfg.InvaderWidth, cfg.InvaderHeight)
		}
	}
	*invaderDirection = 1.0
}

func main() {
	cfg := defaultGameConfig()

	rl.InitWindow(cfg.ScreenWidth, cfg.ScreenHeight, "Go Invaders")
	defer rl.CloseWindow()
	rl.SetTargetFPS(60)

	player := NewPlayer(
		float32(cfg.ScreenWidth)/2.0-cfg.PlayerWidth/2.0,
		cfg.PlayerStartY,
		cfg.PlayerWidth,
		cfg.PlayerHeight,
	)
	var bullets [maxBullets]Bullet
	for i := range bullets {
		bullets[i] = NewBullet(0, 0, cfg.BulletWidth, cfg.BulletHeight)
	}
	var enemyBullets [maxEnemyBullets]EnemyBullet
	for i := range enemyBullets {
		enemyBullets[i] = NewEnemyBullet(0, 0, cfg.BulletWidth, cfg.BulletHeight)
	}
	var shields [shieldCount]Shield
	for i := range shields {
		x := cfg.SheildStartX + float32(i)*cfg.ShieldSpacing
		shields[i] = NewShield(x, cfg.ShieldY, cfg.ShieldWidth, cfg.ShieldHeight)
	}
	var invaders [invaderRows][invaderCols]Invader
	for i := range invaders {
		for j := range invaders[i] {
			x := cfg.InvaderStartX + float32(j)*cfg.InvaderSpacingX
			y := cfg.InvaderStartY + float32(i)*cfg.InvaderSpacingY
			if y < 0 {
				y = 0
			}
			if x < 0 {
				x = 0
			}
			invaders[i][j] = NewInvader(x, y, cfg.InvaderWidth, cfg.InvaderHeight)
		}
	}

	var (
		invaderDirection float32 = 1.0
		moveTimer        int32
		enemyShootTimer  int32
		playerShootTimer int32
		score            int32
		gameOver         bool
		gameWon          bool
	)

	for !rl.WindowShouldClose() {
		rl.BeginDrawing()
		rl.ClearBackground(rl.Black)

		// Game over / win screen
		if gameOver {
			rl.DrawText("Game Over", 270, 250, 40, rl.Red)
			rl.DrawText(fmt.Sprintf("Score: %d", score), 285, 310, 30, rl.White)
			rl.DrawText("Press Enter to Restart, ESC to quit", 180, 360, 20, rl.White)

			if rl.IsKeyPressed(rl.KeyEnter) {
				gameOver = false
				gameWon = false
				resetGame(&player, bullets[:], enemyBullets[:], shields[:], &invaders, &invaderDirection, &score, cfg)
			}
			rl.EndDrawing()
			continue
		}

		if gameWon {
			rl.DrawText("You Won!", 320, 250, 40, rl.Gold)
			rl.DrawText(fmt.Sprintf("Score: %d", score), 285, 310, 30, rl.White)
			rl.DrawText("Press Enter to Restart, ESC to quit", 180, 360, 20, rl.White)

			if rl.IsKeyPressed(rl.KeyEnter) {
				gameOver = false
				gameWon = false
				resetGame(&player, bullets[:], enemyBullets[:], shields[:], &invaders, &invaderDirection, &score, cfg)
			}
			rl.EndDrawing()
			continue
		}

		if rl.IsKeyDown(rl.KeyRight) || rl.IsKeyDown(rl.KeyD) || rl.IsKeyDown(rl.KeyL) {
			player.X += player.Speed
		}
		if rl.IsKeyDown(rl.KeyLeft) || rl.IsKeyDown(rl.KeyA) || rl.IsKeyDown(rl.KeyH) {
			player.X -= player.Speed
		}
		if player.X < 0 {
			player.X = 0
		}
		if player.X+player.Width > float32(cfg.ScreenWidth) {
			player.X = float32(cfg.ScreenWidth) - player.Width
		}
		if playerShootTimer > 0 {
			playerShootTimer--
		}
		if (rl.IsKeyPressed(rl.KeySpace) || rl.IsKeyDown(rl.KeySpace)) && playerShootTimer == 0 {
			for i := range bullets {
				if !bullets[i].Active {
					bullets[i].X = player.X + player.Width/2 - bullets[i].Width/2
					bullets[i].Y = player.Y
					bullets[i].Active = true
					playerShootTimer = 8
					break
				}
			}
		}
		for i := range bullets {
			bullets[i].Update()
		}
		for i := range enemyBullets {
			enemyBullets[i].Update()
		}

		for i := range bullets {
			if !bullets[i].Active {
				continue
			}
			for r := range invaders {
				for c := range invaders[r] {
					inv := &invaders[r][c]
					if inv.Alive && bullets[i].Rect().Intersects(inv.Rect()) {
						bullets[i].Active = false
						inv.Alive = false
						score += 10
						break
					}
				}
				if !bullets[i].Active {
					break
				}
			}
			if !bullets[i].Active {
				continue
			}
			for s := range shields {
				shield := &shields[s]
				if shield.Health > 0 && bullets[i].Rect().Intersects(shield.Rect()) {
					bullets[i].Active = false
					shield.Health -= 1
					break
				}
			}
		}

		for i := range enemyBullets {
			bullet := &enemyBullets[i]
			if !bullet.Active {
				continue
			}
			if bullet.Rect().Intersects(player.Rect()) {
				bullet.Active = false
				gameOver = true
				continue
			}
			for s := range shields {
				shield := &shields[s]
				if shield.Health > 0 && bullet.Rect().Intersects(shield.Rect()) {
					bullet.Active = false
					shield.Health -= 1
					break
				}
			}
		}

		enemyShootTimer++
		if enemyShootTimer >= enemyShootDelay {
			enemyShootTimer = 0
			for i := range invaders {
				for j := range invaders[i] {
					inv := &invaders[i][j]
					if inv.Alive && rl.GetRandomValue(0, 100) < enemyShootChnace {
						for k := range enemyBullets {
							b := &enemyBullets[k]
							if !b.Active {
								b.X = inv.X + inv.Width/2 - b.Width/2
								b.Y = inv.Y + inv.Height
								b.Active = true
								break
							}
						}
						break
					}
				}
			}
		}

		moveTimer++
		if moveTimer >= invaderMoveDelay {
			moveTimer = 0
			hitEdge := false

			for i := range invaders {
				for j := range invaders[i] {
					inv := &invaders[i][j]
					if !inv.Alive {
						continue
					}
					nextX := inv.X + float32(invaderSpeed)*invaderDirection
					if nextX < 0 || nextX+inv.Width > float32(cfg.ScreenWidth) {
						hitEdge = true
						break
					}
				}
				if hitEdge {
					break
				}
			}

			if hitEdge {
				invaderDirection *= -1
				for i := range invaders {
					for j := range invaders[i] {
						invaders[i][j].Update(0, invaderDropDistance)
					}
				}
			} else {
				for i := range invaders {
					for j := range invaders[i] {
						invaders[i][j].Update(float32(invaderSpeed)*invaderDirection, 0)
					}
				}
			}
		}

		for i := range invaders {
			for j := range invaders[i] {
				inv := &invaders[i][j]
				if inv.Alive && inv.Rect().Intersects(player.Rect()) {
					gameOver = true
				}
			}
		}

		allInvadersDead := true
		for i := range invaders {
			for j := range invaders[i] {
				if invaders[i][j].Alive {
					allInvadersDead = false
					break
				}
			}
			if !allInvadersDead {
				break
			}
		}
		if allInvadersDead {
			gameWon = true
		}

		for i := range shields {
			shields[i].Draw()
		}
		player.Draw()
		for i := range bullets {
			if bullets[i].Active {
				bullets[i].Draw()
			}
		}
		for i := range invaders {
			for j := range invaders[i] {
				invaders[i][j].Draw()
			}
		}
		for i := range enemyBullets {
			if enemyBullets[i].Active {
				enemyBullets[i].Draw()
			}
		}

		rl.DrawText(fmt.Sprintf("Score: %d", score), 20, int32(cfg.ScreenHeight)-30, 20, rl.White)
		rl.DrawText("Go Invaders - SPACE to shoot, ESC to quit", 20, 5, 20, rl.Green)
		rl.EndDrawing()

	}

}
