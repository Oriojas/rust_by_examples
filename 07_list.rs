use std::fmt; // Import the `fmt` module.

// define un struct llamado list `List` que contiene un vector `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // extrae el valor de la tupla usando indexacion,
        // y crea la referencia a `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // itera sobre v `v` en `vec` pero enumerando cada iteracion
        // cuenta en `count`.
        for (count, v) in vec.iter().enumerate() {
            // para cada elemento menos el priemro, pone una comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // cierra el [ abierto y retorna fmt::Result value.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}