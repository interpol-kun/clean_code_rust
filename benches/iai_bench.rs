use iai::black_box;

use clean_code::shapes::{
    total_area_rust, total_area_switch, total_area_vtbl, Circle, Rectangle, Shape, ShapeRustEnum,
    ShapeUnion, Square, Triangle,
};
use clean_code::shapes::{total_area_union, ShapeType};

fn iai_vtbl() -> f32 {
    let circle = Circle::new(3.0);
    let square = Square::new(2.0);
    let triangle = Triangle::new(3.0, 2.0);
    let rectangle = Rectangle::new(4.0, 2.0);

    let shapes: Vec<&dyn Shape> = vec![&circle, &square, &triangle, &rectangle];

    total_area_vtbl(&black_box::<Vec<&dyn Shape>>(shapes))
}

fn iai_area_switch() -> f32 {
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

fn iai_switch_and_coefficient_array() -> f32 {
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

    total_area_union(&black_box::<Vec<&ShapeUnion>>(shapes_union))
}

fn iai_rust_enums() -> f32 {
    let cc_rust = ShapeRustEnum::Circle(Circle::new(3.0));
    let ss_rust = ShapeRustEnum::Square(Square::new(4.0));
    let rr_rust = ShapeRustEnum::Rectangle(Rectangle::new(2.0, 4.0));
    let tt_rust = ShapeRustEnum::Triangle(Triangle::new(5.0, 3.0));
    let shapes_rust_enum: Vec<&ShapeRustEnum> = vec![&cc_rust, &ss_rust, &rr_rust, &tt_rust];

    total_area_rust(&black_box::<Vec<&ShapeRustEnum>>(shapes_rust_enum))
}

iai::main!(
    iai_vtbl,
    iai_area_switch,
    iai_switch_and_coefficient_array,
    iai_rust_enums
);
