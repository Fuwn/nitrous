// Copyright (C) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

#[macro_use]
extern crate criterion;

use criterion::Criterion;
use rand::Rng;

fn bench_generate(c: &mut Criterion) {
  c.bench_function("generate", |b| {
    b.iter(|| {
      rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>()
    })
  });
}

criterion_group!(benches, bench_generate);
criterion_main!(benches);
