use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use druid::{
	kurbo::{PathEl, Shape},
	Point,
};
use rust_lipuma::render_objects::{fractal_line::FractalNoise, FractalLine};

fn simulate_fractal_noise(n: u32) -> PathEl {
	let mut p = black_box(PathEl::ClosePath);
	let mut lines = Vec::new();
	for _ in 0..n {
		lines.push(FractalLine {
			start: Point::ZERO,
			end: Point::ZERO,
			noise: FractalNoise::new(n, 0.3, 3),
			width: 5.0,
			density: 0.5,
			samples: 500,
		})
	}
	for line in lines {
		for point in line.path_elements(0.3) {
			p = point;
		}
	}
	p
}

fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("fractal_noise_object 500", |b| {
		b.iter(|| simulate_fractal_noise(black_box(500)))
	});
}

criterion_group!(
	name = benches;
	config = Criterion::default().significance_level(0.01).measurement_time(Duration::from_secs(60));
	targets = criterion_benchmark
);
criterion_main!(benches);
