pub fn total_area_union(shape_union: &[ShapeUnion]) -> f32 {
    let mut accum: f32 = 0.0;

    shape_union
        .iter()
        .for_each(|shape| accum += get_area_union(shape));

    accum
}

pub fn total_area_switch(shape_union: &[ShapeUnion]) -> f32 {
    let mut accum: f32 = 0.0;

    shape_union
        .iter()
        .for_each(|shape| accum += get_area_switch(shape));

    accum
}

const CTABLE: [f32; ShapeType::COUNT] = [1.0, 1.0, 0.5, std::f32::consts::PI];
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

pub fn total_area_vtbl<T>(shapes: &[T]) -> f32
where
    T: Shape,
{
    let mut accum: f32 = 0.0;

    shapes.iter().for_each(|shape| accum += shape.get_area());

    accum
}

pub fn total_area_rust(shapes: &[ShapeRustEnum]) -> f32 {
    let mut accum: f32 = 0.0;

    shapes.iter().for_each(|shape| accum += shape.get_area());

    accum
}

pub fn total_area_separate(shapes: (&[Circle], &[Square], &[Rectangle], &[Triangle])) -> f32 {
    total_area_vtbl(shapes.0) +
    total_area_vtbl(shapes.1) +
    total_area_vtbl(shapes.2) +
    total_area_vtbl(shapes.3)
}

#[derive(Debug, Copy, Clone)]
pub enum ShapeType {
    Square,
    Rectangle,
    Triangle,
    Circle,
    Count,
}

impl ShapeType {
    const COUNT: usize = 4;
}


#[derive(Debug, Clone)]
pub enum ShapeRustEnum {
    Square(Square),
    Rectangle(Rectangle),
    Triangle(Triangle),
    Circle(Circle),
}


#[derive(Debug, Clone)]
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

pub trait Shape {
    fn get_area(&self) -> f32;
}

impl Shape for ShapeRustEnum {
    fn get_area(&self) -> f32 {
        match self {
            ShapeRustEnum::Square(square) => square.get_area(),
            ShapeRustEnum::Rectangle(rectangle) => rectangle.get_area(),
            ShapeRustEnum::Triangle(triangle) => triangle.get_area(),
            ShapeRustEnum::Circle(circle) => circle.get_area(),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

impl Shape for Box<dyn Shape> {
    #[inline(always)]
    fn get_area(&self) -> f32 {
        (**self).get_area()
    }
}
