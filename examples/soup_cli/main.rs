use alphabet_soup::{generate_alphabet_soup, GenerationType, GeneratorSettings};

use colored::Colorize;

use rand::{
    rngs::{SmallRng, ThreadRng},
    thread_rng, Rng, RngCore, SeedableRng,
};
use rand_distr::{Distribution, Poisson};

use std::{env, process};

extern crate alphabet_soup;

#[cfg(all(feature = "f64"))]
pub type Float = f64;
#[cfg(not(feature = "f64"))]
pub type Float = f32;

enum Distributions {
    Poisson(Poisson<Float>),
}

impl Distribution<Float> for Distributions {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Float {
        match self {
            Self::Poisson(dist) => dist.sample(rng),
        }
    }
}

enum Rngs {
    SmallRng(SmallRng),
    ThreadRng(ThreadRng),
}

impl RngCore for Rngs {
    fn next_u32(&mut self) -> u32 {
        match self {
            Rngs::SmallRng(rng) => rng.next_u32(),
            Rngs::ThreadRng(rng) => rng.next_u32(),
        }
    }

    fn next_u64(&mut self) -> u64 {
        match self {
            Rngs::SmallRng(rng) => rng.next_u64(),
            Rngs::ThreadRng(rng) => rng.next_u64(),
        }
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        match self {
            Rngs::SmallRng(rng) => rng.fill_bytes(dest),
            Rngs::ThreadRng(rng) => rng.fill_bytes(dest),
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        match self {
            Rngs::SmallRng(rng) => rng.try_fill_bytes(dest),
            Rngs::ThreadRng(rng) => rng.try_fill_bytes(dest),
        }
    }
}

struct ApplicationSettings {
    generator_settings: GeneratorSettings<Rngs, Distributions>,
    word_highlighting: bool,
}

impl ApplicationSettings {
    pub const fn new(
        generator_settings: GeneratorSettings<Rngs, Distributions>,
        word_highlighting: bool,
    ) -> Self {
        Self {
            generator_settings,
            word_highlighting,
        }
    }
}

fn print_help_and_exit() {
    println!("Usage soup_cli [OPTION...]");
    println!("There will be stuff here in the future");
    process::exit(0);
}

fn handle_parameters(args: Vec<String>) -> ApplicationSettings {
    if args.is_empty() {
        print_help_and_exit();
    }

    let mut gen_type = GenerationType::WordCount(25);
    let mut rng = Rngs::SmallRng(SmallRng::from_entropy());
    let dist = Distributions::Poisson(Poisson::new(5.8).unwrap());
    let mut word_hightlighting = false;

    for mut i in 0..args.len() {
        match &args[i].to_lowercase()[..] {
            "-h" | "--help" => {
                print_help_and_exit();
            }
            "-w" | "--words" => {
                i += 1;

                gen_type = GenerationType::WordCount(
                    args.get(i)
                        .expect("Word count not specified!")
                        .parse()
                        .expect("Failed to parse word count!"),
                );
            }
            "-l" | "--letters" => {
                i += 1;

                gen_type = GenerationType::LetterCount(
                    args.get(i)
                        .expect("Letter count not specified!")
                        .parse()
                        .expect("Failed to parse letter count!"),
                );
            }
            "-ls" | "--letterstrict" => {
                i += 1;

                gen_type = GenerationType::LetterCountStrict(
                    args.get(i)
                        .expect("Letter count not specified!")
                        .parse()
                        .expect("Failed to parse letter count!"),
                );
            }
            "-c" | "--colourwords" => {
                word_hightlighting = true;
            }
            "-r" | "--rng" => {
                i += 1;

                rng = match &args
                    .get(i)
                    .expect("Rng not specified!")
                    .to_ascii_lowercase()[..]
                {
                    "smallrng" | "small_rng" => Rngs::SmallRng(SmallRng::from_entropy()),
                    "threadrng" | "thread_rng" => Rngs::ThreadRng(thread_rng()),
                    _ => panic!("Unknown rng!"),
                };
            }
            _ => {}
        }
    }

    ApplicationSettings::new(
        GeneratorSettings::new(gen_type, rng, dist),
        word_hightlighting,
    )
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    let mut app_settings = handle_parameters(args);

    let soup = generate_alphabet_soup(&mut app_settings.generator_settings);

    for word in soup.split_whitespace() {
        if webster::dictionary(word.clone()).is_some() && app_settings.word_highlighting {
            print!(
                "{} ",
                match word.len() {
                    1 => word.yellow(),
                    2 | 3 => word.green(),
                    4 | 5 => word.red(),
                    6 | 7 => word.cyan(),
                    _ => word.purple(),
                }
            );
        } else {
            print!("{} ", word);
        }
    }
    println!("");
}
