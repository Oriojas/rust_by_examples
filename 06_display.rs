use std::fmt; // Import `fmt`

// Una estructura tiene dos numeros. `Debug` se derivara para que los resultados
// se comparen con `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// implementacion para `Display` para `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use `self.number` para referirse a cada posicion.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// define la estrcutura cuando los campos son nombrables para comparacion
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// de igual manera, se implementa `Display` para `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax); // solo para ver el resultado
    println!("Debug: {:?}", minmax); // debug en la salida ver la diferencia al imprimir el resultado

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point); // solo para ver el resultado
    println!("Debug: {:?}", point); // debug en la salida ver la diferencia al imprimir el resultado

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);
}