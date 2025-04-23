# Advent of Code 2023 - Soluciones en Rust

Este repositorio contiene implementaciones en Rust para los rompecabezas de programación [Advent of Code 2023](https://adventofcode.com/2023).

## Estructura del Repositorio

Este proyecto está organizado como un espacio de trabajo de Rust con múltiples crates, cada uno representando una solución para el desafío de un día específico:

```
aoc-2023/
├── day1/        # Solución para el rompecabezas del Día 1
├── day2/        # Solución para el rompecabezas del Día 2
...
└── day12/       # Solución para el rompecabezas del Día 12
```

Cada directorio diario sigue la misma estructura:
- `src/main.rs` - Contiene el código de la solución con las funciones `part1()` y `part2()`
- `src/input.txt` - Contiene los datos de entrada del rompecabezas
- `Cargo.toml` - Configuración del paquete para el día específico

## Cómo Ejecutar

Para ejecutar la solución de un día específico:

```bash
# Navegar a la raíz del repositorio
cd aoc-2023

# Ejecutar la solución de un día específico
cargo run -p day1  # Reemplazar "day1" con el día que deseas ejecutar
```

Para ejecutar todas las soluciones:

```bash
cargo run --workspace
```

## Detalles de Implementación

Cada solución sigue el mismo patrón:
1. Leer la entrada del rompecabezas desde el archivo `input.txt` usando la macro `include_str!` de Rust
2. Implementar dos partes del rompecabezas diario (funciones `part1` y `part2`)
3. Procesar la entrada según los requisitos específicos del rompecabezas
4. Mostrar los resultados en la consola

## Tecnologías Utilizadas

- Lenguaje de programación Rust (edición 2021)
- Cargo para la gestión del proyecto
- Sin dependencias externas en las soluciones individuales diarias

## Traducciones

- [Inglés (English)](README.md)
- [Francés (Français)](README.fr.md)