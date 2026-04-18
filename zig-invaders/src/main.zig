const rl = @import("raylib");

const Rectangle = struct {
    x: f32,
    y: f32,
    width: f32,
    height: f32,

    pub fn intersects(self: Rectangle, other: Rectangle) bool {
        return self.x < other.x + other.width and
            self.x + self.width > other.x and
            self.y < other.y + other.height and
            self.y + self.height > other.y;
    }
};

const GameConfig = struct {
    screenWidth: i32,
    screenHeight: i32,
    playerWidth: f32,
    playerHeight: f32,
    playerStartY: f32,
    bulletWidth: f32,
    bulletHeight: f32,
    shieldStartX: f32,
    shieldY: f32,
    shieldWidth: f32,
    shieldHeight: f32,
    shieldSpacing: f32,
    invaderStartX: f32,
    invaderStartY: f32,
    invaderWidth: f32,
    invaderHeight: f32,
    invaderSpacingX: f32,
    invaderSpacingY: f32,
};

const Player = struct {
    position_x: f32,
    position_y: f32,
    width: f32,
    height: f32,
    speed: f32,

    pub fn init(position_x: f32, position_y: f32, width: f32, height: f32) @This() {
        return .{
            .position_x = position_x,
            .position_y = position_y,
            .width = width,
            .height = height,
            .speed = 5.0,
        };
    }
    pub fn update(self: *@This()) void {
        if (rl.isKeyDown(rl.KeyboardKey.right)) {
            self.position_x += self.speed;
        }
        if (rl.isKeyDown(rl.KeyboardKey.left)) {
            self.position_x -= self.speed;
        }
        if (self.position_x < 0) {
            self.position_x = 0;
        }
        if (self.position_x + self.width > @as(f32, @floatFromInt(rl.getScreenWidth()))) {
            self.position_x = @as(f32, @floatFromInt(rl.getScreenWidth())) - self.width;
        }
    }

    pub fn getRect(self: *@This()) Rectangle {
        return .{
            .x = self.position_x,
            .y = self.position_y,
            .width = self.width,
            .height = self.height,
        };
    }
    pub fn draw(self: *@This()) void {
        rl.drawRectangle(
            @intFromFloat(self.position_x),
            @intFromFloat(self.position_y),
            @intFromFloat(self.width),
            @intFromFloat(self.height),
            rl.Color.blue,
        );
    }
};

const Bullet = struct {
    position_x: f32,
    position_y: f32,
    width: f32,
    height: f32,
    speed: f32,
    active: bool,

    pub fn init(position_x: f32, position_y: f32, width: f32, height: f32) @This() {
        return .{
            .position_x = position_x,
            .position_y = position_y,
            .width = width,
            .height = height,
            .speed = 10.0,
            .active = false,
        };
    }

    pub fn update(self: *@This()) void {
        if (self.active) {
            self.position_y -= self.speed;
            if (self.position_y < 0) {
                self.position_y = 0;
                self.active = false;
            }
        }
    }

    pub fn draw(self: *@This()) void {
        rl.drawRectangle(
            @intFromFloat(self.position_x),
            @intFromFloat(self.position_y),
            @intFromFloat(self.width),
            @intFromFloat(self.height),
            rl.Color.red,
        );
    }

    pub fn getRect(self: *@This()) Rectangle {
        return .{
            .x = self.position_x,
            .y = self.position_y,
            .width = self.width,
            .height = self.height,
        };
    }
};

const Invader = struct {
    position_x: f32,
    position_y: f32,
    width: f32,
    height: f32,
    speed: f32,
    alive: bool,

    pub fn init(position_x: f32, position_y: f32, width: f32, height: f32) @This() {
        return .{
            .position_x = position_x,
            .position_y = position_y,
            .width = width,
            .height = height,
            .speed = 5.0,
            .alive = true,
        };
    }

    pub fn draw(self: *@This()) void {
        if (self.alive) {
            rl.drawRectangle(
                @intFromFloat(self.position_x),
                @intFromFloat(self.position_y),
                @intFromFloat(self.width),
                @intFromFloat(self.height),
                rl.Color.green,
            );
        }
    }
    pub fn getRect(self: *@This()) Rectangle {
        return .{
            .x = self.position_x,
            .y = self.position_y,
            .width = self.width,
            .height = self.height,
        };
    }

    pub fn update(self: *@This(), dx: f32, dy: f32) void {
        self.position_x += dx;
        self.position_y += dy;
        if (self.position_x < 0) {
            self.position_x = 0;
        }
        if (self.position_x + self.width > @as(f32, @floatFromInt(rl.getScreenWidth()))) {
            self.position_x = @as(f32, @floatFromInt(rl.getScreenWidth())) - self.width;
        }
        if (self.position_y < 0) {
            self.position_y = 0;
        }
        if (self.position_y + self.height > @as(f32, @floatFromInt(rl.getScreenHeight()))) {
            self.position_y = @as(f32, @floatFromInt(rl.getScreenHeight())) - self.height;
            self.alive = false;
        }
    }
};

const enemybullet = struct {
    position_x: f32,
    position_y: f32,
    width: f32,
    height: f32,
    speed: f32,
    active: bool,

    pub fn init(position_x: f32, position_y: f32, width: f32, height: f32) @This() {
        return .{
            .position_x = position_x,
            .position_y = position_y,
            .width = width,
            .height = height,
            .speed = 5.0,
            .active = false,
        };
    }
    pub fn getRect(self: *@This()) Rectangle {
        return .{
            .x = self.position_x,
            .y = self.position_y,
            .width = self.width,
            .height = self.height,
        };
    }

    pub fn update(self: *@This(), screen_height: i32) void {
        if (self.active) {
            self.position_y += self.speed;
            if (self.position_y > @as(f32, @floatFromInt(screen_height))) {
                self.active = false;
            }
        }
    }
    pub fn draw(self: *@This()) void {
        rl.drawRectangle(
            @intFromFloat(self.position_x),
            @intFromFloat(self.position_y),
            @intFromFloat(self.width),
            @intFromFloat(self.height),
            rl.Color.yellow,
        );
    }
};

const Shield = struct {
    position_x: f32,
    position_y: f32,
    width: f32,
    height: f32,
    health: i32,

    pub fn init(position_x: f32, position_y: f32, width: f32, height: f32) @This() {
        return .{
            .position_x = position_x,
            .position_y = position_y,
            .width = width,
            .height = height,
            .health = 10,
        };
    }

    pub fn getRect(self: *@This()) Rectangle {
        return .{
            .x = self.position_x,
            .y = self.position_y,
            .width = self.width,
            .height = self.height,
        };
    }

    pub fn draw(self: *@This()) void {
        if (self.health > 0) {
            const alpha = @as(u8, @intCast(@min(255, self.health * 25)));
            rl.drawRectangle(
                @intFromFloat(self.position_x),
                @intFromFloat(self.position_y),
                @intFromFloat(self.width),
                @intFromFloat(self.height),
                rl.Color{ .r = 0, .g = 255, .b = 255, .a = alpha },
            );
        }
    }
};

fn reset_game(
    player: *Player,
    bullets: []Bullet,
    enemyBullets: []enemybullet,
    shields: []Shield,
    invaders: anytype,
    invaderDirection: *f32,
    score: *i32,
    config: GameConfig,
) void {
    score.* = 0;
    player.* = Player.init(
        @as(f32, @floatFromInt(config.screenWidth)) / 2.0 - config.playerWidth / 2.0,
        @as(f32, @floatFromInt(config.screenHeight)) - 60.0,
        config.playerWidth,
        config.playerHeight,
    );
    for (bullets) |*bullet| {
        bullet.active = false;
    }
    for (enemyBullets) |*bullet| {
        bullet.active = false;
    }
    for (shields, 0..) |*shield, i| {
        const x = config.shieldStartX + @as(f32, @floatFromInt(i)) * config.shieldSpacing;
        shield.* = Shield.init(x, config.shieldY, config.shieldWidth, config.shieldHeight);
    }
    for (invaders, 0..) |*row, i| {
        for (row, 0..) |*invader, j| {
            const x = config.invaderStartX + @as(f32, @floatFromInt(j)) * config.invaderSpacingX;
            const y = config.invaderStartY + @as(f32, @floatFromInt(i)) * config.invaderSpacingY;
            invader.* = Invader.init(x, y, config.invaderWidth, config.invaderHeight);
        }
    }
    invaderDirection.* = 1.0;
}

pub fn main() void {
    const screenWidth = 800;
    const screenHeight = 600;
    const maxBullets = 10;
    const bulletWidth = 4.0;
    const bulletHeight = 10.0;
    const invaderRows = 5;
    const invaderColumns = 11;
    const invaderWidth = 40.0;
    const invaderHeight = 30.0;
    const invaderStartX = 100.0;
    const invaderStartY = 50.0;
    const invaderSpacingX = 60.0;
    const invaderSpacingY = 40.0;
    const invaderSpeed = 5.0;
    const invaderMoveDelay = 30;
    const invaderDropDistance = 20.0;
    var invaderDirection: f32 = 1.0;
    var moveTimer: i32 = 0;
    var score: i32 = 0;
    const maxEnemyBullets = 20;
    const enemyShootDelay = 60;
    var enemyShootTimer: i32 = 0;
    const enemeyShootChance = 5;
    var gameOver: bool = false;
    const shieldWidth = 80.0;
    const shieldHeight = 60.0;
    const shieldStartX = 150.0;
    const shieldY = 450.0;
    const shieldSpacing = 150.0;
    const shieldCount = 4;
    var gameWon: bool = false;
    const playerWidth = 50.0;
    const playerHeight = 30.0;
    const playerStartY = @as(f32, @floatFromInt(screenHeight)) - 60.0;

    const config = GameConfig{
        .screenWidth = screenWidth,
        .screenHeight = screenHeight,
        .playerWidth = playerWidth,
        .playerHeight = playerHeight,
        .playerStartY = playerStartY,
        .bulletWidth = bulletWidth,
        .bulletHeight = bulletHeight,
        .shieldStartX = shieldStartX,
        .shieldY = shieldY,
        .shieldWidth = shieldWidth,
        .shieldHeight = shieldHeight,
        .shieldSpacing = shieldSpacing,
        .invaderStartX = invaderStartX,
        .invaderStartY = invaderStartY,
        .invaderWidth = invaderWidth,
        .invaderHeight = invaderHeight,
        .invaderSpacingX = invaderSpacingX,
        .invaderSpacingY = invaderSpacingY,
    };

    rl.initWindow(screenWidth, screenHeight, "zig-invaders");
    defer rl.closeWindow();

    var player = Player.init(
        @as(f32, @floatFromInt(screenWidth)) / 2.0 - playerWidth / 2.0,
        @as(f32, @floatFromInt(screenHeight)) - 60.0,
        playerWidth,
        playerHeight,
    );

    var shields: [shieldCount]Shield = undefined;
    for (&shields, 0..) |*shield, i| {
        const x = shieldStartX + @as(f32, @floatFromInt(i)) * shieldSpacing;
        shield.* = Shield.init(x, shieldY, shieldWidth, shieldHeight);
    }

    var bullets: [maxBullets]Bullet = undefined;
    for (&bullets) |*bullet| {
        bullet.* = Bullet.init(0, 0, bulletWidth, bulletHeight);
    }

    var enemyBullets: [maxEnemyBullets]enemybullet = undefined;
    for (&enemyBullets) |*bullet| {
        bullet.* = enemybullet.init(0, 0, bulletWidth, bulletHeight);
    }

    var invaders: [invaderRows][invaderColumns]Invader = undefined;

    for (&invaders, 0..) |*row, i| {
        for (row, 0..) |*invader, j| {
            const x = invaderStartX + @as(f32, @floatFromInt(j)) * invaderSpacingX;
            const y = invaderStartY + @as(f32, @floatFromInt(i)) * invaderSpacingY;
            invader.* = Invader.init(x, y, invaderWidth, invaderHeight);
        }
    }

    rl.setTargetFPS(60);

    while (!rl.windowShouldClose()) {
        rl.beginDrawing();
        defer rl.endDrawing();

        rl.clearBackground(rl.Color.black);

        if (gameOver) {
            rl.drawText("GAME OVER", 270, 250, 40, rl.Color.red);
            const scoreText = rl.textFormat("Final Score: %d", .{score});
            rl.drawText(scoreText, 285, 310, 30, rl.Color.white);
            rl.drawText("Press ENTER to play again or ESC to quit", 180, 360, 20, rl.Color.white);

            if (rl.isKeyPressed(rl.KeyboardKey.enter)) {
                gameOver = false;
                reset_game(
                    &player,
                    &bullets,
                    &enemyBullets,
                    &shields,
                    &invaders,
                    &invaderDirection,
                    &score,
                    config,
                );
            }
            continue;
        }
        if (gameWon) {
            rl.drawText("YOU WIN!", 320, 250, 40, rl.Color.gold);
            const scoreText = rl.textFormat("Final Score: %d", .{score});
            rl.drawText(scoreText, 285, 310, 30, rl.Color.white);
            rl.drawText("Press ENTER to play again or ESC to quit", 180, 360, 20, rl.Color.green);

            if (rl.isKeyPressed(rl.KeyboardKey.enter)) {
                gameOver = false;
                reset_game(
                    &player,
                    &bullets,
                    &enemyBullets,
                    &shields,
                    &invaders,
                    &invaderDirection,
                    &score,
                    config,
                );

            }
            continue;
        }

        player.update();
        if (rl.isKeyPressed(rl.KeyboardKey.space)) {
            for (&bullets) |*bullet| {
                if (!bullet.active) {
                    bullet.position_x = player.position_x + player.width / 2 - bullet.width / 2;
                    bullet.position_y = player.position_y;
                    bullet.active = true;
                    break;
                }
            }
        }

        for (&bullets) |*bullet| {
            bullet.update();
        }

        for (&bullets) |*bullet| {
            if (bullet.active) {
                for (&invaders) |*row| {
                    for (row) |*invader| {
                        if (invader.alive) {
                            if (bullet.getRect().intersects(invader.getRect())) {
                                bullet.active = false;
                                invader.alive = false;
                                score += 10;
                                break;
                            }
                        }
                    }
                }
                for (&shields) |*shield| {
                    if (shield.health > 0) {
                        if (bullet.getRect().intersects(shield.getRect())) {
                            bullet.active = false;
                            shield.health -= 1;
                            break;
                        }
                    }
                }
            }
        }

        for (&enemyBullets) |*bullet| {
            bullet.update(rl.getScreenHeight());
            if (bullet.active) {
                if (bullet.getRect().intersects(player.getRect())) {
                    bullet.active = false;
                    gameOver = true;
                }

                for (&shields) |*shield| {
                    if (shield.health > 0) {
                        if (bullet.getRect().intersects(shield.getRect())) {
                            bullet.active = false;
                            shield.health -= 1;
                            break;
                        }
                    }
                }
            }
        }

        enemyShootTimer += 1;
        if (enemyShootTimer >= enemyShootDelay) {
            enemyShootTimer = 0;
            for (&invaders) |*row| {
                for (row) |*invader| {
                    if (invader.alive and rl.getRandomValue(0, 100) < enemeyShootChance) {
                        for (&enemyBullets) |*bullet| {
                            if (!bullet.active) {
                                bullet.position_x = invader.position_x + invader.width / 2 - bullet.width / 2;
                                bullet.position_y = invader.position_y + invader.height;
                                bullet.active = true;
                                break;
                            }
                        }

                        break;
                    }
                }
            }
        }

        moveTimer += 1;
        if (moveTimer >= invaderMoveDelay) {
            moveTimer = 0;
            var hit_edge: bool = false;

            for (&invaders) |*row| {
                for (row) |*invader| {
                    if (invader.alive) {
                        const next_x = invader.position_x + (invaderSpeed * invaderDirection);
                        if (next_x < 0 or next_x + invader.width > @as(f32, @floatFromInt(screenWidth))) {
                            hit_edge = true;
                            break;
                        }
                    }
                }
                if (hit_edge) break;
            }

            if (hit_edge) {
                invaderDirection *= -1.0;
                for (&invaders) |*row| {
                    for (row) |*invader| {
                        invader.update(0, invaderDropDistance);
                    }
                }
            } else {
                for (&invaders) |*row| {
                    for (row) |*invader| {
                        invader.update(invaderSpeed * invaderDirection, 0);
                    }
                }
            }
        }

        for (&invaders) |*row| {
            for (row) |*invader| {
                if (invader.alive) {
                    if (invader.getRect().intersects(player.getRect())) {
                        invader.alive = false;
                        gameOver = true;
                    }
                }
            }
        }

        var all_invaders_dead: bool = true;
        outer_loop: for (&invaders) |*row| {
            for (row) |*invader| {
                if (invader.alive) {
                    all_invaders_dead = false;
                    break :outer_loop;
                }
            }
        }

        if (all_invaders_dead) {
            gameWon = true;
        }

        //Draw logic
        for (&shields) |*shield| {
            shield.draw();
        }

        player.draw();
        for (&bullets) |*bullet| {
            if (bullet.active) {
                bullet.draw();
            }
        }

        for (&invaders) |*row| {
            for (row) |*invader| {
                invader.draw();
            }
        }

        for (&enemyBullets) |*bullet| {
            bullet.draw();
        }

        const scoreText = rl.textFormat("Score: %d", .{score});
        rl.drawText(scoreText, 20, screenHeight - 20, 20, rl.Color.white);

        rl.drawText(
            "zig-invaders - SPACE to shoot, ESC to quit",
            20,
            20,
            20,
            rl.Color.green,
        );
    }
}
