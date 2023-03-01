#[derive(Copy, Clone)]
pub enum ShapeType {
    Square,
    Rectangle,
    Triangle,
    Circle,
    Count,
}

pub enum ShapeRust {
    Square(SquareRust),
    Rectangle(RectangleRust),
    Triangle(TriangleRust),
    Circle(CircleRust),
}

pub enum ShapeRustTrait {
    Square(Square),
    Rectangle(Rectangle),
    Triangle(Triangle),
    Circle(Circle),
}

pub struct ShapeUnion {
    shape_type: ShapeType,
    width: f32,
    height: f32,
}

impl ShapeUnion {
    pub fn new(shape_type: ShapeType, width: f32, height: f32) -> ShapeUnion {
        ShapeUnion {
            shape_type,
            width,
            height,
        }
    }
}

pub fn total_area_union(shape_union: &[&ShapeUnion]) -> f32 {
    let mut accum: f32 = 0.0;

    for shape in shape_union {
        accum += get_area_union(shape);
    }

    accum
}

pub fn total_area_switch(shape_union: &[&ShapeUnion]) -> f32 {
    let mut accum: f32 = 0.0;

    for shape in shape_union {
        accum += get_area_switch(shape);
    }

    accum
}

const CTABLE: [f32; ShapeType::Count as usize] = [1.0, 1.0, 0.5, std::f32::consts::PI];
fn get_area_union(shape: &ShapeUnion) -> f32 {
    CTABLE[shape.shape_type as usize] * shape.width * shape.height
}

fn get_area_switch(shape: &ShapeUnion) -> f32 {
    match shape.shape_type {
        ShapeType::Square => shape.width * shape.width,
        ShapeType::Rectangle => shape.width * shape.height,
        ShapeType::Triangle => 0.5f32 * shape.width * shape.height,
        ShapeType::Circle => std::f32::consts::PI * shape.width * shape.width,
        _ => panic!("Not implemented"),
    }
}

pub fn total_area_vtbl<T>(shapes: &[&T]) -> f32
where
    T: Shape + ?Sized,
{
    let mut accum: f32 = 0.0;

    for shape in shapes {
        accum += shape.get_area();
    }

    accum
}

pub fn total_area_rust(shapes: &[&ShapeRust]) -> f32 {
    let mut accum: f32 = 0.0;

    for shape in shapes {
        accum += match shape {
            ShapeRust::Square(square) => square.get_area(),
            ShapeRust::Rectangle(rectangle) => rectangle.get_area(),
            ShapeRust::Triangle(triangle) => triangle.get_area(),
            ShapeRust::Circle(circle) => circle.get_area(),
        }
    }

    accum
}

pub fn total_area_rust_trait(shapes: &[&ShapeRustTrait]) -> f32 {
    let mut accum: f32 = 0.0;

    for shape in shapes {
        accum += match shape {
            ShapeRustTrait::Square(square) => square.get_area(),
            ShapeRustTrait::Rectangle(rectangle) => rectangle.get_area(),
            ShapeRustTrait::Triangle(triangle) => triangle.get_area(),
            ShapeRustTrait::Circle(circle) => circle.get_area(),
        }
    }

    accum
}

pub trait Shape {
    fn get_area(&self) -> f32;
}

pub struct Square {
    side: f32,
}

impl Square {
    pub fn new(side: f32) -> Square {
        Square { side }
    }
}

impl Shape for Square {
    fn get_area(&self) -> f32 {
        self.side * self.side
    }
}

pub struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Rectangle {
        Rectangle { width, height }
    }
}

impl Shape for Rectangle {
    fn get_area(&self) -> f32 {
        self.height * self.width
    }
}

pub struct Triangle {
    base: f32,
    height: f32,
}

impl Triangle {
    pub fn new(base: f32, height: f32) -> Triangle {
        Triangle { base, height }
    }
}

impl Shape for Triangle {
    fn get_area(&self) -> f32 {
        0.5f32 * self.base * self.height
    }
}

pub struct Circle {
    radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Circle {
        Circle { radius }
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}

pub struct SquareRust {
    side: f32,
}

impl SquareRust {
    pub fn new(side: f32) -> SquareRust {
        SquareRust { side }
    }

    pub fn get_area(&self) -> f32 {
        self.side * self.side
    }
}

pub struct RectangleRust {
    width: f32,
    height: f32,
}

impl RectangleRust {
    pub fn new(width: f32, height: f32) -> RectangleRust {
        RectangleRust { width, height }
    }
    pub fn get_area(&self) -> f32 {
        self.height * self.width
    }
}

pub struct TriangleRust {
    base: f32,
    height: f32,
}

impl TriangleRust {
    pub fn new(base: f32, height: f32) -> TriangleRust {
        TriangleRust { base, height }
    }

    pub fn get_area(&self) -> f32 {
        0.5f32 * self.base * self.height
    }
}

pub struct CircleRust {
    radius: f32,
}

impl CircleRust {
    pub fn new(radius: f32) -> CircleRust {
        CircleRust { radius }
    }
    pub fn get_area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}
