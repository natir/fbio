/*
Copyright (c) 2022 Pierre Marijon <pierre@marijon.fr>

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

type Kmer = kmers::kmer::Kmer<u32, 11, 1>;

fn fast_encoding(c: &mut Criterion) {
    let mut g = c.benchmark_group("encoding");

    let mut rng = rand::thread_rng();
    let nucs = [b'A', b'C', b'T', b'G', b'a', b'c', b't', b'g'];

    for i in 5..16 {
        let len = 1 << i;

        let seq = (0..len)
            .map(|_| *nucs.choose(&mut rng).unwrap())
            .collect::<Vec<u8>>();

        g.bench_with_input(BenchmarkId::new("ACTG", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::ACTG)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("ACGT", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::ACGT)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("ATCG", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::ATCG)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("ATGC", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::ATGC)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("AGCT", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::AGCT)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("AGTC", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::AGTC)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("CATG", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::CATG)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("CAGT", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::CAGT)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("CTAG", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::CTAG)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("CTGA", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::CTGA)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("CGAT", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::CGAT)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("CGTA", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::CGTA)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("TACG", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::TACG)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("TAGC", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::TAGC)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("TCAG", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::TCAG)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("TCGA", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::TCGA)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("TGAC", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::TGAC)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("TGCA", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::TGCA)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("GACT", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::GACT)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("GATC", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::GATC)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("GCAT", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::GCAT)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("GCTA", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::GCTA)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("GTAC", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::GTAC)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });

        g.bench_with_input(BenchmarkId::new("GTCA", len), &seq, |b, seq| {
            b.iter(|| {
                black_box(
                    seq.windows(11)
                        .map(|x| black_box(Kmer::new(x, &kmers::encoding::Naive::GTCA)))
                        .collect::<Vec<Kmer>>(),
                )
            })
        });
    }
}

fn setup(c: &mut Criterion) {
    fast_encoding(c);
}

criterion_group!(benches, setup);

criterion_main!(benches);
