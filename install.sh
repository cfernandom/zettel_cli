#!/bin/bash
# Script de instalación para zettel_cli

set -e

echo "Compilando e instalando zettel_cli..."
cargo install --path .

TEMPLATES_DIR="$HOME/.config/zettel/templates"
echo "configurando templates en: $TEMPLATES_DIR"

mkdir -p "$TEMPLATES_DIR/directories"
cp -r templates/* "$TEMPLATES_DIR/"

echo "Los templates están en: $TEMPLATES_DIR. Puedes editar los templates sin necesidad de recompilar."
echo ""
echo "Instalación completada!"
echo "Para comenzar, ejecuta:"
echo "  zettel init"