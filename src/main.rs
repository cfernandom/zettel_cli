//! # Zettel CLI
//! 
//! Un gestor de línea de comandos para notas Zettelkasten
//! 
//! Este CLI permite crear y gestionar notas siguiendo el método Zettelkasten,
//! facilitando la captura rápida de ideas y la creación de notas permanentes
//! con timestamps.
//! 
//! ## Instalación
//! 
//! ```bash
//! cargo install --path .
//! ```
//! 
//! ## Uso básico
//! 
//! ```bash
//! # Crear nota rápida en inbox
//! zettel quick "Titulo" "Contenido de la nota"
//! 
//! # Crear nota permanente con template
//! zettel new "Título de la nota"
//! ```

use chrono::Local;
use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Estructura de directorios requerida para el sistema Zettelkasten
const REQUIRED_DIRS: &[&str] = &[
    "000-inbox",
    "100-permanent",
    "200-literature",
    "300-projects",
    "400-maps",
];

/// CLI principal para gestionar notas Zettelkasten
#[derive(Parser)]
#[command(name = "zettel")]
#[command(author = "Cristhian F. Moreno cfernandom97@gmail.com")]
#[command(version = "0.2.0")]
#[command(about = "Un CLI para gestionar notas Zettelkasten", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Comandos disponibles en el CLI
#[derive(Subcommand)]
enum Commands {
    /// Crear una nota rápida en inbox
    /// 
    /// Las notas quick son capturas rápidas de ideas que luego
    /// deben procesarse y convertirse en notas permanentes.
    Quick {
        /// Título de la nota (se usará como nombre del archivo)
        title: String,

        /// Contenido de la nota
        content: String,
    },

    /// Crear una nota permanente con ID temporal y template
    /// 
    /// Las notas permanentes son el corazón del sistema Zettelkasten.
    /// Cada nota recibe un ID único basado en timestamp (YYYYMMDDHH)
    New {
        /// Título de la nota (se agregará después del timestamp)
        title: String,
    },

    /// Inicializar la estructura de directorios Zettelkasten
    Init,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Quick { title, content } => {
            if let Err(e) = ensure_directory_exists("000-inbox") {
                eprintln!("Error: {}", e);
                std::process::exit(1)
            }
            create_quick_note(&title, &content);
        }
        Commands::New { title } => {
            if let Err(e) = ensure_directory_exists("100-permanent") {
                eprintln!("Error: {}", e);
                std::process::exit(1)
            }
            match create_permanent_note(&title) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };
        }
        Commands::Init => {
            initialize_zettelkasten_structure();
        }
    }
}

/// Encuentra el directorio de templates
/// 
/// Busca templates en el siguiente orden:
/// 1. Variable de entorno ZETTEL_TEMPLATES_DIR
/// 2. ~/.config/zettel/templates (Linux/Mac)
/// 3. Relativo al ejecutable: ../share/zettel/templates
/// 4. Directorio actual del proyecto (para desarrollo)
/// 
/// # Retorna
/// 
/// * `Ok(PathBuf)` con la ruta al directorio de templates
/// * `Err(io::Error)` si no se encuentra en ningún directorio válido
fn find_templates_dir() -> io::Result<PathBuf> {
    // 1. Variable de entorno
    if let Ok(dir) = env::var("ZETTEL_TEMPLATES_DIR") {
        let path = PathBuf::from(dir);
        if path.exists() {
            return Ok(path);
        }
    }

    // 2. Directorio de configuración del usuario
    if let Some(home) = env::var_os("HOME") {
        let config_dir = Path::new(&home).join(".config/zettel/templates");
        if config_dir.exists() {
            return Ok(config_dir);
        }
    }

    // 3. Relativo al ejecutable (instalación del sistema)
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let templates_dir = exe_dir.join("../share/zettel/templates");
            if templates_dir.exists() {
                return Ok(templates_dir);
            }
        }
    }

    // 4. Directorio actual
    let dev_templates = PathBuf::from("templates");
    if dev_templates.exists() {
        return Ok(dev_templates);
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No se encontró el directorio de templates. \n\
        Intenta: \n\
        1. ejecutar desde el directorio del proyecto\n\
        2. Configurar la variable de entorno ZETTEL_TEMPLATES_DIR\n\
        3. Copiar los templates en ~/.config/zettel/templates",
    ))

}


/// Carga un template desde el filesystem
/// 
/// # Argumentos
/// 
/// * `template_name` - Nombre del archivo template, relativo a directorio templates/
/// 
/// # Ejemplo
/// 
/// ```no_run
/// let content = load_template("permanent-note.md")?;
/// ```
fn load_template(template_name: &str) -> io::Result<String> {
    let templates_dir = find_templates_dir()?;
    let template_path = templates_dir.join(template_name);

    fs::read_to_string(&template_path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!(
                "Error cargando template '{}': {}. \nRuta buscada: {}",
                template_name,
                e,
                template_path.display()
            )
        )
    })
}

/// Verifica que un directorio exista, y lo crea si no existe
/// 
/// # Argumentos
/// 
/// * `dir_path` - Ruta del directorio a verificar/crear
/// 
/// # Retorna
/// 
/// * `Ok(())` si el directorio existe o fue creado exitosamente
/// * `Err(io::Error)` si hubo un problema al crear el directorio
/// 
/// # Ejemplo
/// 
/// ```no_run
/// ensure_directory_exists("000-inbox")?;
/// ```
fn ensure_directory_exists(dir_path: &str) -> io::Result<()> {
    let path = Path::new(dir_path);

    if !path.exists() {
        println!("Creando directorio: {}", dir_path);
        fs::create_dir_all(path)?;
        println!("Directorio creado: {}", dir_path)
    }

    Ok(())
}

/// Inicializa toda la estructura de directorios Zettelkasten
/// 
/// Este comando crea todos los directorios necesarios para el sistema
/// y genera un README.md explicativo en cada uno.
fn initialize_zettelkasten_structure() {
    println!("Inicializando estructura Zettelkasten...\n");

    // Verificar que existan los templates
    match  find_templates_dir() {
        Ok(dir) => println!("Templates encontrados en: {}\n", dir.display()),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    let mut errors = Vec::new();

    for dir in REQUIRED_DIRS {
        match ensure_directory_exists(dir) {
            Ok(_) => {
                // crear README en cada directorio
                if let Err(e) = create_directory_readme(dir) {
                    errors.push(format!("Warning: no se pudo crear README en {}: {}", dir, e));
                }
            }
            Err(e) => {
                errors.push(format!("Error creando {}: {}", dir, e));
            }
        }
    }

    if errors.is_empty() {
        println!("\n Estructura Zettelkasten inicializada correctamente!");
        println!("\n Directorios creados:");
        for dir in REQUIRED_DIRS {
            println!("   - {}", dir);
        }
    } else {
        println!("\n Se completó con algunas advetencias:");
        for error in errors {
            println!("  {}", error);
        }
    }
}

/// Crea un README explicativo en cada directorio
/// 
/// Carga el template desde filesystem según el directorio
/// 
/// # Argumentos
///
/// * `dir_path` - Ruta del directorio donde crear el README
fn create_directory_readme(dir_path: &str) -> io::Result<()> {
    let readme_path = format!("{}/README.md", dir_path);

    // Solo crear si no existe
    if Path::new(&readme_path).exists() {
        return Ok(());
    }

    // Cargar template específico para este directorio
    let template_name = format!("directories/{}.md", dir_path);
    let content = load_template(&template_name)?;

    fs::write(readme_path, content)?;
    Ok(())
}

/// Crea una nota rápida en el directorio inbox
/// 
/// # Argumentos
/// 
/// * `title` - Título de la nota que se usará como nombre de archivo
/// * `content` - Contenido que se escribirá en la nota
/// 
/// # Ejemplo
/// 
/// ```no_run
/// create_quick_note("Mi idea", "Contenido de la idea");
/// ```
fn create_quick_note(title: &str, content: &str) {
    let inbox_path = "000-inbox";
    let filename = format!("{}/{}.md", inbox_path, title);
    
    if let Err(e) = fs::write(&filename, content) {
        eprintln!("Error al crear nota: {}", e);
        std::process::exit(1);
    }
    
    println!("✓ Nota creada en inbox: {}", filename);
}

/// Crea un nota permanente con template y timestamp único
/// 
/// # Argumentos
/// 
/// * `title` - Título de la nota
/// 
/// # Estructura del template
/// 
/// La nota se crea con las siguientes secciones:
/// - Contexto: Por qué es importante esta nota
/// - Explicación: El concepto en tus propias palabras
/// - Ejemplo práctico: Código o ejemplos concretos
/// - Conexiones: Links a otras notas relacionadas
/// - Referencias: Fuentes originales
/// 
/// # Ejemplo
/// 
/// ```no_run
/// create_permanent_note("Ownership en Rust");
/// // Crea: 001-permanent/2025121723 - Ownership en Rust.md
/// ```
fn create_permanent_note(title: &str) -> io::Result<()> {
    let timestamp = Local::now().format("%Y%m%d%H%M");
    let permanent_path = "100-permanent";
    let filename = format!("{}/{} - {}.md", permanent_path, timestamp, title);
    
    let template = load_template("permanent-note.md")?;
    
    let note_content = template
        .replace("{timestamp}", &timestamp.to_string())
        .replace("{title}", title);

    fs::write(&filename, note_content)?;
    
    println!("✓ Nota permanente creada: {}", filename);
    Ok(())
}
