package main

import rl "github.com/gen2brain/raylib-go/raylib"

//
// Rectangle struct (class)
//

type Rectangle struct {
	X, Y, Width, Height float32
}

func (r Rectangle) Intersects(other Rectangle) bool {
	return r.X < other.X+other.Width && r.X+r.Width > other.X &&
		r.Y < other.Y+other.Height && r.Y+r.Height > other.Y
}

//
// Player struct
//

type Player struct {
	X, Y          float32
	Width, Height float32
	Speed         float32
}

func NewPlayer(x, y, width, height float32) Player {
	return Player{X: x, Y: y, Width: width, Height: height, Speed: 5}
}

func (p *Player) Update() {
	if rl.IsKeyDown(rl.KeyRight) || rl.IsKeyDown(rl.KeyD) {
		p.X += p.Speed
	}
	if rl.IsKeyDown(rl.KeyLeft) || rl.IsKeyDown(rl.KeyA) {
		p.X -= p.Speed
	}
	if p.X < 0 {
		p.X = 0 // dont go off screen
	}
	if p.X+p.Width > float32(rl.GetScreenWidth()) {
		p.X = float32(rl.GetScreenWidth()) - p.Width // dont go off screen
	}
}

func (p *Player) Rect() Rectangle {
	return Rectangle{p.X, p.Y, p.Width, p.Height}
}

func (p *Player) Draw() {
	rl.DrawRectangle(int32(p.X), int32(p.Y), int32(p.Width), int32(p.Height), rl.Blue)
}

//
// Bullet (player's bullet -- travels upwards)
//

type Bullet struct {
	X, Y          float32
	Width, Height float32
	Speed         float32
	Active        bool
}

func NewBullet(x, y, width, height float32) Bullet {
	return Bullet{X: x, Y: y, Width: width, Height: height, Speed: 10.0}
}
func (b *Bullet) Update() {
	if !b.Active {
		return
	}
	b.Y -= b.Speed
	if b.Y < 0 {
		b.Y = 0
		b.Active = false
	}
}

func (b *Bullet) Rect() Rectangle {
	return Rectangle{b.X, b.Y, b.Width, b.Height}
}

func (b *Bullet) Draw() {
	rl.DrawRectangle(int32(b.X), int32(b.Y), int32(b.Width), int32(b.Height), rl.Red)
}

//
// Enemy bullet (alien) travels downwards
//

type EnemyBullet struct {
	X, Y          float32
	Width, Height float32
	Speed         float32
	Active        bool
}

func NewEnemyBullet(x, y, width, height float32) EnemyBullet {
	return EnemyBullet{X: x, Y: y, Width: width, Height: height, Speed: 5.0}
}

func (b *EnemyBullet) Update() {
	if !b.Active {
		return
	}
	b.Y += b.Speed
	if b.Y > float32(rl.GetScreenHeight()) {
		b.Active = false
	}
}

func (b *EnemyBullet) Rect() Rectangle {
	return Rectangle{b.X, b.Y, b.Width, b.Height}
}

func (b *EnemyBullet) Draw() {
	rl.DrawRectangle(int32(b.X), int32(b.Y), int32(b.Width), int32(b.Height), rl.Yellow)
}

//
// Invader (alien)
//

type Invader struct {
	X, Y          float32
	Width, Height float32
	Speed         float32
	Alive         bool
}

func NewInvader(x, y, width, height float32) Invader {
	return Invader{X: x, Y: y, Width: width, Height: height, Speed: 5.0, Alive: true}
}

func (i *Invader) Update(dx, dy float32) {
	i.X += dx
	i.Y += dy
	if i.X < 0 {
		i.X = 0 // dont go off screen
	}
	if i.X+i.Width > float32(rl.GetScreenWidth()) {
		i.X = float32(rl.GetScreenWidth()) - i.Width // dont go off screen
	}
	if i.Y < 0 {
		i.Y = 0 // dont go off screen
	}
	if i.Y+i.Height > float32(rl.GetScreenHeight()) {
		i.Y = float32(rl.GetScreenHeight()) - i.Height // dont go off screen
		i.Alive = false
	}
}

func (i *Invader) Rect() Rectangle {
	return Rectangle{i.X, i.Y, i.Width, i.Height}
}

func (i *Invader) Draw() {
	if i.Alive {
		rl.DrawRectangle(int32(i.X), int32(i.Y), int32(i.Width), int32(i.Height), rl.Green)
	}
}

//
// Shield
//

type Shield struct {
	X, Y          float32
	Width, Height float32
	Health        int32
}

func NewShield(x, y, width, height float32) Shield {
	return Shield{X: x, Y: y, Width: width, Height: height, Health: 10}
}

func (s *Shield) Rect() Rectangle {
	return Rectangle{s.X, s.Y, s.Width, s.Height}
}

func (s *Shield) Draw() {
	if s.Health <= 0 {
		return
	}
	alpha := uint8(min32(255, s.Health*25))
	rl.DrawRectangle(
		int32(s.X), int32(s.Y), int32(s.Width), int32(s.Height),
		rl.Color{R: 0, G: 255, B: 255, A: alpha},
	)
}

//
// Hepler functions
//

func min32(a, b int32) int32 {
	if a < b {
		return a
	}
	return b
}
