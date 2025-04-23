# Advent of Code 2023 - Soluciones en Rust

Este repositorio contiene mis soluciones en Rust para los desafíos de programación de [Advent of Code 2023](https://adventofcode.com/2023).

## Estructura del Proyecto

El repositorio está organizado como un espacio de trabajo de Rust con múltiples crates, cada uno representando una solución para un desafío diario específico:

- `day1` hasta `day12`: Crates individuales para cada rompecabezas diario
- Cada día sigue la misma estructura:
  - `src/main.rs`: Contiene el código de la solución con las funciones `part1()` y `part2()`
  - `src/input.txt`: Contiene los datos de entrada del rompecabezas
  - `Cargo.toml`: Configuración del paquete

## Detalles Técnicos

- **Lenguaje**: Rust (edición 2021)
- **Sistema de Construcción**: Cargo
- **Dependencias**: Sin dependencias externas en los crates de solución

## Cómo Ejecutar

Puedes ejecutar la solución de cualquier día usando Cargo:

```bash
# Ejecutar un día específico (por ejemplo, day1)
cargo run -p day1

# Ejecutar con optimizaciones de versión
cargo run --release -p day1
```

## Soluciones Implementadas

- Día 1: ¿Trebuchet?!
- Día 2: El Enigma de los Cubos
- Día 3: Proporciones de Engranajes
- Día 4: Tarjetas para Rascar
- Día 5: Si le Das una Semilla a un Fertilizante
- Día 6: Espera un Momento
- Día 7: Cartas de Camello
- Día 8: Páramo Embrujado
- Día 9: Mantenimiento del Espejismo
- Día 10: Laberinto de Tuberías
- Día 11: Expansión Cósmica
- Día 12: Aguas Termales

## Acerca de Advent of Code

Advent of Code es un conjunto anual de rompecabezas de programación con temática navideña que pueden resolverse en cualquier lenguaje de programación. Cada rompecabezas consta de dos partes, donde la segunda se basa en la primera.

Para más información, visita [adventofcode.com](https://adventofcode.com/).