use criterion::{black_box, criterion_group, criterion_main, Criterion};
use druid::{
	kurbo::{PathEl, Shape},
	Point,
};
use rust_lipuma::render_objects::{fractal_line::FractalNoise, FractalLine};

fn simulate_fractal_noise(n: u32) -> Vec<PathEl> {
	let mut path_el = Vec::new();
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
			path_el.push(point);
		}
	}
	path_el
}

fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("fractal_noise_object 500", |b| {
		b.iter(|| simulate_fractal_noise(black_box(500)))
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
