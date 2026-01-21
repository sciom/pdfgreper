#!/bin/bash
#
# pdfgreper - Instalacijska skripta
#
# Ovaj softver razvijen je u okviru projekta advanDEB - Napredni modeli
# dinamičkih energijskih budžeta s transportnom mrežom (IP-2024-05-3615),
# financiranog od strane Hrvatske zaklade za znanost (HRZZ).
#
# Licenca: MIT
#

set -e

# Boje za ispis
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Verzija
VERSION="0.1.0"

# Zadana instalacijska lokacija
DEFAULT_INSTALL_DIR="$HOME/.local/bin"
INSTALL_DIR="${INSTALL_DIR:-$DEFAULT_INSTALL_DIR}"

# Naziv binarne datoteke
BINARY_NAME="pdfgreper"

# Putanja do binarne datoteke u repozitoriju
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_PATH="$SCRIPT_DIR/target/release/$BINARY_NAME"

echo -e "${BLUE}"
echo "  ____  ____  _____                               "
echo " |  _ \|  _ \|  ___|__ _ _ __ ___ _ __   ___ _ __ "
echo " | |_) | | | | |_ / _\` | '__/ _ \ '_ \ / _ \ '__|"
echo " |  __/| |_| |  _| (_| | | |  __/ |_) |  __/ |   "
echo " |_|   |____/|_|  \__, |_|  \___| .__/ \___|_|   "
echo "                  |___/         |_|              "
echo -e "${NC}"
echo -e "${YELLOW}Verzija: $VERSION${NC}"
echo -e "${YELLOW}Projekt: advanDEB (IP-2024-05-3615) - HRZZ${NC}"
echo ""

# Funkcija za ispis poruka
info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[OK]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[UPOZORENJE]${NC} $1"
}

error() {
    echo -e "${RED}[GREŠKA]${NC} $1"
    exit 1
}

# Provjeri je li binarna datoteka dostupna
check_binary() {
    if [[ ! -f "$BINARY_PATH" ]]; then
        error "Binarna datoteka nije pronađena: $BINARY_PATH
       
Molimo prvo kompajlirajte projekt:
  cargo build --release

Ili preuzmite unaprijed kompajliranu verziju s GitHub-a."
    fi
    success "Binarna datoteka pronađena: $BINARY_PATH"
}

# Provjeri pdftotext
check_pdftotext() {
    if command -v pdftotext &> /dev/null; then
        success "pdftotext je instaliran: $(which pdftotext)"
    else
        warning "pdftotext nije instaliran!
       
pdfgreper zahtijeva pdftotext za ekstrakciju teksta iz PDF-ova.
Instalirajte ga pomoću:

  Ubuntu/Debian: sudo apt-get install poppler-utils
  Fedora:        sudo dnf install poppler-utils
  Arch Linux:    sudo pacman -S poppler
  macOS:         brew install poppler"
    fi
}

# Kreiraj instalacijski direktorij
create_install_dir() {
    if [[ ! -d "$INSTALL_DIR" ]]; then
        info "Kreiram direktorij: $INSTALL_DIR"
        mkdir -p "$INSTALL_DIR"
        success "Direktorij kreiran"
    fi
}

# Kopiraj binarnu datoteku
install_binary() {
    info "Instaliram $BINARY_NAME u $INSTALL_DIR/"
    cp "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    success "Binarna datoteka instalirana: $INSTALL_DIR/$BINARY_NAME"
}

# Provjeri i ažuriraj PATH
setup_path() {
    # Provjeri je li direktorij već u PATH-u
    if echo "$PATH" | tr ':' '\n' | grep -q "^$INSTALL_DIR$"; then
        success "$INSTALL_DIR je već u PATH-u"
        return
    fi

    info "Dodajem $INSTALL_DIR u PATH..."
    
    # Odredi koju shell konfiguraciju koristiti
    SHELL_CONFIG=""
    if [[ -f "$HOME/.bashrc" ]]; then
        SHELL_CONFIG="$HOME/.bashrc"
    elif [[ -f "$HOME/.bash_profile" ]]; then
        SHELL_CONFIG="$HOME/.bash_profile"
    elif [[ -f "$HOME/.zshrc" ]]; then
        SHELL_CONFIG="$HOME/.zshrc"
    elif [[ -f "$HOME/.profile" ]]; then
        SHELL_CONFIG="$HOME/.profile"
    fi

    if [[ -n "$SHELL_CONFIG" ]]; then
        # Provjeri je li već dodano
        if ! grep -q "# pdfgreper PATH" "$SHELL_CONFIG" 2>/dev/null; then
            echo "" >> "$SHELL_CONFIG"
            echo "# pdfgreper PATH" >> "$SHELL_CONFIG"
            echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG"
            success "PATH ažuriran u $SHELL_CONFIG"
        else
            info "PATH konfiguracija već postoji u $SHELL_CONFIG"
        fi
    else
        warning "Nije pronađena shell konfiguracijska datoteka.
Ručno dodajte sljedeće u vašu shell konfiguraciju:

  export PATH=\"\$PATH:$INSTALL_DIR\""
    fi
}

# Provjeri instalaciju
verify_installation() {
    echo ""
    info "Provjeravam instalaciju..."
    
    # Osvježi PATH za trenutnu sesiju
    export PATH="$PATH:$INSTALL_DIR"
    
    if command -v "$BINARY_NAME" &> /dev/null; then
        success "pdfgreper je uspješno instaliran!"
        echo ""
        echo -e "${GREEN}Možete pokrenuti:${NC}"
        echo "  pdfgreper --help"
        echo ""
    else
        warning "pdfgreper instaliran, ali nije u PATH-u za trenutnu sesiju.
        
Pokrenite jednu od sljedećih naredbi:
  source ~/.bashrc
  source ~/.zshrc
  
Ili otvorite novi terminal."
    fi
}

# Deinstalacija
uninstall() {
    echo -e "${YELLOW}Deinstalacija pdfgreper...${NC}"
    
    if [[ -f "$INSTALL_DIR/$BINARY_NAME" ]]; then
        rm "$INSTALL_DIR/$BINARY_NAME"
        success "Binarna datoteka uklonjena: $INSTALL_DIR/$BINARY_NAME"
    else
        warning "Binarna datoteka nije pronađena: $INSTALL_DIR/$BINARY_NAME"
    fi
    
    echo ""
    info "PATH konfiguracija nije uklonjena. Ručno uklonite sljedeći redak iz vaše shell konfiguracije:"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    echo ""
    success "Deinstalacija završena"
}

# Pomoć
show_help() {
    echo "Upotreba: $0 [OPCIJA]"
    echo ""
    echo "Opcije:"
    echo "  install     Instaliraj pdfgreper (zadano)"
    echo "  uninstall   Deinstaliraj pdfgreper"
    echo "  --help      Prikaži ovu pomoć"
    echo ""
    echo "Varijable okruženja:"
    echo "  INSTALL_DIR   Instalacijski direktorij (zadano: $DEFAULT_INSTALL_DIR)"
    echo ""
    echo "Primjeri:"
    echo "  ./install.sh                           # Standardna instalacija"
    echo "  ./install.sh install                   # Standardna instalacija"
    echo "  ./install.sh uninstall                 # Deinstalacija"
    echo "  INSTALL_DIR=/usr/local/bin ./install.sh  # Instalacija u /usr/local/bin"
}

# Glavna funkcija
main() {
    case "${1:-install}" in
        install)
            check_binary
            check_pdftotext
            create_install_dir
            install_binary
            setup_path
            verify_installation
            ;;
        uninstall)
            uninstall
            ;;
        --help|-h|help)
            show_help
            ;;
        *)
            error "Nepoznata opcija: $1
            
Pokrenite '$0 --help' za pomoć."
            ;;
    esac
}

# Pokreni
main "$@"
