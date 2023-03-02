use clean_code::shapes::ShapeType;
use clean_code::shapes::{
    total_area_rust, total_area_switch, total_area_union, total_area_vtbl, Circle, Rectangle,
    Shape, ShapeRustEnum, ShapeUnion, Square, Triangle,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("reduced-sample-size");
    group
        .sample_size(30)
        .measurement_time(std::time::Duration::from_secs(45));

    const ARRAY_SIZE: u32 = 100_000;

    let circle = Circle::new(3.0);
    let square = Square::new(2.0);
    let triangle = Triangle::new(3.0, 2.0);
    let rectangle = Rectangle::new(4.0, 2.0);
    let mut shapes: Vec<&dyn Shape> = Vec::new();

    for _ in 0..ARRAY_SIZE {
        shapes.push(&circle);
        shapes.push(&square);
        shapes.push(&triangle);
        shapes.push(&rectangle);
    }

    let circle_union = ShapeUnion::new(ShapeType::Circle, 3.0, 3.0);
    let square_union = ShapeUnion::new(ShapeType::Square, 3.0, 2.0);
    let rectangle_union = ShapeUnion::new(ShapeType::Rectangle, 3.0, 5.0);
    let triangle_union = ShapeUnion::new(ShapeType::Triangle, 7.0, 2.0);
    let mut shapes_union = Vec::new();

    for _ in 0..ARRAY_SIZE {
        shapes_union.push(&circle_union);
        shapes_union.push(&triangle_union);
        shapes_union.push(&rectangle_union);
        shapes_union.push(&square_union);
    }

    let cc_rust = ShapeRustEnum::Circle(Circle::new(3.0));
    let ss_rust = ShapeRustEnum::Square(Square::new(4.0));
    let rr_rust = ShapeRustEnum::Rectangle(Rectangle::new(2.0, 4.0));
    let tt_rust = ShapeRustEnum::Triangle(Triangle::new(5.0, 3.0));
    let mut shapes_rust_enum: Vec<&ShapeRustEnum> = Vec::new();

    for _ in 0..ARRAY_SIZE {
        shapes_rust_enum.push(&cc_rust);
        shapes_rust_enum.push(&ss_rust);
        shapes_rust_enum.push(&rr_rust);
        shapes_rust_enum.push(&tt_rust);
    }

    group.bench_function("Dynamic dispatch (VTBL)", |b| {
        b.iter(|| total_area_vtbl(black_box(&shapes)))
    });

    group.bench_function("GS with switch", |b| {
        b.iter(|| total_area_switch(black_box(&shapes_union)))
    });

    group.bench_function("GS with switch and lookup table", |b| {
        b.iter(|| total_area_union(black_box(&shapes_union)))
    });

    group.bench_function("Rust enums", |b| {
        b.iter(|| total_area_rust(black_box(&shapes_rust_enum)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
