use macroquad::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Rock, Paper, Scissors, Lizard, Spock, Empty
}

impl Cell {
    fn color(self) -> Color {
        match self {
            Cell::Rock => Color::from_rgba(255, 0, 0, 255),
            Cell::Paper => Color::from_rgba(0, 255, 0, 255),
            Cell::Scissors => Color::from_rgba(0, 0, 255, 255),
            Cell::Lizard => Color::from_rgba(255, 255, 32, 255),
            Cell::Spock => Color::from_rgba(128, 128, 128, 255),
            Cell::Empty => Color::from_rgba(0, 0, 0, 255)
        }
    }

    fn next(self, right: Cell, down: Cell, left: Cell, up: Cell) -> Cell {
        let neighbours = [right, down, left, up];
        match self {
            Cell::Rock | Cell::Spock if neighbours.contains(&Cell::Paper) => Cell::Paper,
            Cell::Rock | Cell::Scissors if neighbours.contains(&Cell::Spock) => Cell::Spock,
            Cell::Paper | Cell::Lizard if neighbours.contains(&Cell::Scissors) => Cell::Scissors,
            Cell::Paper | Cell::Spock if neighbours.contains(&Cell::Lizard) => Cell::Lizard,
            Cell::Scissors | Cell::Lizard if neighbours.contains(&Cell::Rock) => Cell::Rock,
            _ => self
        }
    }
}
struct Playground([[Cell; 64]; 36]);

impl Playground {
    fn new() -> Playground {
        Playground([[Cell::Empty; 64]; 36])
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn next(&self) -> Playground {
        let mut new = Playground::new();
        for y in 1..self.height() - 1 {
            for x in 1..self.width() - 1 {
                new.0[y][x] = self.0[y][x].next(self.0[y][x+1], self.0[y+1][x], self.0[y][x-1], self.0[y-1][x]);
            }
        }
        new
    }
}

#[macroquad::main("Rock-paper-scissors")]
async fn main() {
    let mut playground = Playground::new();
    let scale = 25.0;

    request_new_screen_size(playground.width() as f32 * scale, playground.height() as f32 * scale);

    loop {
        clear_background(BLACK);

        let (x, y) = mouse_position();
        let x = ((x / scale) as usize).clamp(0, playground.width() - 1);
        let y = ((y / scale) as usize).clamp(0, playground.height() - 1);

        if is_key_down(KeyCode::Q) {
            playground.0[y][x] = Cell::Rock;
        } else if is_key_down(KeyCode::W) {
            playground.0[y][x] = Cell::Paper;
        } else if is_key_down(KeyCode::E) {
            playground.0[y][x] = Cell::Scissors;
        } else if is_key_down(KeyCode::R) {
            playground.0[y][x] = Cell::Lizard;
        } else if is_key_down(KeyCode::T) {
            playground.0[y][x] = Cell::Spock;
        } else if is_key_down(KeyCode::Delete) {
            playground.0[y][x] = Cell::Empty;
        }

        if is_key_down(KeyCode::Space) {
            playground = playground.next();
        }

        for y in 0..playground.height() {
            for x in 0..playground.width() {
                draw_rectangle(x as f32 * scale, y as f32 * scale, scale, scale, playground.0[y][x].color());
            }
        }

        next_frame().await;
    }
}
