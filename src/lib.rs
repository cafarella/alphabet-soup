use num_traits::cast::NumCast;

use rand::Rng;
use rand_distr::Distribution;

use std::marker::PhantomData;

pub enum GenerationType {
    LetterCountStrict(usize),
    LetterCount(usize),
    WordCount(usize),
}

pub struct GeneratorSettings<R: Rng, D: Distribution<T>, T: NumCast> {
    pub gen_type: GenerationType,
    pub rng: R,
    pub length_distribution: D,
    pub phantom_data: PhantomData<T>,
}

impl<R: Rng, D: Distribution<T>, T: NumCast> GeneratorSettings<R, D, T> {
    pub const fn new(gen_type: GenerationType, rng: R, length_distribution: D) -> Self {
        Self {
            gen_type,
            rng,
            length_distribution,
            phantom_data: PhantomData,
        }
    }
}

pub fn generate_alphabet_soup<R: Rng, D: Distribution<T>, T: NumCast>(
    settings: &mut GeneratorSettings<R, D, T>,
) -> String {
    let mut soup: Vec<char> = Vec::new();

    let letters = |length, rng: &mut R| -> Vec<char> {
        (0..length)
            .map(|_| rng.gen_range(97..123u8) as char)
            .collect()
    };

    let get_word_length = |rng: &mut R, dist: &D| -> usize { dist.sample(rng).to_usize().unwrap() };

    match settings.gen_type {
        GenerationType::LetterCountStrict(letter_count) => {
            while soup.len() < letter_count {
                let diff = letter_count - soup.len();
                let word_length =
                    get_word_length(&mut settings.rng, &settings.length_distribution).max(diff);

                soup.extend(letters(word_length, &mut settings.rng));

                // The string can end in a space if make the string exactly letter_count long
                if word_length < diff {
                    soup.push(32 as char);
                }
            }
        }
        GenerationType::LetterCount(letter_count) => {
            while soup.len() < letter_count {
                let word_length = get_word_length(&mut settings.rng, &settings.length_distribution);

                soup.extend(letters(word_length, &mut settings.rng));

                if soup.len() < letter_count {
                    soup.push(32 as char);
                }
            }
        }
        GenerationType::WordCount(max_words) => {
            for i in 0..max_words {
                let word_length = get_word_length(&mut settings.rng, &settings.length_distribution);

                soup.extend(letters(word_length, &mut settings.rng));

                if i != max_words - 1 {
                    soup.push(32 as char);
                }
            }
        }
    }

    soup.into_iter().collect()
}
