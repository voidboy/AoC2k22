use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day7::{part1, part2};

const INPUT: &str = include_str!("../input");
pub fn criterion_benchmark_part1(c: &mut Criterion) {
    c.bench_function("day7part1", |b| b.iter(|| part1(black_box(INPUT))));
}

pub fn criterion_benchmark_part2(c: &mut Criterion) {
    c.bench_function("day7part2", |b| b.iter(|| part2(black_box(INPUT))));
}

criterion_group!(benches, criterion_benchmark_part1, criterion_benchmark_part2);
criterion_main!(benches);
