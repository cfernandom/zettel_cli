# Changelog

Todos los cambios notables en este proyecto serán documentados en este archivo.

El formato está basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.0.0/),
y este proyecto adhiere a [Semantic Versioning](https://semver.org/lang/es/).

## [0.1.0] - 2024-12-17

### Añadido
- Comando `quick` para crear notas rápidas en inbox
- Comando `new` para crear notas permanentes con template
- Generación automática de IDs con timestamp
- Template estructurado para notas permanentes
- Documentación inicial en código
- README con instrucciones de uso

### Características técnicas
- Uso de Clap 4.4 para parsing de CLI
- Chrono para timestamps
- Manejo básico de errores con Result