# Zettel CLI

Un gestor de línea de comandos para notas [Zettelkasten](https://zettelkasten.de/), construido en Rust.

## Motivación

Como desarrollador que aprende Rust y arquitectura de software, necesitaba una herramienta simple para gestionar mis notas de aprendizaje siguiendo el método Zettelkasten. Este proyecto combina:

- **Aprendizaje práctico de Rust**: Ownership, borrowing, manejo de errores
- **Gestión de conocimiento**: Sistema Zettelkasten para conectar ideas
- **Desarrollo de CLI**: Uso de crates como `clap` y `chrono`

## Características

- ✅ Creación rápida de notas en inbox
- ✅ Generación automática de IDs basados en timestamp
- ✅ Templates predefinidos para notas permanentes
- ✅ Estructura de carpetas organizada
- 🚧 Búsqueda de notas (próximamente)
- 🚧 Vinculación automática de notas (próximamente)

## Requisitos

- Rust 1.70 o superior
- Cargo (incluido con Rust)

## Instalación

### Instalación automática
```bash
# Clonar el repositorio
git clone https://github.com/cfernandom/zettel_cli.git
cd zettel_cli

# Instalar (compila y configura templates)
./install.sh

# Verificar instalación
zettel --version
```

Esto:
1. Compila el binario en modo release y lo instala en `~/.cargo/bin/`
2. Copia los templates a `~/.config/zettel/templates/`

### Instalación manual
```bash
# Clonar el repositorio
git clone https://github.com/cfernandom/zettel_cli.git
cd zettel_cli

# Compilar e instalar binario
cargo install --path .

# Copiar templates manualmente
mkdir -p ~/.config/zettel/templates/directories
cp -r templates/* ~/.config/zettel/templates/

# Verificar instalación
zettel --version
```

### Variables de entorno (opcional)

Puedes personalizar la ubicación de los templates:
```bash
# En tu .bashrc o .zshrc
export ZETTEL_TEMPLATES_DIR="/ruta/personalizada/templates"
```

## Personalización

### Editar templates

Los templates están en `~/.config/zettel/templates/`:
```bash
# Editar template de nota permanente
nano ~/.config/zettel/templates/permanent-note.md

# Editar README de inbox
nano ~/.config/zettel/templates/directories/000-inbox.md
```

## Uso

### Comandos disponibles

#### Inicializar estructura de directorios

Para crear la estructura de directorios inicial:
```bash
zettel init
```

Resultado:
```
000-inbox/
100-permanent/
200-literature/
300-projects/
400-maps/
```

#### Crear nota rápida (inbox)

Para capturar ideas rápidamente sin procesar:
```bash
zettel quick "Título de la idea" "Contenido de la nota"
```

**Ejemplo:**
```bash
zettel quick "Ownership en Rust" "El ownership transfiere responsabilidad de memoria"
```

Resultado: `000-inbox/Ownership en Rust.md`

#### Crear nota permanente

Para notas procesadas con estructura completa:
```bash
zettel new "Título de la nota"
```

**Ejemplo:**
```bash
zettel new "Pattern matching vs switch"
```

Resultado: `100-permanent/20241217154530 - Pattern matching vs switch.md`

La nota incluirá un template con secciones:
- Contexto
- Explicación
- Ejemplo práctico
- Conexiones
- Referencias

### Workflow recomendado

1. **Captura rápida**: Usa `quick` durante el día para capturar ideas
2. **Procesamiento**: Revisa tu inbox regularmente
3. **Permanencia**: Convierte ideas procesadas a notas permanentes con `new`
4. **Vinculación**: Conecta notas relacionadas manualmente en Obsidian

## Estructura del proyecto
````
zettel_cli/
├── src/
│   └── main.rs          # Código principal
├── Cargo.toml           # Dependencias y metadata
├── Cargo.lock           # Versiones exactas de dependencias
└── README.md            # Este archivo
`````

## Desarrollo

### Compilar en modo debug
`````bash
cargo build
./target/debug/zettel_cli --help
`````

### Compilar optimizado (release)
`````bash
cargo build --release
./target/release/zettel_cli --help
`````

### Ejecutar tests
`````bash
cargo test
`````

### Generar documentación
`````bash
cargo doc --open
`````

## Dependencias

- [`clap`](https://docs.rs/clap/) - Parser de argumentos CLI
- [`chrono`](https://docs.rs/chrono/) - Manejo de fechas y timestamps

## Roadmap

- [ ] Comando `list` para listar notas por carpeta
- [ ] Comando `search` para buscar en títulos y contenido
- [ ] Comando `link` para mostrar conexiones entre notas
- [ ] Comando `graph` para visualizar red de notas
- [ ] Soporte para tags
- [ ] Integración con editores (neovim, vscode)
- [ ] Export a diferentes formatos (HTML, PDF)

## Contribuciones

Este es un proyecto personal de aprendizaje, pero sugerencias y feedback son bienvenidos.

## Licencia

MIT License - siente libre de usar este código para tu propio aprendizaje.

## Referencias

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Método Zettelkasten](https://zettelkasten.de/introduction/)
- [Clap Documentation](https://docs.rs/clap/)

---

**Hecho con 🦀 Rust como parte de mi viaje de aprendizaje**