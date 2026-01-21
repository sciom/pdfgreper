# Uputstvo za korištenje - pdfgreper

**Verzija**: 0.1.0  
**Licenca**: MIT  
**Projekt**: advanDEB - Napredni modeli dinamičkih energijskih budžeta s transportnom mrežom (IP-2024-05-3615)  
**Financirano od**: Hrvatska zaklada za znanost (HRZZ)

---

## Sadržaj

1. [Uvod](#1-uvod)
2. [Instalacija iz prekompajlirane verzije](#2-instalacija-iz-prekompajlirane-verzije)
3. [Kompilacija iz izvornog koda](#3-kompilacija-iz-izvornog-koda)
4. [Osnove korištenja](#4-osnove-korištenja)
5. [Boolean izrazi - detaljna sintaksa](#5-boolean-izrazi---detaljna-sintaksa)
6. [Opcije naredbenog retka](#6-opcije-naredbenog-retka)
7. [Praktični primjeri](#7-praktični-primjeri)
8. [Izlazne datoteke](#8-izlazne-datoteke)
9. [Napredne tehnike](#9-napredne-tehnike)
10. [Rješavanje problema](#10-rješavanje-problema)
11. [Često postavljana pitanja](#11-često-postavljana-pitanja)
12. [Tehnički detalji](#12-tehnički-detalji)

---

## 1. Uvod

### Što je pdfgreper?

**pdfgreper** je specijalizirani alat za pretraživanje tekstualnog sadržaja unutar PDF dokumenata. Dizajniran je za istraživače i stručnjake koji trebaju brzo pronaći relevantne dokumente unutar velikih kolekcija znanstvenih radova, tehničkih izvještaja ili drugih PDF materijala.

### Glavne mogućnosti

- **Boolean pretraživanje**: Kombinirajte više pojmova korištenjem `AND` i `OR` operatora
- **Podrška za fraze**: Pretražujte točne višerječne izraze
- **Batch obrada**: Obradite tisuće PDF-ova u jednom prolazu
- **Automatska organizacija**: Pronađeni dokumenti se automatski kopiraju/premještaju
- **Strukturirani izvještaji**: Rezultati se spremaju u CSV format za daljnju analizu
- **Statički kompajlirano**: Jedna binarna datoteka radi na svim Linux distribucijama

### Tipični scenariji korištenja

1. **Sistematski pregled literature** - pronalazak svih radova koji spominju određene ključne pojmove
2. **Filtriranje kolekcije** - izdvajanje relevantnih dokumenata iz veće zbirke
3. **Provjera citiranosti** - traženje radova koji citiraju određene metode ili autore
4. **Ekstrakcija podataka** - identifikacija dokumenata s određenim eksperimentalnim podacima

---

## 2. Instalacija iz prekompajlirane verzije

Ovo je **preporučeni način instalacije** za većinu korisnika. Prekompajlirana verzija je statički linkana i radi na svim modernim Linux distribucijama bez dodatnih ovisnosti.

### 2.1 Sistemski preduvjeti

| Komponenta | Zahtjev |
|------------|---------|
| OS | Linux (x86_64) - bilo koja distribucija |
| RAM | 512 MB (preporučeno 2+ GB za velike kolekcije) |
| Disk | 10 MB za instalaciju + prostor za PDF-ove |
| **pdftotext** | **Obavezno** - iz paketa `poppler-utils` |

### 2.2 Instalacija pdftotext (obavezno)

Prije instalacije pdfgreper-a, morate instalirati `pdftotext`:

#### Ubuntu / Debian
```bash
sudo apt-get update
sudo apt-get install -y poppler-utils
```

#### Fedora / RHEL / CentOS
```bash
sudo dnf install poppler-utils
```

#### Arch Linux / Manjaro
```bash
sudo pacman -S poppler
```

#### openSUSE
```bash
sudo zypper install poppler-tools
```

#### Provjera instalacije
```bash
pdftotext -v
```

### 2.3 Preuzimanje i instalacija pdfgreper-a

#### Metoda 1: Automatska instalacija (preporučeno)

```bash
# 1. Klonirajte repozitorij
git clone https://github.com/KORISNIK/pdfgreper.git
cd pdfgreper

# 2. Pokrenite instalacijsku skriptu
./install.sh
```

Instalacijska skripta će:
- Kopirati binarnu datoteku u `~/.local/bin/`
- Automatski dodati `~/.local/bin` u vaš PATH
- Provjeriti je li pdftotext instaliran

#### Metoda 2: Ručna instalacija

```bash
# 1. Klonirajte repozitorij
git clone https://github.com/KORISNIK/pdfgreper.git
cd pdfgreper

# 2. Kopirajte binarnu datoteku
mkdir -p ~/.local/bin
cp pdfgreper ~/.local/bin/
chmod +x ~/.local/bin/pdfgreper

# 3. Dodajte u PATH (ako već nije)
echo 'export PATH="$PATH:$HOME/.local/bin"' >> ~/.bashrc
source ~/.bashrc
```

#### Metoda 3: Instalacija za sve korisnike (zahtijeva sudo)

```bash
sudo cp pdfgreper /usr/local/bin/
sudo chmod +x /usr/local/bin/pdfgreper
```

### 2.4 Provjera instalacije

```bash
pdfgreper --help
```

### 2.5 Deinstalacija

```bash
# Automatska deinstalacija
./install.sh uninstall

# Ili ručno
rm ~/.local/bin/pdfgreper
```

---

## 3. Kompilacija iz izvornog koda

Ovaj način je namijenjen za:
- Napredne korisnike koji žele modificirati kod
- Korisnike na platformama koje nisu x86_64 Linux
- Razvojne svrhe

### 3.1 Preduvjeti za kompilaciju

#### Rust toolchain

Instalirajte Rust (ako nije već instaliran):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Provjera
rustc --version
cargo --version
```

#### Sistemske ovisnosti

```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# Fedora
sudo dnf install gcc
```

### 3.2 Kompilacija

```bash
# Uđite u source direktorij
cd source

# Standardna release kompilacija
cargo build --release

# Statički linkana kompilacija (preporučeno za distribuciju)
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release
```

Binarna datoteka: `source/target/release/pdfgreper`

### 3.3 Provjera statičkog linkanja

```bash
file source/target/release/pdfgreper
# Trebao bi ispisati: "statically linked"
```

### 3.4 Instalacija kompajlirane verzije

```bash
cp source/target/release/pdfgreper ~/.local/bin/
```

---

## 4. Osnove korištenja

### 4.1 Minimalni primjer

```bash
pdfgreper --bool 'earthworm'
```

### 4.2 Specificiranje direktorija

```bash
pdfgreper --dir /home/korisnik/Documents/radovi --bool 'earthworm'
```

### 4.3 Rekurzivno pretraživanje

```bash
pdfgreper --dir /putanja/do/radova --recursive --bool 'earthworm'
```

### 4.4 Kombiniranje pojmova

```bash
# AND
pdfgreper --bool 'earthworm AND toxicity'

# OR
pdfgreper --bool 'earthworm OR collembola'

# Kombinacija
pdfgreper --bool '(earthworm OR collembola) AND pesticide'
```

### 4.5 Potpuni primjer

```bash
pdfgreper \
  --dir /data/scientific_papers \
  --out /results/pretraga_2024.csv \
  --recursive \
  --bool '("acetylcholinesterase" OR "AChE") AND (earthworm OR earthworms)' \
  --cporrm copy \
  --folder /results/odabrani_radovi
```

---

## 5. Boolean izrazi - detaljna sintaksa

### 5.1 Operatori

| Operator | Opis | Primjer |
|----------|------|---------|
| `AND` | Oba uvjeta moraju biti zadovoljena | `A AND B` |
| `OR` | Barem jedan uvjet mora biti zadovoljen | `A OR B` |
| `( )` | Zagrade za grupiranje | `(A OR B) AND C` |
| `" "` | Navodnici za višerječne fraze | `"exact phrase"` |

### 5.2 Prioritet operatora

| Prioritet | Operator |
|-----------|----------|
| 1 (najviši) | `( )` |
| 2 | `AND` |
| 3 (najniži) | `OR` |

### 5.3 Fraze

```bash
# Višerječna fraza
--bool '"acetylcholine esterase"'

# Kombinacija fraze i pojma
--bool '("acetylcholine esterase" OR AChE) AND earthworm'
```

### 5.4 Case sensitivity

Pretraživanje **nije osjetljivo na velika/mala slova**.

### 5.5 Pravila za shell navodnike

```bash
# Preporučeno: vanjski jednostruki, unutarnji dvostruki
--bool '("acetylcholine esterase" OR AChE) AND earthworm'
```

---

## 6. Opcije naredbenog retka

| Opcija | Opis | Zadano |
|--------|------|--------|
| `--dir <putanja>` | Direktorij s PDF-ovima | Trenutni dir |
| `--out <csv>` | Izlazna CSV datoteka | `rezultati_pdfgreper.csv` |
| `--recursive` | Rekurzivno pretraživanje | Isključeno |
| `--bool <izraz>` | Boolean izraz **(obavezno)** | - |
| `--cporrm <copy\|remove>` | Kopiraj ili premjesti | `copy` |
| `--folder <naziv>` | Ciljni direktorij | `SELECTED_PDFs` |
| `--help` | Pomoć | - |

---

## 7. Praktični primjeri

### 7.1 Sistematski pregled literature

```bash
pdfgreper \
  --dir /literature/downloaded_papers \
  --recursive \
  --bool '("acetylcholinesterase" OR "AChE" OR "cholinesterase") AND ("earthworm" OR "Eisenia" OR "Lumbricus")' \
  --out /results/ache_earthworm_review.csv \
  --folder /results/ache_earthworm_papers
```

### 7.2 Filtriranje prema metodama

```bash
pdfgreper \
  --dir /papers/toxicology \
  --recursive \
  --bool '("HPLC" OR "GC-MS" OR "LC-MS") AND ("pesticide" OR "insecticide")' \
  --out metode_analiza.csv
```

### 7.3 Traženje specifičnih organizama

```bash
pdfgreper \
  --dir /research/soil_fauna \
  --recursive \
  --bool '("collembola" OR "springtail" OR "Folsomia")' \
  --folder Collembola_radovi
```

### 7.4 Premještanje odabranih radova

```bash
pdfgreper \
  --dir /downloads/new_papers \
  --bool '("DEB" OR "dynamic energy budget") AND model' \
  --cporrm remove \
  --folder /archive/DEB_papers
```

### 7.5 Kombinacija vrsta i toksikanata

```bash
pdfgreper \
  --dir /toxicology_archive \
  --recursive \
  --bool '(earthworm OR earthworms) AND (copper OR cadmium OR lead) AND (biomarker OR "oxidative stress")' \
  --out heavy_metals_earthworm.csv
```

---

## 8. Izlazne datoteke

### 8.1 CSV izvještaj

```csv
file,matched,snippet
"Smith2020_AChE.pdf",true,"Abstract: This study investigates..."
"Jones2019_fish.pdf",false,
```

### 8.2 Direktorij s PDF-ovima

```
SELECTED_PDFs/
├── Smith2020_AChE.pdf
└── Wilson2022_biomarkers.pdf
```

### 8.3 Log grešaka

```
pdftotext_failures_pdfgreper.txt
```

---

## 9. Napredne tehnike

### 9.1 Batch obrada

```bash
#!/bin/bash
# batch_search.sh

BASE_DIR="/data/papers"

pdfgreper --dir "$BASE_DIR" --recursive \
  --bool '("AChE" OR "acetylcholinesterase") AND earthworm' \
  --out ache.csv --folder ache_papers

pdfgreper --dir "$BASE_DIR" --recursive \
  --bool '(biomarker OR biomarkers) AND soil' \
  --out biomarkers.csv --folder biomarker_papers
```

### 9.2 Korištenje s cron-om

```bash
# Dodaj u crontab
0 2 * * * pdfgreper --dir /downloads --bool 'earthworm' --out /logs/daily_$(date +\%Y\%m\%d).csv
```

---

## 10. Rješavanje problema

### "pdftotext: command not found"

Instalirajte poppler-utils (vidi sekciju 2.2).

### "Argument --bool je obavezan"

```bash
pdfgreper --dir /data --bool 'vaš izraz'
```

### "Nedostaje zatvorena zagrada"

Provjerite parnost zagrada:
```bash
# POGREŠNO
--bool '((a OR b) AND c'

# ISPRAVNO
--bool '((a OR b) AND c)'
```

### PDF-ovi se ne pronalaze

1. Provjerite PDF-ove: `ls /dir/*.pdf`
2. Testirajte pdftotext: `pdftotext sample.pdf -`
3. Jednostavniji upit: `--bool 'the'`

---

## 11. Često postavljana pitanja

**P: Mogu li koristiti NOT operator?**  
O: Ne, trenutna verzija ne podržava negaciju.

**P: Je li pretraživanje case-sensitive?**  
O: Ne, svi pojmovi se pretvaraju u mala slova.

**P: Radi li na skeniranim PDF-ovima?**  
O: Ne izravno. Potreban je OCR.

**P: Koliko PDF-ova mogu obraditi?**  
O: Nema ograničenja. Testirano na 10.000+ PDF-ova.

**P: Moram li kompajlirati program?**  
O: Ne! Prekompajlirana verzija je uključena u repozitorij.

**P: Na kojim Linux distribucijama radi?**  
O: Na svima - Ubuntu, Debian, Fedora, Arch, openSUSE, Alpine...

---

## 12. Tehnički detalji

### 12.1 Arhitektura

```
main()
  ├── parse_args()      # CLI parsing
  ├── collect_pdfs()    # File discovery
  ├── parse_bool_query()# Expression parsing
  └── Main loop
      ├── pdf_to_text() # Text extraction
      ├── eval_expr()   # Expression evaluation
      └── write_csv()   # Result output
```

### 12.2 Statičko linkanje

Prekompajlirana verzija koristi statičko linkanje:
- Sve biblioteke su ugrađene u binarnu datoteku
- Nema ovisnosti o sistemskim bibliotekama
- Radi na bilo kojoj Linux distribuciji
- Veličina: ~2 MB

### 12.3 Performanse

| Operacija | Kompleksnost |
|-----------|-------------|
| Tokenizacija | O(n) |
| Parsiranje | O(n) |
| Evaluacija | O(m*k) |
| Ukupno | Dominira I/O |

---

*Ovaj softver razvijen je u okviru projekta advanDEB - Napredni modeli dinamičkih energijskih budžeta s transportnom mrežom (IP-2024-05-3615), financiranog od strane Hrvatske zaklade za znanost (HRZZ).*
