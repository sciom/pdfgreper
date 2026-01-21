# pdfgreper

**pdfgreper** je CLI alat napisan u programskom jeziku Rust za brzo i učinkovito pretraživanje sadržaja PDF dokumenata korištenjem logičkih (boolean) izraza. Razvijen je za potrebe obrade velikih kolekcija znanstvenih radova u PDF formatu.

---

## O projektu

Ovaj softver razvijen je u okviru projekta **advanDEB** - *Napredni modeli dinamičkih energijskih budžeta s transportnom mrežom* (IP-2024-05-3615), financiranog od strane **Hrvatske zaklade za znanost (HRZZ)**.

**Verzija**: 0.1.0  
**Licenca**: MIT  
**Programski jezik**: Rust  
**Platforma**: Linux (x86_64)

---

## Značajke

- **Boolean pretraživanje** - korištenje logičkih operatora `AND` i `OR` te zagrada za kompleksne upite
- **Podrška za fraze** - pretraživanje višerječnih izraza korištenjem navodnika
- **Rekurzivno pretraživanje** - opcija za pretraživanje svih poddirektorija
- **CSV izvještaji** - automatsko generiranje strukturiranih izvještaja o rezultatima
- **Organizacija datoteka** - automatsko kopiranje ili premještanje pronađenih PDF-ova
- **Case-insensitive** - pretraživanje nije osjetljivo na velika/mala slova
- **Statički kompajlirano** - jedna binarna datoteka radi na svim Linux distribucijama

---

## Struktura repozitorija

```
pdfgreper/
├── pdfgreper           # Prekompajlirana binarna datoteka (statički linkana)
├── install.sh          # Instalacijska skripta
├── README.md           # Ova dokumentacija
├── USAGE.md            # Detaljno uputstvo za korištenje
└── source/             # Izvorni kod
    ├── Cargo.toml      # Manifest projekta
    ├── Cargo.lock      # Zaključane verzije ovisnosti
    └── src/
        └── main.rs     # Glavni izvorni kod
```

---

## Brza instalacija (prekompajlirana verzija)

### Preduvjet: pdftotext

Prije instalacije pdfgreper-a, morate instalirati `pdftotext`:

```bash
# Ubuntu/Debian
sudo apt-get install poppler-utils

# Fedora/RHEL
sudo dnf install poppler-utils

# Arch Linux
sudo pacman -S poppler
```

### Instalacija

```bash
# 1. Klonirajte repozitorij
git clone https://github.com/KORISNIK/pdfgreper.git
cd pdfgreper

# 2. Pokrenite instalacijsku skriptu
./install.sh
```

To je sve! Program je sada instaliran i dostupan kao `pdfgreper` naredba.

> **Napomena**: Prekompajlirana verzija (`pdfgreper`) je statički linkana i radi na svim modernim Linux distribucijama bez dodatnih ovisnosti.

---

## Kompilacija iz izvornog koda

Za korisnike koji žele kompajlirati iz izvornog koda:

### Preduvjeti

- Rust toolchain (cargo, rustc) - verzija 1.56+
- pdftotext (poppler-utils)

### Kompilacija

```bash
cd source

# Standardna kompilacija
cargo build --release

# Statički linkana kompilacija (preporučeno za distribuciju)
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release
```

Binarna datoteka će biti kreirana u: `source/target/release/pdfgreper`

---

## Brzi početak

### Osnovni primjer

```bash
pdfgreper --dir /putanja/do/pdfs --bool 'earthworm AND pesticide'
```

### Kompleksniji upit s frazama

```bash
pdfgreper \
  --dir /putanja/do/pdfs \
  --recursive \
  --bool '("acetylcholinesterase" OR "AChE") AND (earthworm OR earthworms)'
```

---

## Opcije naredbenog retka

| Opcija | Opis | Zadana vrijednost |
|--------|------|-------------------|
| `--dir <putanja>` | Direktorij s PDF datotekama | Trenutni direktorij |
| `--out <csv>` | Putanja do izlazne CSV datoteke | `rezultati_pdfgreper.csv` |
| `--recursive` | Rekurzivno pretraži poddirektorije | Isključeno |
| `--bool <izraz>` | Boolean izraz za pretraživanje **(obavezno)** | - |
| `--cporrm <copy\|remove>` | Kopiraj ili premjesti pronađene PDF-ove | `copy` |
| `--folder <naziv>` | Ciljni direktorij za pronađene PDF-ove | `SELECTED_PDFs` |
| `--help` | Prikaži pomoć | - |

---

## Boolean izrazi

### Podržani operatori

- **AND** - oba uvjeta moraju biti zadovoljena
- **OR** - barem jedan uvjet mora biti zadovoljen
- **( )** - zagrade za grupiranje izraza
- **" "** - navodnici za višerječne fraze

### Prioritet operatora

1. Zagrade `( )` - najviši prioritet
2. `AND`
3. `OR` - najniži prioritet

### Primjeri izraza

```bash
# Jednostavan AND
--bool 'earthworm AND toxicity'

# Jednostavan OR
--bool 'earthworm OR collembola'

# Kombinacija s zagradama
--bool '(earthworm OR earthworms) AND (pesticide OR herbicide)'

# Fraze u navodnicima
--bool '"acetylcholine esterase" AND earthworm'

# Kompleksni izraz
--bool '("AChE" OR "acetylcholinesterase") AND (earthworm OR earthworms) AND (activity OR inhibition)'
```

---

## Izlazne datoteke

### CSV izvještaj

| Stupac | Opis |
|--------|------|
| `file` | Naziv PDF datoteke |
| `matched` | `true` ili `false` - je li izraz pronađen |
| `snippet` | Prvih ~200 znakova teksta (ako je pronađen) |

**Primjer:**
```csv
file,matched,snippet
"rad_01.pdf",true,"This study examines acetylcholinesterase activity in earthworm tissues..."
"rad_02.pdf",false,
```

### Direktorij s pronađenim PDF-ovima

PDF-ovi za koje je izraz zadovoljen automatski se kopiraju (ili premještaju) u ciljni direktorij.

### Log datoteka grešaka

Ako `pdftotext` ne uspije za neke datoteke, generira se:
```
pdftotext_failures_pdfgreper.txt
```

---

## Ovisnosti (za kompilaciju iz izvornog koda)

| Biblioteka | Verzija | Namjena |
|------------|---------|---------|
| `anyhow` | 1.x | Rukovanje greškama s kontekstom |
| `csv` | 1.x | Pisanje CSV datoteka |
| `walkdir` | 2.x | Rekurzivno prolaženje direktorija |

---

## Ograničenja

- **Nema NOT operatora** - trenutno nisu podržane negacije
- **Substring pretraga** - termini se traže kao podnizovi, ne cijele riječi
- **Sekvencijalna obrada** - PDF-ovi se obrađuju jedan po jedan
- **Ovisnost o pdftotext** - zahtijeva vanjski alat za ekstrakciju teksta

---

## Licenca

Ovaj softver je objavljen pod **MIT licencom**.

```
MIT License

Copyright (c) 2024-2025 advanDEB projekt

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## Zahvale

Ovaj softver razvijen je u okviru istraživačkog projekta **advanDEB** - *Napredni modeli dinamičkih energijskih budžeta s transportnom mrežom* (IP-2024-05-3615), financiranog od strane **Hrvatske zaklade za znanost (HRZZ)**.

---

## Citiranje

Ako koristite ovaj softver u svom istraživanju, molimo citirajte:

```
pdfgreper v0.1.0 (2025). Alat za pretraživanje PDF dokumenata.
Razvijeno u okviru projekta advanDEB (IP-2024-05-3615).
Hrvatska zaklada za znanost.
GitHub: https://github.com/SCIOM/pdfgreper
DOI: 10.5281/zenodo.18320209
```
```

---

## Kontakt

Za pitanja, prijedloge ili prijavu grešaka:
- Otvorite issue na GitHub-u
- Kontaktirajte tim projekta advanDEB

---

## Dokumentacija

- [USAGE.md](USAGE.md) - Detaljno uputstvo za korištenje

## Vanjske veze

- [Poppler Utils](https://poppler.freedesktop.org/) - Alati za rad s PDF-ovima
- [Rust Programming Language](https://www.rust-lang.org/) - Programski jezik Rust
- [Hrvatska zaklada za znanost](https://hrzz.hr/) - HRZZ
