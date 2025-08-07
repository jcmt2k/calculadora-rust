// ###################################################################################
// # Explicación del Proyecto de Calculadora en Rust con Iced #
// ###################################################################################

// # 1. Introducción
//
// Este proyecto es una calculadora de escritorio desarrollada en Rust utilizando la
// biblioteca `iced`. Iced es un kit de herramientas de GUI (Interfaz Gráfica de Usuario)
// multiplataforma que se centra en la simplicidad y la seguridad de tipos.
//
// El objetivo de este proyecto es demostrar cómo construir una aplicación de escritorio
// interactiva en Rust, cubriendo conceptos como:
//
// - **Estructura de una aplicación `iced`**: El patrón Modelo-Vista-Actualización (MVA).
// - **Manejo de eventos**: Cómo la aplicación reacciona a las interacciones del usuario.
// - **Análisis de expresiones matemáticas**: Implementación de un `lexer` y un `parser`
//   para interpretar y evaluar las expresiones introducidas por el usuario.
// - **Manejo de errores**: Cómo gestionar y mostrar errores de forma segura.
//
// # 2. Importaciones
//
// Aquí importamos todos los componentes necesarios de la biblioteca `iced`.
//
// - `border`, `executor`, `widget::{...}`: Componentes para construir la interfaz gráfica.
// - `Application`, `Command`, `Element`, `Length`, `Settings`, `Theme`: Tipos y `traits`
//   fundamentales de `iced`.
// - `alignment`, `Color`, `Size`, `window`: Utilidades para el estilo y la configuración
//   de la ventana.
use iced::{border, executor, widget::{button, column, row, text, container, Space}, Application, Command, Element, Length, Settings, Theme, alignment, Color, Size, window};

// # 3. Carga del Icono
//
// Esta función se encarga de cargar el icono de la aplicación desde un fichero.
//
// - `include_bytes!`: Una macro de Rust que incluye los bytes de un fichero en el
//   binario de la aplicación en tiempo de compilación. Esto es útil para asegurar que
//   el icono siempre esté disponible.
// - `image::load_from_memory`: Carga la imagen desde los bytes incluidos.
// - `window::icon::from_rgba`: Crea un icono de ventana a partir de los datos de la
//   imagen en formato RGBA (Rojo, Verde, Azul, Alfa).
fn load_icon() -> window::Icon {
    let image = image::load_from_memory(include_bytes!("../assets/icon.png")).unwrap().to_rgba8();
    let (width, height) = image.dimensions();
    window::icon::from_rgba(image.into_raw(), width, height).unwrap()
}

// # 4. Mensajes de la Aplicación
//
// En `iced`, los "mensajes" son la forma en que se comunican los eventos de la
// interfaz de usuario a la lógica de la aplicación. Este `enum` define todos los
// posibles mensajes que nuestra aplicación puede manejar.
#[derive(Debug, Clone)]
enum Message {
    NumberPressed(char),
    OperationPressed(char),
    EqualsPressed,
    ClearPressed,
    ParenthesisPressed(char),
}

// # 5. Estructura de la Calculadora (El Modelo)
//
// Esta estructura (`struct`) representa el estado de nuestra aplicación. En el patrón
// MVA, esto es el "Modelo".
//
// - `expression`: Almacena la expresión matemática que el usuario está introduciendo.
// - `result`: Almacena el resultado del último cálculo.
struct Calculator {
    expression: String,
    result: String,
}

// # 6. Implementación del `trait` `Application`
//
// El `trait` `Application` de `iced` es el núcleo de la aplicación. Aquí definimos
// cómo se inicializa, cómo se actualiza su estado, cómo se dibuja y qué título tiene.
impl Application for Calculator {
    // `Executor` se encarga de ejecutar comandos asíncronos. `Default` es suficiente
    // para la mayoría de los casos.
    type Executor = executor::Default;
    // El tipo de mensaje que nuestra aplicación utilizará.
    type Message = Message;
    // El tema visual de la aplicación. `Theme::default()` nos da el tema estándar.
    type Theme = Theme;
    // `Flags` se utiliza para pasar datos a la aplicación al iniciarla. No lo usamos aquí.
    type Flags = ();

    // `new` se llama una sola vez, al iniciar la aplicación. Devuelve el estado inicial
    // de la aplicación y cualquier comando inicial que deba ejecutarse.
    fn new(_flags: ()) -> (Calculator, Command<Message>) {
        (
            Calculator {
                expression: String::new(),
                result: String::new(),
            },
            Command::none(),
        )
    }

    // `title` devuelve el título que se mostrará en la barra de la ventana.
    fn title(&self) -> String {
        String::from("Calculadora Rust")
    }

    // `update` se llama cada vez que se recibe un `Message`. Aquí es donde se actualiza
    // el estado de la aplicación (el "Modelo").
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NumberPressed(digit) => {
                self.expression.push(digit);
            }
            Message::OperationPressed(op) => {
                self.expression.push(' ');
                self.expression.push(op);
                self.expression.push(' ');
            }
            Message::ParenthesisPressed(paren) => {
                self.expression.push(paren);
            }
            Message::EqualsPressed => {
                // Aquí es donde se produce la magia:
                // 1. `lexer`: Convierte la cadena de expresión en una secuencia de `Token`s.
                // 2. `Parser::new(tokens).parse()`: Convierte los `Token`s en un Árbol de
                //    Sintaxis Abstracta (AST).
                // 3. `evaluate_ast`: Evalúa el AST para obtener el resultado.
                match lexer(&self.expression).and_then(|tokens| Parser::new(tokens).parse()) {
                    Ok(ast) => match evaluate_ast(&ast) {
                        Ok(res) => {
                            self.result = res.to_string();
                            self.expression.clear();
                        }
                        Err(e) => {
                            self.result = format!("Error: {}", e);
                            self.expression.clear();
                        }
                    },
                    Err(e) => {
                        self.result = format!("Error: {}", e);
                        self.expression.clear();
                    }
                }
            }
            Message::ClearPressed => {
                self.expression.clear();
                self.result.clear();
            }
        }
        // `Command::none()` indica que no se debe ejecutar ningún comando asíncrono.
        Command::none()
    }

    // `view` se llama para dibujar la interfaz de usuario. Devuelve un `Element`, que es
    // un widget de `iced`. En el patrón MVA, esta es la "Vista".
    fn view(&self) -> Element<'_, Message> {
        // Creamos el texto para el display de la expresión.
        let expression_text = text(&self.expression)
            .size(40)
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Right);

        // Creamos un contenedor para el display de la expresión, con un borde.
        let expression_display = container(expression_text)
            .width(Length::Fill)
            .padding(10)
            .center_y()
            .style(|_theme: &Theme| container::Appearance {
                border: border::Border {
                    color: Color::BLACK,
                    width: 2.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            });

        // Creamos el texto para el display del resultado.
        let result_text = text(&self.result)
            .size(30)
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Right);

        // Creamos un contenedor para el display del resultado.
        let result_display = container(result_text)
            .width(Length::Fill)
            .padding(10)
            .center_y()
            .style(|_theme: &Theme| container::Appearance {
                border: border::Border {
                    color: Color::BLACK,
                    width: 2.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            });

        // Creamos los botones de la calculadora.
        let clear_button = button(text("C").size(30).horizontal_alignment(alignment::Horizontal::Center))
            .on_press(Message::ClearPressed)
            .width(Length::Fill);

        let equals_button = button(text("=").size(30).horizontal_alignment(alignment::Horizontal::Center))
            .on_press(Message::EqualsPressed)
            .width(Length::Fill);

        // Usamos una clausura (`closure`) para crear los botones numéricos.
        let num_button = |digit: char| {
            button(text(digit).size(30).horizontal_alignment(alignment::Horizontal::Center))
                .on_press(Message::NumberPressed(digit))
                .width(Length::Fill)
        };

        // Y otra clausura para los botones de operación.
        let op_button = |op: char| {
            button(text(op).size(30).horizontal_alignment(alignment::Horizontal::Center))
                .on_press(Message::OperationPressed(op))
                .width(Length::Fill)
        };

        let paren_button = |paren: char| {
            button(text(paren).size(30).horizontal_alignment(alignment::Horizontal::Center))
                .on_press(Message::ParenthesisPressed(paren))
                .width(Length::Fill)
        };

        // Organizamos los widgets en una columna (`column!`).
        column![
            expression_display,
            result_display,
            Space::with_height(Length::Fixed(10.0)), // Espacio entre el display y los botones
            // Cada fila (`row!`) de botones tiene un espaciado.
            row![clear_button, paren_button('('), paren_button(')'), op_button('/')].spacing(10),
            row![num_button('7'), num_button('8'), num_button('9'), op_button('*')].spacing(10),
            row![num_button('4'), num_button('5'), num_button('6'), op_button('-')].spacing(10),
            row![num_button('1'), num_button('2'), num_button('3'), op_button('+')].spacing(10),
            row![num_button('0'), num_button('.'), equals_button].spacing(10),
        ]
        .padding(20)
        .spacing(10)
        .into()
    }
}

// # 7. Lexer (Analizador Léxico)
//
// El `lexer` es el primer paso para interpretar la expresión matemática. Su trabajo
// es convertir la cadena de entrada (ej: "3 + 4") en una secuencia de "tokens"
// (ej: `[Number(3.0), Plus, Number(4.0)]`).
#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    UnaryMinus,
}

fn lexer(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut last_token_was_operator = true;

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                let num = num_str.parse::<f64>().map_err(|_| format!("Número inválido: {}", num_str))?;
                tokens.push(Token::Number(num));
                last_token_was_operator = false;
            }
            '+' => { tokens.push(Token::Plus); chars.next(); last_token_was_operator = true; },
            '-' => {
                // Aquí diferenciamos entre el signo de resta y el de un número negativo.
                if last_token_was_operator {
                    tokens.push(Token::UnaryMinus);
                } else {
                    tokens.push(Token::Minus);
                }
                chars.next();
                last_token_was_operator = true;
            },
            '*' => { tokens.push(Token::Multiply); chars.next(); last_token_was_operator = true; },
            '/' => { tokens.push(Token::Divide); chars.next(); last_token_was_operator = true; },
            '(' => { tokens.push(Token::LParen); chars.next(); last_token_was_operator = true; },
            ')' => { tokens.push(Token::RParen); chars.next(); last_token_was_operator = false; },
            ' ' => { chars.next(); }, // Ignorar espacios
            _ => return Err(format!("Carácter inesperado: {}", c)),
        }
    }
    Ok(tokens)
}

// # 8. Parser (Analizador Sintáctico) y AST (Árbol de Sintaxis Abstracta)
//
// El `parser` toma la secuencia de `Token`s del `lexer` y la convierte en un
// Árbol de Sintaxis Abstracta (AST). El AST es una estructura de datos en forma de
// árbol que representa la estructura de la expresión matemática.
#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Literal(f64),
    UnaryOp {
        op: Token,
        expr: Box<Expr>,
    },
    BinaryOp {
        op: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    // `peek` nos permite ver el siguiente token sin consumirlo.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    // `next` consume y devuelve el siguiente token.
    fn next(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    // `expect` consume el siguiente token y comprueba que es del tipo esperado.
    fn expect(&mut self, expected_token: Token) -> Result<Token, String> {
        let token = self.next().ok_or_else(|| format!("Se esperaba {:?}, pero se encontró el final de la entrada", expected_token))?;
        if token == expected_token {
            Ok(token)
        } else {
            Err(format!("Se esperaba {:?}, pero se encontró {:?}", expected_token, token))
        }
    }

    // `parse` es el punto de entrada del `parser`.
    fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.parse_expression()?;
        if self.position < self.tokens.len() {
            Err(format!("Tokens inesperados al final de la expresión: {:?}", &self.tokens[self.position..]))
        } else {
            Ok(expr)
        }
    }

    // Las siguientes funciones implementan la precedencia de operadores (PEMDAS/BODMAS).
    fn parse_expression(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_term()?;
        while let Some(token) = self.peek() {
            match token {
                Token::Plus | Token::Minus => {
                    let op = self.next().unwrap();
                    let right = self.parse_term()?;
                    expr = Expr::BinaryOp { op, left: Box::new(expr), right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_factor()?;
        while let Some(token) = self.peek() {
            match token {
                Token::Multiply | Token::Divide => {
                    let op = self.next().unwrap();
                    let right = self.parse_factor()?;
                    expr = Expr::BinaryOp { op, left: Box::new(expr), right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let token = self.next().ok_or("Se esperaba un número o paréntesis, pero se encontró el final de la entrada")?;
        match token {
            Token::Number(value) => Ok(Expr::Literal(value)),
            Token::UnaryMinus => {
                let expr = self.parse_factor()?;
                Ok(Expr::UnaryOp { op: Token::UnaryMinus, expr: Box::new(expr) })
            }
            Token::LParen => {
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => Err(format!("Token inesperado: {:?}", token)),
        }
    }
}

// # 9. Evaluación del AST
//
// Esta función recorre el AST de forma recursiva y calcula el resultado final de la
// expresión.
fn evaluate_ast(expr: &Expr) -> Result<f64, String> {
    match expr {
        Expr::Literal(value) => Ok(*value),
        Expr::UnaryOp { op, expr } => {
            let val = evaluate_ast(expr)?;
            match op {
                Token::UnaryMinus => Ok(-val),
                _ => Err(format!("Operador unario inesperado en AST: {:?}", op)),
            }
        }
        Expr::BinaryOp { op, left, right } => {
            let left_val = evaluate_ast(left)?;
            let right_val = evaluate_ast(right)?;
            match op {
                Token::Plus => Ok(left_val + right_val),
                Token::Minus => Ok(left_val - right_val),
                Token::Multiply => Ok(left_val * right_val),
                Token::Divide => {
                    if right_val == 0.0 {
                        Err("División por cero no permitida".to_string())
                    } else {
                        Ok(left_val / right_val)
                    }
                },
                _ => Err(format!("Operador binario inesperado en AST: {:?}", op)),
            }
        }
    }
}

// # 10. Función `main`
//
// El punto de entrada de la aplicación. Aquí configuramos y ejecutamos la aplicación `iced`.
fn main() -> iced::Result {
    Calculator::run(Settings {
        window: iced::window::Settings {
            size: Size::new(400.0, 600.0),
            icon: Some(load_icon()),
            ..Default::default()
        },
        ..Settings::default()
    })
}
