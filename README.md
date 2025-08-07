# Calculadora en Rust con Iced

Este proyecto es una calculadora de escritorio simple desarrollada en Rust utilizando la biblioteca de GUI `iced`. Es un excelente ejemplo para estudiantes que están aprendiendo Rust y quieren ver cómo se construye una aplicación de escritorio interactiva.

## Características

*   Interfaz gráfica de usuario simple y limpia.
*   Operaciones aritméticas básicas: suma, resta, multiplicación y división.
*   Soporte para números decimales y paréntesis.
*   Manejo de números negativos.
*   Display separado para la expresión y el resultado.
*   Icono de aplicación personalizado.

## Captura de Pantalla

*(Aquí puedes añadir una captura de pantalla de la aplicación en funcionamiento)*

## Cómo Compilar y Ejecutar

Para compilar y ejecutar este proyecto, necesitarás tener instalado el [toolchain de Rust](https://www.rust-lang.org/tools/install).

1.  **Clona el repositorio:**

    ```bash
    git clone <URL-DEL-REPOSITORIO>
    cd calculadora_rust
    ```

2.  **Asegúrate de tener el icono:**

    Coloca un fichero de icono llamado `icon.png` en el directorio `assets`.

3.  **Compila y ejecuta la aplicación:**

    ```bash
    cargo run
    ```

    Esto compilará el proyecto y, si todo va bien, abrirá la ventana de la calculadora.

## Estructura del Código

El código está contenido en un único fichero, `src/main.rs`, y sigue el patrón de diseño **Modelo-Vista-Actualización (MVA)**, que es fundamental en `iced`.

*   **Modelo (`Calculator` struct):** Representa el estado de la aplicación. En nuestro caso, contiene la `expression` que el usuario está escribiendo y el `result` del último cálculo.

*   **Vista (`view` function):** Se encarga de dibujar la interfaz de usuario. Define cómo se ven los displays, los botones y cómo se organizan en la ventana.

*   **Actualización (`update` function):** Maneja los eventos de la aplicación. Cada vez que el usuario pulsa un botón, se envía un `Message` a la función `update`, que modifica el estado (el "Modelo") en consecuencia.

### Análisis de Expresiones

Para evaluar las expresiones matemáticas, el código implementa un `lexer` y un `parser`:

1.  **Lexer (`lexer` function):** Convierte la cadena de texto de la expresión (ej: `"5 * (10 - 2)"`) en una secuencia de "tokens" (ej: `[Number(5.0), Multiply, LParen, Number(10.0), Minus, Number(2.0), RParen]`).

2.  **Parser (`Parser` struct):** Toma los tokens del lexer y los convierte en un **Árbol de Sintaxis Abstracta (AST)**. El AST es una estructura de datos en forma de árbol que representa la estructura jerárquica de la expresión, respetando la precedencia de los operadores.

3.  **Evaluación del AST (`evaluate_ast` function):** Recorre el AST de forma recursiva para calcular el resultado final de la expresión.

## Dependencias

Este proyecto utiliza las siguientes dependencias (definidas en `Cargo.toml`):

*   [`iced`](https://crates.io/crates/iced): El kit de herramientas de GUI multiplataforma.
*   [`image`](https://crates.io/crates/image): Para cargar y procesar el icono de la aplicación.

## Contribuciones

Las contribuciones son bienvenidas. Si encuentras un error o tienes una idea para mejorar la calculadora, no dudes en abrir un *issue* o un *pull request*.
