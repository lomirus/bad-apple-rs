mod data;
use crossterm::{cursor::MoveTo, style::Print, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

const FRAMES: usize = data::DATA.len();
const HEIGHT: usize = data::DATA[0].len();
const WIDTH: usize = data::DATA[0][0].len();
const FPS: u64 = 24;

fn main() {
    let mut stdout = stdout();
    for row in 0..HEIGHT {
        stdout.queue(MoveTo(27, row as u16)).unwrap();
        for col in 0..WIDTH {
            stdout
                .queue(Print(if data::DATA[0][row][col] { 'M' } else { ' ' }))
                .unwrap();
        }
    }
    for i in 1..FRAMES {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if data::DATA[i - 1][row][col] != data::DATA[i][row][col] {
                    stdout
                        .queue(MoveTo(col as u16 + 27, row as u16))
                        .unwrap()
                        .queue(Print(if data::DATA[i][row][col] { 'M' } else { ' ' }))
                        .unwrap();
                }
            }
        }
        stdout.flush().unwrap();
        sleep(Duration::from_millis(1000 / FPS));
    }
}
