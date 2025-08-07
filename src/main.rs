use std::io::{self, Write};

fn main() {
    println!("¡Bienvenido a la Calculadora Rust!");
    println!("Operaciones soportadas: +, -, *, /");
    println!("Escribe 'salir' para terminar.");

    loop {
        print!("Ingrese la expresión (ej. 1 + 2): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Fallo al leer la línea");
        let input = input.trim();

        if input.eq_ignore_ascii_case("salir") {
            println!("¡Hasta luego!");
            break;
        }

        match evaluate_expression(input) {
            Ok(result) => println!("Resultado: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn evaluate_expression(expression: &str) -> Result<f64, String> {
    let parts: Vec<&str> = expression.split_whitespace().collect();

    if parts.len() != 3 {
        return Err("Formato de expresión inválido. Use: número operador número (ej. 1 + 2)".to_string());
    }

    let num1 = parts[0].parse::<f64>().map_err(|_| "Primer número inválido".to_string())?;
    let operator = parts[1];
    let num2 = parts[2].parse::<f64>().map_err(|_| "Segundo número inválido".to_string())?;

    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("División por cero no permitida".to_string())
            } else {
                Ok(num1 / num2)
            }
        },
        _ => Err("Operador inválido. Use +, -, * o /".to_string()),
    }
}