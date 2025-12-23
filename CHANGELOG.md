# Changelog

Todos los cambios notables en este proyecto serán documentados en este archivo.

El formato está basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.0.0/),
y este proyecto adhiere a [Semantic Versioning](https://semver.org/lang/es/).

## [0.1.0] - 2025-12-17

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

## [0.2.0] - 2025-12-23

### Añadido
- Comando `init` para crear estructura de directorios automáticamente
- Carga dinámica de templates desde filesystem
- Script de instalación automática (`install.sh`)
- Soporte para templates personalizables por usuario
- READMEs explicativos en cada directorio
- Mejoras en manejo de errores con mensajes descriptivos

### Cambiado
- **BREAKING**: Templates ahora se cargan desde filesystem en runtime
- Los comandos `quick` y `new` ahora crean directorios automáticamente
- Mensajes de error más claros con sugerencias de solución
- Templates separados del código fuente
