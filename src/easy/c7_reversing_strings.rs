/*
 * Crea un programa que invierta el orden de una cadena de texto
 * sin usar funciones propias del lenguaje que lo hagan de forma automática.
 * - Si le pasamos "Hola mundo" nos retornaría "odnum aloH"
 */

#[allow(dead_code)]
pub fn reverse_string(text: &str) -> String {
    let characters: Vec<char> = text.chars().collect();

    let length = characters.len();

    let mut reversed_characters: Vec<char> = Vec::with_capacity(length);

    for i in (0..length).rev() {
        reversed_characters.push(characters[i]);
    }

    let reversed_text: String = reversed_characters.iter().collect();

    reversed_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("Hola mundo"), "odnum aloH");
    }
}
