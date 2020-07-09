/*
Copyright (c) 2020 Pierre Marijon <pmarijon@hhu.de>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use rand::seq::SliceRandom;

use fbio;

fn iter_cano(c: &mut Criterion) {
    for k in (5..21).step_by(2) {
        let mut g = c.benchmark_group(format!("iter_cano k={}", k));

        let mut rng = rand::thread_rng();
        let nucs = [b'A', b'C', b'T', b'G', b'a', b'c', b't', b'g'];

        for i in 5..16 {
            let len = 1 << i;

            let seq = (0..len)
                .map(|_| *nucs.choose(&mut rng).unwrap())
                .collect::<Vec<u8>>();

            g.bench_with_input(BenchmarkId::new("forward", len), &seq, |b, seq| {
                b.iter(|| {
                    cocktail::tokenizer::Tokenizer::new(black_box(&seq), black_box(k))
                        .map(|x| cocktail::kmer::canonical(x, k))
                        .collect::<Vec<u64>>()
                })
            });

            g.bench_with_input(BenchmarkId::new("forward_reverse", len), &seq, |b, seq| {
                b.iter(|| {
                    cocktail::tokenizer::Canonical::new(black_box(&seq), black_box(5))
                        .collect::<Vec<u64>>()
                })
            });

            g.bench_with_input(
                BenchmarkId::new("forward_reverse_lexi", len),
                &seq,
                |b, seq| {
                    b.iter(|| {
                        fbio::iter_cano::Lexi::new(black_box(&seq), black_box(5))
                            .collect::<Vec<u64>>()
                    })
                },
            );
        }
    }
}

fn setup(c: &mut Criterion) {
    iter_cano(c);
}

criterion_group!(benches, setup);

criterion_main!(benches);
