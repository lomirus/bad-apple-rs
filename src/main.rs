use crossterm::{
    cursor::{self, MoveTo},
    style::Print,
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
use std::io::{stdout, Write};
use std::iter::zip;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::{Duration, Instant};
use liblzma::write::XzDecoder;

const FRAMES: usize = 5258;         // This is the known frame count
const HEIGHT: usize = 48;
const WIDTH: usize = 16;
const FPS: u64 = 24;

fn main() {
    let data_compressed = include_bytes!("source.bin");
    let mut data = Vec::<u8>::new();
    XzDecoder::new(&mut data).write_all(data_compressed).unwrap();
    debug_assert!(*data.iter().max().unwrap() == 255);

    let lines: Vec<_> = data.chunks_exact(WIDTH).collect::<Vec<_>>();
    let frames = lines.chunks_exact(HEIGHT);
    debug_assert_eq!(frames.len(), FRAMES);         // Verify frame count

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || r.store(false, Ordering::Relaxed)).unwrap();

    let (left, top) = get_padding();
    let mut stdout = stdout();

    stdout
        .queue(Clear(ClearType::All))
        .unwrap()
        .queue(cursor::Hide)
        .unwrap();

    let start = Instant::now();
    let mut c: usize = 0;
    for frame in frames {
        for (i, row) in zip(0.., frame) {
            stdout.queue(MoveTo(left, top + i)).unwrap();
            for n in row.iter() {
                stdout
                    .queue(Print(data_to_str(*n)))
                    .unwrap();
            }
        }
        stdout.flush().unwrap();
        while (start.elapsed().as_millis() as f64) < 1000_f64 / (FPS as f64) * (c as f64) {
            sleep(Duration::from_millis(10));
        }
        c += 1;
        if !running.load(Ordering::Relaxed) { break; };
    }

    stdout
        .queue(Clear(ClearType::All))
        .unwrap()
        .queue(MoveTo(0, 0))
        .unwrap()
        .queue(Print(format!("Played {} frames in {} secs.\n", c, (Instant::now() - start).as_secs())))
        .unwrap()
        .queue(cursor::Show)
        .unwrap();
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
