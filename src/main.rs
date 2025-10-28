use macroquad::prelude::*;
use macroquad::ui;

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

    fn next(self, neighbours: &[Cell]) -> Cell {
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

    fn neighbours(&self, y: usize, x: usize) -> [Cell; 4] {
        [
            if y > 0 { self.0[y-1][x] } else { Cell::Empty },
            if x > 0 { self.0[y][x-1] } else { Cell::Empty },
            if y < self.height()-1 { self.0[y+1][x] } else { Cell::Empty },
            if x < self.width()-1 { self.0[y][x+1] } else { Cell::Empty },
        ]
    }

    fn next(&self) -> Playground {
        let mut new = Playground::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                new.0[y][x] = self.0[y][x].next(&self.neighbours(y, x));
            }
        }
        new
    }
}

#[macroquad::main("Rock-paper-scissors")]
async fn main() {
    let mut playground = Playground::new();
    let mut brush = Cell::Rock;
    let mut playing = false;
    let scale = 25.0;

    request_new_screen_size(playground.width() as f32 * scale, playground.height() as f32 * scale);

    loop {
        clear_background(BLACK);

        if ui::root_ui().button(None, "Rock") {
            brush = Cell::Rock;
        }
        if ui::root_ui().button(None, "Paper") {
            brush = Cell::Paper;
        }
        if ui::root_ui().button(None, "Scissors") {
            brush = Cell::Scissors;
        }
        if ui::root_ui().button(None, "Lizard") {
            brush = Cell::Lizard;
        }
        if ui::root_ui().button(None, "Spock") {
            brush = Cell::Spock;
        }
        if ui::root_ui().button(None, "Eraser") {
            brush = Cell::Empty;
        }

        if ui::root_ui().button(None, if playing {"Pause"} else {"Play"}) {
            playing = !playing;
        }

        let (mouse_x, mouse_y) = mouse_position();
        let x = ((mouse_x / scale) as usize).clamp(0, playground.width() - 1);
        let y = ((mouse_y / scale) as usize).clamp(0, playground.height() - 1);

        if is_mouse_button_down(MouseButton::Left) {
            playground.0[y][x] = brush;
        }

        if playing {
            playground = playground.next();
        }

        for y in 0..playground.height() {
            for x in 0..playground.width() {
                draw_rectangle(x as f32 * scale, y as f32 * scale, scale, scale, playground.0[y][x].color());
            }
        }

        draw_circle(mouse_x, mouse_y, scale / 4.0, brush.color());
        next_frame().await;
    }
}
