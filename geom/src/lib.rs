pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius : f64,
}

pub struct Square {
    pub edge : f64,
}

pub struct Diamond {
    pub diag1 : f64,
    pub diag2 : f64,
}

pub struct Rectangle {
    pub width : f64,
    pub height : f64,
}

pub struct Triangle {
    pub base : f64,
    pub height : f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 { 
        return std::f64::consts::PI * self.radius * self.radius 
    }
}

impl Shape for Square {
    fn area(&self) -> f64 { 
        return self.edge * self.edge
    }
}

impl Shape for Diamond {
    fn area(&self) -> f64 { 
        return (self.diag1 * self.diag2) / 2.0
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 { 
        return (self.base * self.height) / 2.0
    }
}

pub fn print_area<T: Shape>(s: &T) { 
    println!("Aire = {}", s.area()); 
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_rectangle() {
        let rect : Rectangle = Rectangle{width : 4.0, height: 5.0};
        assert!(rect.area() == 20.0);
    }

    #[test]
    fn area_square() {
        let squr : Square = Square{edge : 4.0};
        assert!(squr.area() == 16.0);
    }

    #[test]
    fn area_diamond() {
        let dia : Diamond = Diamond{diag1:4.0, diag2:5.0};
        assert!(dia.area() == 10.0);
    }

    #[test]
    fn area_triangle() {
        let tri : Triangle = Triangle{base: 4.0, height:5.0};
        assert!(tri.area() == 10.0);
    }

    /*#[test]
    fn area_circle() {
        let circle : Circle = Circle{5};
        assert(circle.area() == )
    }*/


}
