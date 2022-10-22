// suma struct

struct Entrada {
    a: i64,
    b: i64,
}

fn main() {
    let entrada = Entrada{a: 5, b: 3};

    let c = entrada.a + entrada.b;

    println!("{} + {} = {}", entrada.a, entrada.b, c);


}