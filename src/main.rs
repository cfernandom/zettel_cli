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
use std::fs;

/// CLI principal para gestionar notas Zettelkasten
#[derive(Parser)]
#[command(name = "zettel")]
#[command(author = "Cristhian F. Moreno cfernandom97@gmail.com")]
#[command(version = "0.1.0")]
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
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Quick { title, content } => {
            create_quick_note(&title, &content);
        }
        Commands::New { title } => {
            create_permanent_note(&title);
        }
    }
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
        return;
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
fn create_permanent_note(title: &str) {
    let timestamp = Local::now().format("%Y%m%d%H");
    let permanent_path = "001-permanent";
    let filename = format!("{}/{} - {}.md", permanent_path, timestamp, title);
    
    let template = format!(
        "# {} - {}\n\n## Contexto\n\n## Explicación\n\n## Ejemplo práctico\n\n## Conexiones\n\n## Referencias\n",
        timestamp, title
    );
    
    if let Err(e) = fs::write(&filename, template) {
        eprintln!("Error al crear nota: {}", e);
        return;
    }
    
    println!("✓ Nota permanente creada: {}", filename);
}
