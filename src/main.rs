mod data;
use crossterm::{cursor::MoveTo, style::Print, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

const FRAMES: usize = data::DATA.len();
const ROWS: usize = data::DATA[0].len();
const COLS: usize = data::DATA[0][0].len();

fn main() {
    let mut stdout = stdout();
    for row in 0..ROWS {
        stdout.queue(MoveTo(27, row as u16)).unwrap();
        for col in 0..COLS {
            stdout
                .queue(Print(if data::DATA[0][row][col] { 'M' } else { ' ' }))
                .unwrap();
        }
    }
    for i in 1..FRAMES {
        for row in 0..ROWS {
            for col in 0..COLS {
                if data::DATA[i - 1][row][col] != data::DATA[i][row][col] {
                    stdout
                        .queue(MoveTo((col + 27) as u16, row as u16))
                        .unwrap()
                        .queue(Print(if data::DATA[i][row][col] { 'M' } else { ' ' }))
                        .unwrap();
                }
            }
        }
        stdout.flush().unwrap();
        sleep(Duration::from_millis((1000 / FRAMES) as u64));
    }
}
