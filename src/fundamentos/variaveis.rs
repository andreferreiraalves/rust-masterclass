pub fn imutaveis() {
    let x = 5;
    // x = 10;
    println!("x => {}", x);
}

pub fn mutaveis() {
    let mut x = 10;

    let y = x;
    println!("x, y => {} {}", x, y);

    x = 15;
    println!("x, y => {} {}", x, y);
}

pub fn constantes() {
    const Z: i32 = 20;
    println!("O valor de z é: {}", Z);
}

pub fn shadowing() {
    let a = 25;
    println!("o valor de a é: {}", a);

    let a = "Texto";
    println!("o valor de a é: {}", a);
}
