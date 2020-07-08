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
use rand::distributions::Distribution;

use fbio;

fn nuc2bit(c: &mut Criterion) {
    let mut g = c.benchmark_group("nuc2bit");

    g.sample_size(2000);
    g.warm_up_time(std::time::Duration::from_secs(1));

    let dist = rand::distributions::Uniform::from(0..8);
    let mut rng = rand::thread_rng();
    let nucleotides = [b'A', b'C', b'T', b'G', b'a', b'c', b't', b'g'];

    for i in 0..16 {
        let len = 1 << i;

        let seq: Vec<u8> = dist
            .sample_iter(&mut rng)
            .take(len)
            .map(|x| nucleotides[x])
            .collect();

        g.bench_with_input(BenchmarkId::new("move_mask", len), &seq, |b, seq| {
            b.iter(|| {
                for nuc in seq.iter() {
                    black_box(fbio::nuc2bit::move_mask(*nuc));
                }
            })
        });

        g.bench_with_input(BenchmarkId::new("move_move", len), &seq, |b, seq| {
            b.iter(|| {
                for nuc in seq.iter() {
                    black_box(fbio::nuc2bit::move_move(*nuc));
                }
            })
        });

        g.bench_with_input(BenchmarkId::new("test_match", len), &seq, |b, seq| {
            b.iter(|| {
                for nuc in seq.iter() {
                    black_box(fbio::nuc2bit::test_match(*nuc));
                }
            })
        });

        g.bench_with_input(BenchmarkId::new("test_if", len), &seq, |b, seq| {
            b.iter(|| {
                for nuc in seq.iter() {
                    black_box(fbio::nuc2bit::test_if(*nuc));
                }
            })
        });

        g.bench_with_input(BenchmarkId::new("test_match_upper", len), &seq, |b, seq| {
            b.iter(|| {
                for nuc in seq.iter() {
                    black_box(fbio::nuc2bit::test_match_upper(*nuc));
                }
            })
        });

        g.bench_with_input(BenchmarkId::new("test_if_upper", len), &seq, |b, seq| {
            b.iter(|| {
                for nuc in seq.iter() {
                    black_box(fbio::nuc2bit::test_if_upper(*nuc));
                }
            })
        });

        g.bench_with_input(BenchmarkId::new("lookup", len), &seq, |b, seq| {
            b.iter(|| {
                for nuc in seq.iter() {
                    black_box(fbio::nuc2bit::lookup(*nuc));
                }
            })
        });

        g.bench_with_input(BenchmarkId::new("lookup_nocheck", len), &seq, |b, seq| {
            b.iter(|| {
                for nuc in seq.iter() {
                    black_box(fbio::nuc2bit::lookup(*nuc));
                }
            })
        });
    }
}

fn setup(c: &mut Criterion) {
    nuc2bit(c);
}

criterion_group!(benches, setup);

criterion_main!(benches);
