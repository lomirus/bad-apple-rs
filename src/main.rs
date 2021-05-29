mod data;
use crossterm::{cursor::MoveTo, style::Print, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut stdout = stdout();
    for row in 0..44 {
        stdout.queue(MoveTo(27, row as u16)).unwrap();
        for col in 0..101 {
            stdout
                .queue(Print(if data::DATA[0][row][col] { 'M' } else { ' ' }))
                .unwrap();
        }
    }
    for i in 1..4383 {
        for row in 0..44 {
            for col in 0..101 {
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
        sleep(Duration::from_millis(50));
    }
}
