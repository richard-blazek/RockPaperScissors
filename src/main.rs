use macroquad::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Rock, Paper, Scissors
}

impl Cell {
    fn draw(self, x: f32, y: f32, size: f32) {
        let s = size;
        match self {
            Cell::Rock => {
                draw_rectangle(x, y, size, size, Color::from_rgba(48, 48, 54, 255));
                draw_poly(x + s * 0.5, y + s * 0.5, 7, s / 2.0, 0.0, Color::from_rgba(92, 92, 92, 255));
            }
            Cell::Paper => {
                draw_rectangle(x, y, size, size, Color::from_rgba(245, 242, 230, 255));
                let rule = Color::from_rgba(170, 200, 235, 255);
                draw_line(x + s * 0.05, y + s / 3.0, x + s * 0.9, y + s / 3.0, 1.0, rule);
                draw_line(x + s * 0.05, y + s * 2.0 / 3.0, x + s * 0.9, y + s * 2.0 / 3.0, 1.0, rule);
                let c = s * 0.28;
                let fold = vec2(x + s - c, y + c);
                draw_triangle(vec2(x + s - c, y), fold, vec2(x + s, y + c), Color::from_rgba(218, 212, 195, 255));
                draw_line(x + s - c, y, x + s, y + c, 1.0, Color::from_rgba(185, 178, 160, 255));
            }
            Cell::Scissors => {
                draw_rectangle(x, y, size, size, Color::from_rgba(96, 160, 40, 255));
                let p = |fx: f32, fy: f32| (x + s * fx, y + s * fy);
                let blade = Color::from_rgba(215, 220, 228, 255);
                let handle = Color::from_rgba(220, 70, 50, 255);
                let t = s * 0.08;
                let (ax, ay) = p(0.37, 0.66);
                let (bx, by) = p(0.8, 0.12);
                draw_line(ax, ay, bx, by, t, blade);
                let (ax, ay) = p(0.63, 0.66);
                let (bx, by) = p(0.2, 0.12);
                draw_line(ax, ay, bx, by, t, blade);
                let (hx, hy) = p(0.3, 0.77);
                draw_circle_lines(hx, hy, s * 0.13, s * 0.06, handle);
                let (hx, hy) = p(0.7, 0.77);
                draw_circle_lines(hx, hy, s * 0.13, s * 0.06, handle);
                let (px, py) = p(0.5, 0.497);
                draw_circle(px, py, s * 0.05, Color::from_rgba(60, 60, 66, 255));
            }
        }
    }

    fn beaten_by(self) -> Cell {
        match self {
            Cell::Rock => Cell::Paper,
            Cell::Paper => Cell::Scissors,
            Cell::Scissors => Cell::Rock,
        }
    }

    fn next(self, neighbours: &[Cell]) -> Cell {
        let beaten_by = self.beaten_by();
        let count = neighbours.iter().filter(|&&c| c == beaten_by).count();
        if count >= 3 {
            beaten_by
        } else {
            self
        }
    }
}

struct Playground([[Cell; 64]; 36]);

impl Playground {
    fn new() -> Playground {
        Playground([[Cell::Paper; 64]; 36])
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn neighbours(&self, y: usize, x: usize) -> [Cell; 8] {
        let this = self.0[y][x];
        let r = self.width()-1;
        let b = self.height()-1;
        [
            if y > 0 { self.0[y-1][x] } else { this },
            if x > 0 { self.0[y][x-1] } else { this },
            if y < b { self.0[y+1][x] } else { this },
            if x < r { self.0[y][x+1] } else { this },
            if y > 0 && x > 0 { self.0[y-1][x-1] } else { this },
            if y < b && x > 0 { self.0[y+1][x-1] } else { this },
            if y > 0 && x < r { self.0[y-1][x+1] } else { this },
            if y < b && x < r { self.0[y+1][x+1] } else { this },
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

fn draw_play_pause(x: f32, y: f32, size: f32, playing: bool) {
    let s = size;
    let lime = Color::from_rgba(130, 230, 30, 255);
    draw_rectangle(x, y, s, s, Color::from_rgba(40, 40, 46, 255));
    if playing {
        draw_rectangle(x + s * 0.25, y + s * 0.2, s * 0.17, s * 0.6, lime);
        draw_rectangle(x + s * 0.58, y + s * 0.2, s * 0.17, s * 0.6, lime);
    } else {
        draw_triangle(vec2(x + s * 0.3, y + s * 0.2), vec2(x + s * 0.3, y + s * 0.8), vec2(x + s * 0.8, y + s * 0.5), lime);
    }
}

#[macroquad::main("Rock-paper-scissors")]
async fn main() {
    let mut playground = Playground::new();
    let mut brush = Cell::Scissors;
    let mut playing = false;
    let scale = 25.0;
    let control = scale * 2.0;
    let line = 6.0;
    let bar = control + line;
    let azure = Color::from_rgba(0, 127, 255, 255);
    let brushes = [Cell::Rock, Cell::Paper, Cell::Scissors];

    request_new_screen_size(playground.width() as f32 * scale, bar + playground.height() as f32 * scale);

    loop {
        clear_background(BLACK);

        let (mouse_x, mouse_y) = mouse_position();

        if is_mouse_button_pressed(MouseButton::Left) && mouse_y < control && mouse_x % (control + line) < control {
            match (mouse_x / (control + line)) as usize {
                i @ 0..3 => brush = brushes[i],
                3 => playing = !playing,
                _ => {}
            }
        }

        if is_mouse_button_down(MouseButton::Left) && mouse_y >= bar {
            let x = ((mouse_x / scale) as usize).clamp(0, playground.width() - 1);
            let y = (((mouse_y - bar) / scale) as usize).clamp(0, playground.height() - 1);
            playground.0[y][x] = brush;
        }

        if playing {
            playground = playground.next();
        }

        for y in 0..playground.height() {
            for x in 0..playground.width() {
                playground.0[y][x].draw(x as f32 * scale, bar + y as f32 * scale, scale);
            }
        }

        for i in 0..4 {
            let cx = i as f32 * (control + line);
            match brushes.get(i) {
                Some(cell) => cell.draw(cx, 0.0, control),
                None => draw_play_pause(cx, 0.0, control, playing),
            }
            draw_rectangle(cx + control, 0.0, line, control, azure);
        }
        draw_rectangle(0.0, control, screen_width(), line, azure);

        let cursor = scale * 0.6;
        brush.draw(mouse_x - cursor / 2.0, mouse_y - cursor / 2.0, cursor);
        draw_rectangle_lines(mouse_x - cursor / 2.0, mouse_y - cursor / 2.0, cursor, cursor, 1.5, WHITE);
        next_frame().await;
    }
}
