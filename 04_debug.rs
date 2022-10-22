// Esta estructura no pudes ser impresa con `fmt::Display` o
// con `fmt::Debug`.
struct UnPrintable(i32);

// El atributo `derive` automaticamente crea la implementacion
// requier hacer un `struct` imprimible con `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);

// Derivado de `fmt::Debug` la implementacion para `Structure`. `Structure`
// es una estructura que solo tiene `i32`.
#[derive(Debug)]
struct Structure(i32);

// Ponga una `Structure` dentro de la estructura `Deep`. hace que pueda ser imprimible
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Imprimieble con `{:?}` es similar a `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` es imprimible!
    println!("Now {:?} will print!", Structure(3));
    
    // El problema con `derive` es que no se tiene control
    // con el resultado a mostrar. Que es justamente ese `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
}
