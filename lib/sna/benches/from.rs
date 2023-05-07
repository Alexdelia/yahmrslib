use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sna::{NumArr, Sign, Signed, Unsigned};

fn from_signed_iter(n: i32) -> NumArr<Signed> {
    let mut arr = Vec::new();
    let mut n = n;
    let mut sign = Sign::Pos;

    if n < 0 {
        sign = Sign::Neg;
        n = -n;
    }

    while n > 0 {
        arr.push((n % 10) as u8);
        n /= 10;
    }

    NumArr {
        arr,
        sign: Signed(sign),
    }
}

fn from_signed_str(n: i32) -> NumArr<Signed> {
    let s = n.to_string();
    let mut sign = Sign::Pos;
    let mut i = 0;

    if s.starts_with('-') {
        sign = Sign::Neg;
        i = 1;
    }

    NumArr {
        arr: s[i..]
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect(),
        sign: Signed(sign),
    }
}

fn from_signed_iter_bench(c: &mut Criterion) {
    c.bench_function("from_signed_iter", |b| {
        b.iter(|| from_signed_iter(black_box(42)))
    });
}

fn from_signed_str_bench(c: &mut Criterion) {
    c.bench_function("from_signed_str", |b| {
        b.iter(|| from_signed_str(black_box(42)))
    });
}

criterion_group!(benches, from_signed_iter_bench, from_signed_str_bench);
criterion_main!(benches);
