use clean_code::shapes::{
    total_area_rust, total_area_switch, total_area_union, total_area_vtbl, Circle, Rectangle,
    Shape, ShapeRustEnum, ShapeUnion, Square, Triangle,
};
use clean_code::shapes::{total_area_separate, ShapeType};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;
use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("reduced-sample-size");
    group
        .sample_size(30)
        .measurement_time(std::time::Duration::from_secs(45));

    const ARRAY_SIZE: usize = 100_000;

    let mut rng = rand::thread_rng();

    let circle = Box::new(Circle::new(3.0));
    let square = Box::new(Square::new(2.0));
    let triangle = Box::new(Triangle::new(3.0, 2.0));
    let rectangle = Box::new(Rectangle::new(4.0, 2.0));
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();

    for _ in 0..4 * ARRAY_SIZE {
        match rng.gen_range(0..4) {
            0 => shapes.push(circle.clone()),
            1 => shapes.push(square.clone()),
            2 => shapes.push(triangle.clone()),
            3 => shapes.push(rectangle.clone()),
            _ => unreachable!(),
        }
    }

    let mut stride_shapes: Vec<Box<dyn Shape>> = Vec::new();

    for _ in 0..ARRAY_SIZE {
        stride_shapes.push(circle.clone());
        stride_shapes.push(square.clone());
        stride_shapes.push(triangle.clone());
        stride_shapes.push(rectangle.clone());
    }

    let mut shuffled_shapes: Vec<Box<dyn Shape>> = Vec::new();

    for _ in 0..4 * ARRAY_SIZE {
        match rng.gen_range(0..4) {
            0 => shuffled_shapes.push(circle.clone()),
            1 => shuffled_shapes.push(square.clone()),
            2 => shuffled_shapes.push(triangle.clone()),
            3 => shuffled_shapes.push(rectangle.clone()),
            _ => unreachable!(),
        }
    }

    // shuffle to try and trick cache
    shuffled_shapes.shuffle(&mut rng);

    let circle_union = ShapeUnion::new(ShapeType::Circle, 3.0, 3.0);
    let square_union = ShapeUnion::new(ShapeType::Square, 3.0, 2.0);
    let rectangle_union = ShapeUnion::new(ShapeType::Rectangle, 3.0, 5.0);
    let triangle_union = ShapeUnion::new(ShapeType::Triangle, 7.0, 2.0);
    let mut shapes_union = Vec::new();

    for _ in 0..4 * ARRAY_SIZE {
        match rng.gen_range(0..4) {
            0 => shapes_union.push(circle_union.clone()),
            1 => shapes_union.push(square_union.clone()),
            2 => shapes_union.push(triangle_union.clone()),
            3 => shapes_union.push(rectangle_union.clone()),
            _ => unreachable!(),
        }
    }

    let mut stride_union = Vec::new();

    for _ in 0..ARRAY_SIZE {
        stride_union.push(circle_union.clone());
        stride_union.push(square_union.clone());
        stride_union.push(triangle_union.clone());
        stride_union.push(rectangle_union.clone());
    }

    let cc_rust = ShapeRustEnum::Circle(Circle::new(3.0));
    let ss_rust = ShapeRustEnum::Square(Square::new(4.0));
    let rr_rust = ShapeRustEnum::Rectangle(Rectangle::new(2.0, 4.0));
    let tt_rust = ShapeRustEnum::Triangle(Triangle::new(5.0, 3.0));
    let mut shapes_rust_enum: Vec<ShapeRustEnum> = Vec::new();

    for _ in 0..4 * ARRAY_SIZE {
        match rng.gen_range(0..4) {
            0 => shapes_rust_enum.push(cc_rust.clone()),
            1 => shapes_rust_enum.push(ss_rust.clone()),
            2 => shapes_rust_enum.push(rr_rust.clone()),
            3 => shapes_rust_enum.push(tt_rust.clone()),
            _ => unreachable!(),
        }
    }

    let mut stride_enum = Vec::new();

    for _ in 0..ARRAY_SIZE {
        stride_enum.push(cc_rust.clone());
        stride_enum.push(ss_rust.clone());
        stride_enum.push(rr_rust.clone());
        stride_enum.push(tt_rust.clone());
    }

    let circles = vec![Circle::new(3.0); ARRAY_SIZE];
    let squares = vec![Square::new(4.0); ARRAY_SIZE];
    let rectangles = vec![Rectangle::new(2.0, 4.0); ARRAY_SIZE];
    let triangles = vec![Triangle::new(5.0, 3.0); ARRAY_SIZE];

    group.bench_function("Dynamic dispatch (VTBL)", |b| {
        b.iter(|| total_area_vtbl(black_box(&shapes)))
    });

    group.bench_function("Dynamic dispatch (VTBL) strided data", |b| {
        b.iter(|| total_area_vtbl(black_box(&stride_shapes)))
    });

    group.bench_function("Dynamic dispatch (VTBL) shuffled", |b| {
        b.iter(|| total_area_vtbl(black_box(&shuffled_shapes)))
    });

    group.bench_function("GS with switch", |b| {
        b.iter(|| total_area_switch(black_box(&shapes_union)))
    });

    group.bench_function("GS with switch, strided data", |b| {
        b.iter(|| total_area_switch(black_box(&stride_union)))
    });

    group.bench_function("GS with switch and lookup table", |b| {
        b.iter(|| total_area_union(black_box(&shapes_union)))
    });

    group.bench_function("GS with switch and lookup table, strided data", |b| {
        b.iter(|| total_area_union(black_box(&stride_union)))
    });

    group.bench_function("Rust enums", |b| {
        b.iter(|| total_area_rust(black_box(&shapes_rust_enum)))
    });

    group.bench_function("Rust enums, strided data", |b| {
        b.iter(|| total_area_rust(black_box(&stride_enum)))
    });

    group.bench_function("Separate data", |b| {
        b.iter(|| total_area_separate(black_box((&circles, &squares, &rectangles, &triangles))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
