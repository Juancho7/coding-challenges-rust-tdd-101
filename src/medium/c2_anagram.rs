/*
 * Escribe una función que reciba dos palabras (String) y retorne
 * verdadero o falso (Bool) según sean o no anagramas.
 * - Un Anagrama consiste en formar una palabra reordenando TODAS
 *   las letras de otra palabra inicial.
 * - NO hace falta comprobar que ambas palabras existan.
 * - Dos palabras exactamente iguales no son anagrama.
 */

#[allow(dead_code)]
pub fn is_anagram(s1: &str, s2: &str) -> bool {
    let mut chars1: Vec<char> = s1.chars().collect();
    let mut chars2: Vec<char> = s2.chars().collect();

    chars1.sort();
    chars2.sort();

    chars1 == chars2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_not_anagram() {
        assert_eq!(is_anagram("hello", "world"), false);
    }

    #[test]
    fn test_is_anagram() {
        assert_eq!(is_anagram("cinema", "iceman"), true);
    }
}
