use iai::black_box;

use clean_code::shapes::ShapeType;
use clean_code::shapes::{
    total_area_switch, total_area_vtbl, Circle, Rectangle, Shape, ShapeUnion, Square, Triangle,
};

fn iai_vtbl() -> f32 {
    let circle = Circle::new(3.0);
    let square = Square::new(2.0);
    let triangle = Triangle::new(3.0, 2.0);
    let rectangle = Rectangle::new(4.0, 2.0);

    let shapes: Vec<&dyn Shape> = vec![&circle, &square, &triangle, &rectangle];

    total_area_vtbl(&black_box::<Vec<&dyn Shape>>(shapes))
}

fn iai_benchmark_long() -> f32 {
    let circle_union = ShapeUnion::new(ShapeType::Circle, 3.0, 3.0);
    let square_union = ShapeUnion::new(ShapeType::Square, 3.0, 2.0);
    let rectangle_union = ShapeUnion::new(ShapeType::Rectangle, 3.0, 5.0);
    let triangle_union = ShapeUnion::new(ShapeType::Triangle, 7.0, 2.0);

    let shapes_union = vec![
        &circle_union,
        &square_union,
        &rectangle_union,
        &triangle_union,
    ];

    total_area_switch(&black_box::<Vec<&ShapeUnion>>(shapes_union))
}

iai::main!(iai_vtbl, iai_benchmark_long);
