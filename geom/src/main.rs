use geom::Square;
use geom::Rectangle;
use geom::Diamond;
use geom::Triangle;
use geom::Circle;
use geom::print_area;

fn main() {
    let rect : geom::Rectangle = geom::Rectangle{width : 4.0, height: 5.0};
    let squr : geom::Square = geom::Square{edge : 4.0};
    let dia : geom::Diamond = geom::Diamond{diag1:4.0, diag2:5.0};
    let tri : geom::Triangle = geom::Triangle{base: 4.0, height:5.0};
    let circle : geom::Circle = geom::Circle{radius:5.0};
    print_area(&rect);
    print_area(&squr);
    print_area(&dia);
    print_area(&tri);
    print_area(&circle);
}


