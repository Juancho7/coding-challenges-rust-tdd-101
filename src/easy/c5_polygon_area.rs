/*
 * Crea una única función (importante que sólo sea una) que sea capaz
 * de calcular y retornar el área de un polígono.
 * - La función recibirá por parámetro sólo UN polígono a la vez.
 * - Los polígonos soportados serán Triángulo, Cuadrado y Rectángulo.
 * - Imprime el cálculo del área de un polígono de cada tipo.
 */

#[allow(dead_code)]
pub enum Polygon {
    Triangle { base: f64, height: f64 },
    Square { side: f64 },
    Rectangle { width: f64, height: f64 },
}

pub fn polygon_area(polygon: &Polygon) -> f64 {
    match polygon {
        Polygon::Triangle { base, height } => (base * height) / 2.0,
        Polygon::Square { side } => side * side,
        Polygon::Rectangle { width, height } => width * height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_area() {
        let square = Polygon::Square { side: 5.0 };
        let expected_area = 25.0;

        assert_eq!(polygon_area(&square), expected_area, "Testing square area");
    }

    #[test]
    fn test_rectangle_area() {
        let rectangle = Polygon::Rectangle {
            width: 5.0,
            height: 10.0,
        };
        let expected_area = 50.0;

        assert_eq!(
            polygon_area(&rectangle),
            expected_area,
            "Testing rectangle area"
        );
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Polygon::Triangle {
            base: 5.0,
            height: 10.0,
        };
        let expected_area = 25.0;

        assert_eq!(
            polygon_area(&triangle),
            expected_area,
            "Testing triangle area"
        );
    }
}
