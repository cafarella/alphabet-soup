use alphabet_soup::{generate_alphabet_soup, GenerationType, GeneratorSettings};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use rand::{rngs::SmallRng, SeedableRng};
use rand_distr::Poisson;

pub fn throughput(c: &mut Criterion) {
    let sizes = [1, 10, 100, 1000, 10_000, 100_000];
    let mut gen_settings = GeneratorSettings::new(
        GenerationType::LetterCount(0),
        SmallRng::from_entropy(),
        Poisson::new(5.8).unwrap(),
    );

    let mut group = c.benchmark_group("alphabet soup throughput (Poisson: 5.8)");
    for size in sizes.iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                gen_settings.gen_type = GenerationType::LetterCount(size);

                generate_alphabet_soup(&mut gen_settings);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, throughput);
criterion_main!(benches);
