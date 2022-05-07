use alphabet_soup::{generate_alphabet_soup, GenerationType, GeneratorSettings};

use colored::Colorize;

use rand::{rngs::SmallRng, Rng, SeedableRng};
use rand_distr::{Distribution, Poisson};

use std::process;

extern crate alphabet_soup;

#[cfg(all(feature = "f64"))]
pub type Float = f64;
#[cfg(not(feature = "f64"))]
pub type Float = f32;

struct ApplicationSettings<R: Rng, D: Distribution<Float>> {
    generator_settings: GeneratorSettings<R, D>,
    word_highlighting: bool,
}

impl<R: Rng, D: Distribution<Float>> ApplicationSettings<R, D> {
    pub fn new(generator_settings: GeneratorSettings<R, D>, word_highlighting: bool) -> Self {
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

fn handle_parameters(args: Vec<String>) -> ApplicationSettings<SmallRng, Poisson<Float>> {
    if args.is_empty() {
        print_help_and_exit();
    }

    let mut gen_type = GenerationType::WordCount(25);
    let rng = SmallRng::from_entropy();
    let dist = Poisson::new(5.8).unwrap();
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
            "-c" | "colourwords" => {
                word_hightlighting = true;
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
    let args: Vec<String> = std::env::args().collect::<Vec<String>>()[1..].to_vec();
    let mut app_settings = handle_parameters(args);

    let soup = generate_alphabet_soup(&mut app_settings.generator_settings);

    for word in soup.split_whitespace() {
        if webster::dictionary(word.clone()).is_some() && app_settings.word_highlighting {
            match word.len() {
                1 => {
                    print!("{} ", word.yellow());
                }
                2 | 3 => {
                    print!("{} ", word.green())
                }
                4 | 5 => {
                    print!("{} ", word.red());
                }
                6 | 7 => {
                    print!("{} ", word.cyan());
                }
                _ => {
                    print!("{} ", word.purple())
                }
            }
        } else {
            print!("{} ", word);
        }
    }
}
