mod data;
use crossterm::{cursor::MoveTo, style::Print, QueueableCommand, terminal};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

const FRAMES: usize = data::DATA.len();
const HEIGHT: usize = data::DATA[0].len();
const WIDTH: usize = data::DATA[0][0].len();
const FPS: u64 = 24;

fn main() {
    let mut stdout = stdout();
    let (left, top) = get_padding();
    for row in 0..HEIGHT {
        stdout.queue(MoveTo(left, top + row as u16)).unwrap();
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
                        .queue(MoveTo(left + col as u16 , top + row as u16))
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

fn get_padding() -> (u16, u16) {
    let (t_width, t_height) = terminal::size().unwrap();
    let left = if WIDTH < t_width as usize {
        (t_width - WIDTH as u16) / 2
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