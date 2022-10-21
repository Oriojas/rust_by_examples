// Rust permite formatear el texto de salida de varias maneras
fn main() {
    
    // con {} se puede ubicar el texto dentro del string donde se necesite
    println!("{} days", 32);

    // dentro de {} se puede indicar donde colocar los parametros indicando un numero entero
    println!("{0}, this is {1}. {1}, this is {0}", "Pollo", "Gallina");

    // se pueden noimbrar los argumentos.
    println!("{subject} {verb} {object}",
            object="del perro perezoso",
            subject="El veloz zorro marrón",
            verb="salta encima");

    // se puede formatear de diferente manera invocando y especificando el formato despues de
    // `:`
    println!("Base 10 repr:               {}",   69420);
    println!("Base 2 (binary) repr:       {:b}", 69420);
    println!("Base 8 (octal) repr:        {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);

    // se puede alinear a la derecha del texto especificando el ancho
    println!("{number:>5}", number=1);

    // se pueden agregar 0 a la derecha
    println!("{number:0>5}", number=1);

    // se pueden agregar agrumentos especificando el signo $ despues de la variable
    println!("{number:0>width$}", number=1, width=5);

    // rust revisa el orden de los argumentos delstring si no se corrgie el codigo y se deja un solo nombre genera error
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // solo los tipos que implementan fmt::Display se pueden formatear con `{}`
    // los tipos definidos por el usuario no implementan fmt::Display de forma predeterminada
    #[allow(dead_code)]
    struct Structure(i32);

    // Para Rust 1.58 y superior, puede capturar directamente el argumento de una 
    // variable circundante. Al igual que el anterior, esto generará // " 1". 5 espacios en blanco y un "1".

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
