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

fn nuc2bit(c: &mut Criterion) {
    let mut g = c.benchmark_group("nuc2bit");

    g.sample_size(100);
    g.warm_up_time(std::time::Duration::from_secs(1));

    let mut rng = rand::thread_rng();
    let nucs = [b'A', b'C', b'T', b'G', b'a', b'c', b't', b'g'];

    for len in (10..100)
        .step_by(10)
        .chain((100..500).step_by(50))
        .chain((500..1000).step_by(100))
        .chain((1000..=20000).step_by(1000))
    {
        let seq = (0..len)
            .map(|_| *nucs.choose(&mut rng).unwrap())
            .collect::<Vec<u8>>();

        let up = &seq.to_ascii_uppercase();

        g.bench_with_input(BenchmarkId::new("move_mask", len), &seq, |b, seq| {
            b.iter(|| {
                let mut count = [0; 4];

                for nuc in seq.iter() {
                    count[fbio::nuc2bit::move_mask(*nuc) as usize] += 1;
                }

                black_box(count);
            })
        });

        g.bench_with_input(BenchmarkId::new("move_move", len), &seq, |b, seq| {
            b.iter(|| {
                let mut count = [0; 4];

                for nuc in seq.iter() {
                    count[fbio::nuc2bit::move_move(*nuc) as usize] += 1;
                }

                black_box(count);
            })
        });

        g.bench_with_input(BenchmarkId::new("test_match", len), &seq, |b, seq| {
            b.iter(|| {
                let mut count = [0; 4];

                for nuc in seq.iter() {
                    count[fbio::nuc2bit::test_match(*nuc) as usize] += 1;
                }

                black_box(count);
            })
        });

        g.bench_with_input(BenchmarkId::new("test_if", len), &seq, |b, seq| {
            b.iter(|| {
                let mut count = [0; 4];

                for nuc in seq.iter() {
                    count[fbio::nuc2bit::test_if(*nuc) as usize] += 1;
                }

                black_box(count);
            })
        });

        g.bench_with_input(BenchmarkId::new("test_match_upper", len), &seq, |b, seq| {
            b.iter(|| {
                let mut count = [0; 4];

                for nuc in seq.iter() {
                    count[fbio::nuc2bit::test_match_upper(*nuc) as usize] += 1;
                }

                black_box(count);
            })
        });

        g.bench_with_input(BenchmarkId::new("test_if_upper", len), &seq, |b, seq| {
            b.iter(|| {
                let mut count = [0; 4];

                for nuc in seq.iter() {
                    count[fbio::nuc2bit::test_if_upper(*nuc) as usize] += 1;
                }

                black_box(count);
            })
        });

        g.bench_with_input(BenchmarkId::new("lookup", len), &seq, |b, seq| {
            b.iter(|| {
                let mut count = [0; 4];

                for nuc in seq.iter() {
                    count[fbio::nuc2bit::lookup(*nuc) as usize] += 1;
                }

                black_box(count);
            })
        });

        g.bench_with_input(BenchmarkId::new("lookup_nocheck", len), &seq, |b, seq| {
            b.iter(|| {
                let mut count = [0; 4];

                for nuc in seq.iter() {
                    count[fbio::nuc2bit::lookup_nocheck(*nuc) as usize] += 1;
                }

                black_box(count);
            })
        });

        g.bench_with_input(BenchmarkId::new("group_vector16", len), &seq, |b, seq| {
            b.iter(|| {
                let mut count = [0; 4];

                for nuc in fbio::nuc2bit::GroupVec::<16>::new(seq) {
                    count[nuc as usize] += 1;
                }

                black_box(count);
            })
        });

        g.bench_with_input(BenchmarkId::new("group_vector32", len), &seq, |b, seq| {
            b.iter(|| {
                let mut count = [0; 4];

                for nuc in fbio::nuc2bit::GroupVec::<32>::new(seq) {
                    count[nuc as usize] += 1;
                }

                black_box(count);
            })
        });

        g.bench_with_input(BenchmarkId::new("group_phf", len), &up, |b, up| {
            b.iter(|| {
                let mut count = [0; 4];

                for nuc in fbio::nuc2bit::GroupPhf::new(up) {
                    count[nuc as usize] += 1;
                }

                black_box(count);
            })
        });

        g.bench_with_input(
            BenchmarkId::new("group_phf_with_upper", len),
            &seq,
            |b, seq| {
                b.iter(|| {
                    let mut count = [0; 4];

                    for nuc in fbio::nuc2bit::GroupPhf::new(&seq.to_ascii_uppercase()) {
                        count[nuc as usize] += 1;
                    }

                    black_box(count);
                })
            },
        );

        g.bench_with_input(BenchmarkId::new("ram_acces", len), &seq, |b, seq| {
            b.iter(|| {
                let mut count = [0; 4];

                for nuc in seq.iter() {
                    count[black_box(0)] += 1;
                }

                black_box(count);
            })
        });
    }
}

fn setup(c: &mut Criterion) {
    nuc2bit(c);
}

criterion_group!(benches, setup);

criterion_main!(benches);
