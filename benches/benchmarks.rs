use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::VecDeque;

fn split_digits_vec_deque(d: usize) -> Vec<u8> {
    let mut digits = VecDeque::new();
    let mut n = d;
    while n > 0 {
        digits.push_front((n % 10) as u8);
        n /= 10;
    }

    digits.into()
}

fn split_digits_vec(d: usize) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut n = d;
    while n > 0 {
        digits.insert(0, (n % 10) as u8);
        n /= 10;
    }

    digits
}

fn split_digits_vec_push_reverse(d: usize) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut n = d;
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }

    digits.reverse();

    digits
}

fn split_digits(mut num: usize) -> impl Iterator<Item = usize> {
    let mut divisor = 1;
    while num >= divisor * 10 {
        divisor *= 10;
    }

    std::iter::from_fn(move || {
        if divisor == 0 {
            None
        } else {
            let v = num / divisor;
            num %= divisor;
            divisor /= 10;
            Some(v)
        }
    })
}

fn split_digits_iterator(d: usize) -> Vec<u8> {
    split_digits(d).map(|x| x as u8).collect::<Vec<u8>>()
}

fn benchmark_split_digits(c: &mut Criterion) {
    let mut group = c.benchmark_group("split_digits");
    for (d, s) in [
        (1, 1),
        (12, 2),
        (123, 3),
        (1234, 4),
        (12345, 5),
        (123456, 6),
        (1234567, 7),
    ]
    .iter()
    {
        group.bench_with_input(BenchmarkId::new("split_digits_vec_deque", s), d, |b, d| {
            b.iter(|| split_digits_vec_deque(black_box(*d)))
        });
        group.bench_with_input(BenchmarkId::new("split_digits_vec", s), d, |b, d| {
            b.iter(|| split_digits_vec(black_box(*d)))
        });
        group.bench_with_input(
            BenchmarkId::new("split_digits_vec_push_reverse", s),
            d,
            |b, d| b.iter(|| split_digits_vec_push_reverse(black_box(*d))),
        );
        group.bench_with_input(BenchmarkId::new("split_digits_iterator", s), d, |b, d| {
            b.iter(|| split_digits_iterator(black_box(*d)))
        });
    }

    group.finish()
}

fn transform_char_if(c: char) -> String {
    if c.is_alphabetic() {
        return c.to_string();
    } else if c == '\'' {
        return "".to_string();
    }

    " ".to_string()
}

fn transform_char_match(c: char) -> String {
    match c {
        'a'..='z' | 'A'..='Z' => c.to_string(),
        '\'' => "".to_string(),
        _ => " ".to_string(),
    }
}

fn benchmark_transform_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform_char");
    for c in ['a', 'h', 'G', '\'', '9'].iter() {
        group.bench_with_input(BenchmarkId::new("transform_char_if", c), c, |b, c| {
            b.iter(|| transform_char_if(black_box(*c)))
        });
        group.bench_with_input(BenchmarkId::new("transform_char_match", c), c, |b, c| {
            b.iter(|| transform_char_match(black_box(*c)))
        });
    }

    group.finish()
}

criterion_group!(benches, benchmark_split_digits, benchmark_transform_char);
criterion_main!(benches);
