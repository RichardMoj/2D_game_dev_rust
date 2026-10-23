use bracket_lib::prelude::*;

const FRAME_DURATION: f32 = 75.0;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct Obstacle {
    x: i32, // World-Space coords
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut rng_number = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: rng_number.range(10, 40),
            size: i32::max(20 - score, 2),
        }
    }

    fn render(&self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn is_colliding(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let overlapping_pos = player.x == self.x;
        let player_above_top = player.y < self.gap_y - half_size;
        let player_below_bot = player.y > self.gap_y + half_size;

        overlapping_pos && (player_above_top || player_below_bot)
    }
}

struct Player {
    x: i32, // World-Space coords
    y: i32, // Screen-Space coords
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        if self.y < 0 {
            self.y = 0;
        }

        self.x += 1;
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
    score: i32,
    obstacle: Obstacle,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
            score: 0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        ctx.print(0, 0, "Press SPACE to flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        self.obstacle.render(ctx, self.player.x);

        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.is_colliding(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(1, "Main Menu");
        ctx.print_centered(5, "Welcome to Flappy Dragon!");
        ctx.print_centered(8, "(P)lay");
        ctx.print_centered(9, "(Q)uit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(1, "Game Over");
        ctx.print_centered(6, &format!("Final Score: {}", self.score));
        ctx.print_centered(8, "(P)lay Again");
        ctx.print_centered(9, "(Q)uit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.mode = GameMode::Playing;
        self.frame_time = 0.0;
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
