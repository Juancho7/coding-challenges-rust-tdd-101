/*
 * Escribe un programa que imprima los 50 primeros números de la sucesión
 * de Fibonacci empezando en 0.
 * - La serie Fibonacci se compone por una sucesión de números en
 *   la que el siguiente siempre es la suma de los dos anteriores.
 *   0, 1, 1, 2, 3, 5, 8, 13...
 */

#[allow(dead_code)]
pub fn fibonacci() {
    let mut n1 = 0;
    let mut n2 = 1;
    let mut s: i64 = n1;

    for _ in 0..50 {
        println!("{}", s);
        n1 = n2;
        n2 = s;
        s = n1 + n2;
    }
}
