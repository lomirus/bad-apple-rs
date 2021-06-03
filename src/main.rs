mod data;
use crossterm::{cursor::MoveTo, style::Print, terminal::{self, Clear, ClearType}, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

const FRAMES: usize = data::DATA.len();
const HEIGHT: usize = data::DATA[0].len();
const WIDTH: usize = data::DATA[0][0].len();
const FPS: u64 = 24;

fn main() {
    let mut stdout = stdout();
    let (left, top) = get_padding();
    
    stdout.queue(Clear(ClearType::All)).unwrap();
    for row in 0..HEIGHT {
        stdout.queue(MoveTo(left, top + row as u16)).unwrap();
        for col in 0..WIDTH {
            stdout
                .queue(Print(data_to_str(data::DATA[0][row][col])))
                .unwrap();
        }
    }
    let start = Instant::now();
    for i in 1..FRAMES {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if data::DATA[i - 1][row][col] != data::DATA[i][row][col] {
                    stdout
                        .queue(MoveTo(left + col as u16 * 8, top + row as u16))
                        .unwrap()
                        .queue(Print(data_to_str(data::DATA[i][row][col])))
                        .unwrap();
                }
            }
        }
        stdout.flush().unwrap();
        while (start.elapsed().as_millis() as f64) < (1000 as f64) / (FPS as f64) * (i as f64) {
            sleep(Duration::from_millis(10));
        }
    }
}

fn get_padding() -> (u16, u16) {
    let (t_width, t_height) = terminal::size().unwrap();
    let left = if WIDTH * 8 < t_width as usize {
        (t_width - WIDTH as u16 * 8) / 2
    } else {
        0
    };
    let top = if HEIGHT < t_height as usize {
        (t_height - HEIGHT as u16) / 2
    } else {
        0
    };
    (left, top)
}

fn data_to_str(n: u8) -> String {
    let mut res = String::new();
    let mut t = n;
    for i in (0..8).rev() {
        if t / (1 << i) == 1 {
            res.push('M');
            t -= 1 << i;
        } else {
            res.push(' ')
        }
    }
    res
}
