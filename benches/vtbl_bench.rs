use std::vec;

use clean_code::shapes::{
    total_area_rust, total_area_rust_trait, total_area_switch, total_area_union, total_area_vtbl,
    Circle, CircleRust, Rectangle, RectangleRust, Shape, ShapeRust, ShapeRustTrait, ShapeUnion,
    Square, SquareRust, Triangle, TriangleRust,
};
use criterion::{criterion_group, criterion_main, Criterion};

use clean_code::shapes::ShapeType;

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

    let c_rust = ShapeRust::Circle(CircleRust::new(3.0));
    let s_rust = ShapeRust::Square(SquareRust::new(4.0));
    let r_rust = ShapeRust::Rectangle(RectangleRust::new(2.0, 4.0));
    let t_rust = ShapeRust::Triangle(TriangleRust::new(5.0, 3.0));
    let mut shapes_rust: Vec<&ShapeRust> = Vec::new();

    for _ in 0..ARRAY_SIZE {
        shapes_rust.push(&c_rust);
        shapes_rust.push(&s_rust);
        shapes_rust.push(&r_rust);
        shapes_rust.push(&t_rust);
    }

    let cc_rust = ShapeRustTrait::Circle(Circle::new(3.0));
    let ss_rust = ShapeRustTrait::Square(Square::new(4.0));
    let rr_rust = ShapeRustTrait::Rectangle(Rectangle::new(2.0, 4.0));
    let tt_rust = ShapeRustTrait::Triangle(Triangle::new(5.0, 3.0));
    let mut shapes_rust_trait: Vec<&ShapeRustTrait> = Vec::new();

    for _ in 0..ARRAY_SIZE {
        shapes_rust_trait.push(&cc_rust);
        shapes_rust_trait.push(&ss_rust);
        shapes_rust_trait.push(&rr_rust);
        shapes_rust_trait.push(&tt_rust);
    }

    group.bench_function("Dynamic dispatch (VTBL)", |b| {
        b.iter(|| total_area_vtbl(&shapes))
    });

    group.bench_function("GS with switch", |b| {
        b.iter(|| total_area_switch(&shapes_union))
    });

    group.bench_function("GS with switch and coefficient array", |b| {
        b.iter(|| total_area_union(&shapes_union))
    });

    group.bench_function(
        "Rust enums (implemented method from trait w direct call)",
        |b| b.iter(|| total_area_rust_trait(&shapes_rust_trait)),
    );

    group.bench_function("Rust enums (struct method w direct call)", |b| {
        b.iter(|| total_area_rust(&shapes_rust))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
