struct Circle;
struct Triangle;

trait Figure {
    fn print(&self);
}

impl Figure for Circle {
    fn print(&self) {
        println!("Circle");
    }
}

impl Figure for Triangle {
    fn print(&self) {
        println!("Triangle");
    }
}

// static dispatch
fn log<T: Figure>(figure: &T) {
    figure.print();
}

// dynamic dispatch: function takes a trait object
fn logd(figure: &Figure) {
    figure.print();
}

fn main() {
    // static dispatch
    let circle   = Circle;
    let triangle = Triangle;
    
    log(&circle);
    log(&triangle);

// dynamic dispatch: 
    let mut figures: Vec<Box<Figure>> = Vec::new();
    figures.push(Box::new(Circle));
    figures.push(Box::new(Triangle));
    
    // the precise type of figure can only be known at runtime:
    for figure in figures {
        // log(&*figure); // error: the trait `core::marker::Sized` is not implemented for the type `Figure`
        logd(&*figure);
    }
}
// Circle
// Triangle
// Circle
// Triangle