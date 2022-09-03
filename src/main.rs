extern crate core;

use clap::{Parser, ValueEnum};
use image::{ImageBuffer, Rgb};

const CODE: [[i8; 6]; 103] = [
    [2, 1, 2, 2, 2, 2],
    [2, 2, 2, 1, 2, 2],
    [2, 2, 2, 2, 2, 1],
    [1, 2, 1, 2, 2, 3],
    [1, 2, 1, 3, 2, 2],
    [1, 3, 1, 2, 2, 2],
    [1, 2, 2, 2, 1, 3],
    [1, 2, 2, 3, 1, 2],
    [1, 3, 2, 2, 1, 2],
    [2, 2, 1, 2, 1, 3],
    [2, 2, 1, 3, 1, 2],
    [2, 3, 1, 2, 1, 2],
    [1, 1, 2, 2, 3, 2],
    [1, 2, 2, 1, 3, 2],
    [1, 2, 2, 2, 3, 1],
    [1, 1, 3, 2, 2, 2],
    [1, 2, 3, 1, 2, 2],
    [1, 2, 3, 2, 2, 1],
    [2, 2, 3, 2, 1, 1],
    [2, 2, 1, 1, 3, 2],
    [2, 2, 1, 2, 3, 1],
    [2, 1, 3, 2, 1, 2],
    [2, 2, 3, 1, 1, 2],
    [3, 1, 2, 1, 3, 1],
    [3, 1, 1, 2, 2, 2],
    [3, 2, 1, 1, 2, 2],
    [3, 2, 1, 2, 2, 1],
    [3, 1, 2, 2, 1, 2],
    [3, 2, 2, 1, 1, 2],
    [3, 2, 2, 2, 1, 1],
    [2, 1, 2, 1, 2, 3],
    [2, 1, 2, 3, 2, 1],
    [2, 3, 2, 1, 2, 1],
    [1, 1, 1, 3, 2, 3],
    [1, 3, 1, 1, 2, 3],
    [1, 3, 1, 3, 2, 1],
    [1, 1, 2, 3, 1, 3],
    [1, 3, 2, 1, 1, 3],
    [1, 3, 2, 3, 1, 1],
    [2, 1, 1, 3, 1, 3],
    [2, 3, 1, 1, 1, 3],
    [2, 3, 1, 3, 1, 1],
    [1, 1, 2, 1, 3, 3],
    [1, 1, 2, 3, 3, 1],
    [1, 3, 2, 1, 3, 1],
    [1, 1, 3, 1, 2, 3],
    [1, 1, 3, 3, 2, 1],
    [1, 3, 3, 1, 2, 1],
    [3, 1, 3, 1, 2, 1],
    [2, 1, 1, 3, 3, 1],
    [2, 3, 1, 1, 3, 1],
    [2, 1, 3, 1, 1, 3],
    [2, 1, 3, 3, 1, 1],
    [2, 1, 3, 1, 3, 1],
    [3, 1, 1, 1, 2, 3],
    [3, 1, 1, 3, 2, 1],
    [3, 3, 1, 1, 2, 1],
    [3, 1, 2, 1, 1, 3],
    [3, 1, 2, 3, 1, 1],
    [3, 3, 2, 1, 1, 1],
    [3, 1, 4, 1, 1, 1],
    [2, 2, 1, 4, 1, 1],
    [4, 3, 1, 1, 1, 1],
    [1, 1, 1, 2, 2, 4],
    [1, 1, 1, 4, 2, 2],
    [1, 2, 1, 1, 2, 4],
    [1, 2, 1, 4, 2, 1],
    [1, 4, 1, 1, 2, 2],
    [1, 4, 1, 2, 2, 1],
    [1, 1, 2, 2, 1, 4],
    [1, 1, 2, 4, 1, 2],
    [1, 2, 2, 1, 1, 4],
    [1, 2, 2, 4, 1, 1],
    [1, 4, 2, 1, 1, 2],
    [1, 4, 2, 2, 1, 1],
    [2, 4, 1, 2, 1, 1],
    [2, 2, 1, 1, 1, 4],
    [4, 1, 3, 1, 1, 1],
    [2, 4, 1, 1, 1, 2],
    [1, 3, 4, 1, 1, 1],
    [1, 1, 1, 2, 4, 2],
    [1, 2, 1, 1, 4, 2],
    [1, 2, 1, 2, 4, 1],
    [1, 1, 4, 2, 1, 2],
    [1, 2, 4, 1, 1, 2],
    [1, 2, 4, 2, 1, 1],
    [4, 1, 1, 2, 1, 2],
    [4, 2, 1, 1, 1, 2],
    [4, 2, 1, 2, 1, 1],
    [2, 1, 2, 1, 4, 1],
    [2, 1, 4, 1, 2, 1],
    [4, 1, 2, 1, 2, 1],
    [1, 1, 1, 1, 4, 3],
    [1, 1, 1, 3, 4, 1],
    [1, 3, 1, 1, 4, 1],
    [1, 1, 4, 1, 1, 3],
    [1, 1, 4, 3, 1, 1],
    [4, 1, 1, 1, 1, 3],
    [4, 1, 1, 3, 1, 1],
    [1, 1, 3, 1, 4, 1],
    [1, 1, 4, 1, 3, 1],
    [3, 1, 1, 1, 4, 1],
    [4, 1, 1, 1, 3, 1],
];
const START_CODE_A: [i8; 6] = [2, 1, 1, 4, 1, 2];
const START_CODE_B: [i8; 6] = [2, 1, 1, 2, 1, 4];
const START_CODE_C: [i8; 6] = [2, 1, 1, 2, 3, 2];
const STOP_CODE: [i8; 7] = [2, 3, 3, 1, 1, 1, 2];

const CODE_A: &'static str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]\
^_\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\
\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f";
const CODE_B: &'static str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]\
^_`abcdefghijklmnopqrstuvwxyz{|}~\x7f";

#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[clap(value_parser)]
    query: String,
    #[clap(short, long, arg_enum, value_parser)]
    format: Format,
    #[clap(short, long, arg_enum, value_parser)]
    code: Code,
    #[clap(short, long, value_parser)]
    out: Option<String>,
}

#[derive(Clone, Eq, PartialEq, ValueEnum)]
enum Format {
    Text,
    Image,
}

#[derive(Clone, Eq, PartialEq, ValueEnum)]
enum Code {
    A,
    B,
    C,
}

fn main() {
    let cli = Cli::parse();
    if cli.format == Format::Image && cli.out.is_none() {
        panic!("Requires --out argument when format is image.");
    }
    let res = match build_table(&cli.query, &cli.code) {
        None => panic!("Receives unsupported characters."),
        Some(r) => r,
    };
    match cli.format {
        Format::Text => println!(
            "{}",
            res.iter()
                .fold(String::new(), |r, c| format!("{0}{1}", r, c))
        ),
        Format::Image => {
            match build_image(&res).save(cli.out.unwrap()) {
                Ok(_) => {}
                Err(e) => panic!("Failed to generate the image. {:?}", e),
            };
        }
    }
}

fn build_table(q: &str, code: &Code) -> Option<Vec<i8>> {
    let code = match code {
        Code::A => Code::A,
        Code::B => Code::B,
        Code::C => {
            if q.len() < 2 {
                Code::B
            } else {
                Code::C
            }
        }
    };
    let set = match code {
        Code::A => CODE_A,
        Code::B => CODE_B,
        Code::C => "",
    };
    let mut data: Vec<i8> = vec![];
    let mut cd = match code {
        Code::A => 103,
        Code::B => 104,
        Code::C => 105,
    };
    data.extend_from_slice(match code {
        Code::A => &START_CODE_A,
        Code::B => &START_CODE_B,
        Code::C => &START_CODE_C,
    });
    if code == Code::C {
        let chars = q.chars().collect::<Vec<char>>();
        for (i, c) in chars.chunks(2).enumerate() {
            if c.len() < 2 {
                // Switch to the Code B.
                data.extend_from_slice(&CODE[100]);
                cd += 100 * (i + 1);
                match CODE_B.find(c[0]) {
                    Some(j) => {
                        data.extend_from_slice(&CODE[j]);
                        cd += j * (i + 2);
                    }
                    None => return None,
                }
                continue;
            }
            let s = format!("{0}{1}", c[0], c[1]);
            match usize::from_str_radix(&s, 10) {
                Ok(j) => {
                    data.extend_from_slice(&CODE[j]);
                    cd += j * (i + 1);
                }
                Err(_) => return None,
            }
        }
    } else {
        for (i, c) in q.chars().enumerate() {
            match set.find(c) {
                None => return None,
                Some(j) => {
                    data.extend_from_slice(&CODE[j]);
                    cd += j * (i + 1);
                }
            };
        }
    }
    cd %= 103;
    data.extend_from_slice(&CODE[cd]);
    data.extend_from_slice(&STOP_CODE);
    return Some(data);
}

fn build_image(v: &Vec<i8>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = v.iter().map(|c| *c as u32).sum();
    let height = width / 2;
    let mut img = ImageBuffer::new(width, height);
    let mut black = true;
    let mut x: u32 = 0;
    for i in 0..v.len() {
        let color = if black {
            Rgb([0, 0, 0])
        } else {
            Rgb([255, 255, 255])
        };
        for _ in 0..v[i] {
            for y in 0..height {
                let pixel = img.get_pixel_mut(x, y);
                *pixel = color;
            }
            x += 1;
        }
        black = !black;
    }
    return img;
}
