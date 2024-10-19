use std::fs::OpenOptions;
use std::io::Write;
use liblzma::write::XzEncoder;

fn main() {
    let source = std::str::from_utf8(include_bytes!("src/source.txt")).unwrap();

    let data: Vec<u8> = source
        .split("\n\n")
        .flat_map(|frame| {
            frame.lines().map(|line| {
                line.chars()
                .collect::<Vec<char>>()
                .chunks(8).map(|chunk| {
                    let mut n: u8 = 0;
                    for c in chunk {
                        n <<= 1;
                        if *c == '#' {
                            n += 1;
                        }
                    };
                    n
                })
                .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>()
        })
        .flatten()
        .collect();

    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("src/source.bin")
        .unwrap();

    XzEncoder::new(file, 9)
        .write_all(&data)
        .unwrap();
}
