package main

// game config
type GameConfig struct {
	ScreenWidth     int32
	ScreenHeight    int32
	PlayerWidth     float32
	PlayerHeight    float32
	PlayerStartY    float32
	BulletWidth     float32
	BulletHeight    float32
	SheildStartX    float32
	ShieldY         float32
	ShieldWidth     float32
	ShieldHeight    float32
	ShieldSpacing   float32
	InvaderStartX   float32
	InvaderStartY   float32
	InvaderWidth    float32
	InvaderHeight   float32
	InvaderSpacingX float32
	InvaderSpacingY float32
}

const (
	maxBullets      = 10
	maxEnemyBullets = 20
	invaderRows     = 5
	invaderCols     = 11
	shieldCount     = 4

	invaderSpeed        = 5
	invaderMoveDelay    = 30
	invaderDropDistance = 20.0
	enemyShootDelay     = 60
	enemyShootChnace    = 5
)

func defaultGameConfig() GameConfig {
	const sw int32 = 800
	const sh int32 = 600
	const pw float32 = 50.0
	const ph float32 = 30.0
	return GameConfig{
		ScreenWidth:     sw,
		ScreenHeight:    sh,
		PlayerWidth:     pw,
		PlayerHeight:    ph,
		PlayerStartY:    float32(sh) - 60,
		BulletWidth:     4.0,
		BulletHeight:    10.0,
		SheildStartX:    150.0,
		ShieldY:         450.0,
		ShieldWidth:     80.0,
		ShieldHeight:    60.0,
		ShieldSpacing:   150.0,
		InvaderStartX:   100.0,
		InvaderStartY:   30.0,
		InvaderWidth:    40.0,
		InvaderHeight:   30.0,
		InvaderSpacingX: 60.0,
		InvaderSpacingY: 40.0,
	}
}
