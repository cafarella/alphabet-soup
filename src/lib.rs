use rand::Rng;
use rand_distr::Distribution;

#[cfg(all(feature = "f64"))]
pub type Float = f64;
#[cfg(not(feature = "f64"))]
pub type Float = f32;

pub enum GenerationType {
    LetterCountStrict(usize),
    LetterCount(usize),
    WordCount(usize),
}

pub struct GeneratorSettings<R: Rng, D: Distribution<Float>> {
    pub gen_type: GenerationType,
    pub rng: R,
    pub length_distribution: D,
}

impl<R: Rng, D: Distribution<Float>> GeneratorSettings<R, D> {
    pub const fn new(gen_type: GenerationType, rng: R, length_distribution: D) -> Self {
        Self {
            gen_type,
            rng,
            length_distribution,
        }
    }
}

pub fn generate_alphabet_soup<R: Rng, D: Distribution<Float>>(
    settings: &mut GeneratorSettings<R, D>,
) -> String {
    let mut soup: Vec<char> = Vec::new();

    let letters = |length, rng: &mut R| -> Vec<char> {
        (0..length)
            .map(|_| rng.gen_range(97..123u8) as char)
            .collect()
    };

    let get_word_length = |rng: &mut R, dist: &D| dist.sample(rng) as usize;

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
