/*
Importar (via `use`) el `fmt` para hacer disponible el modulo.
*/
use std::fmt; // Import `fmt`

// Una estructura que contiene dos numeros. `Debug` se derivará para que los resultados puedan
// ser contrastados con `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implementacion `Display` para `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Igualmente, imlpementar `Display` para `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. los dos `Debug` y `Display` se implementaron, pero `{:b}`
    // requiere `fmt::Binary` para ser implementado. esto no funcionara.
    // println!("What does Point2D look like in binary: {:b}?", point);
}