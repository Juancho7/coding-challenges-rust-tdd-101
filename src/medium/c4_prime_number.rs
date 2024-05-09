/*
 * Escribe un programa que se encargue de comprobar si un número es o no primo.
 * Hecho esto, imprime los números primos entre 1 y 100.
 */

#[allow(dead_code)]
pub fn prime_number() {
    for i in 2..101 {
        let mut dividers = 0;

        for j in 2..i {
            if i % j == 0 {
                dividers += 1;
            }
        }

        if dividers == 0 {
            println!("{}", i)
        }
    }
}
