use anyhow::{anyhow, Context, Result};
use csv::Writer;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

struct Config {
    dir: PathBuf,
    out_csv: PathBuf,
    recursive: bool,
    bool_query: String,
    cporrm: String,
    target_folder: PathBuf,
}

fn print_help() {
    eprintln!(
        r#"
  ____   ____ ___ ___  __  __ 
 / ___| / ___|_ _/ _ \|  \/  |
 \___ \| |    | | | | | |\/| |
  ___) | |___ | | |_| | |  | |
 |____/ \____|___\___/|_|  |_|

 Razvijeno za potrebe HRZZ projekta advanDEB
 (Advanced Dynamic Energy Budget models for ecological risk assessment)

─────────────────────────────────────────────────────────────────────

Upotreba: pdfgreper [OPCIJE]

Opcije:
  --dir <putanja>        Direktorij s PDF datotekama (default: current dir)
  --out <csv>            Izlazni CSV (default: rezultati_pdfgreper.csv u --dir)
  --recursive            Rekurzivno pretraži poddirektorije
  --bool <fraza>         Logička fraza za pretragu 
                         (npr. "(ache OR acetylcholinesterase) AND (earthworm OR earthworms)")
  --cporrm <copy|remove> Kopiraj ili premjesti pogođene PDF-ove (default: copy)
  --folder <naziv>       Ciljni poddirektorij za odabrane PDF-ove (default: SELECTED_PDFs)
  --help                 Prikaži ovu pomoć
"#
    );
}

fn parse_args() -> Result<Config> {
    let mut args = std::env::args().skip(1);

    let mut dir: Option<PathBuf> = None;
    let mut out_csv: Option<PathBuf> = None;
    let mut recursive = false;
    let mut bool_query: Option<String> = None;
    let mut cporrm: Option<String> = None;
    let mut target_folder: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--dir" => {
                let value = args
                    .next()
                    .ok_or_else(|| anyhow!("--dir zahtijeva putanju"))?;
                dir = Some(PathBuf::from(value));
            }
            "--out" => {
                let value = args
                    .next()
                    .ok_or_else(|| anyhow!("--out zahtijeva naziv CSV datoteke"))?;
                out_csv = Some(PathBuf::from(value));
            }
            "--recursive" => {
                recursive = true;
            }
            "--bool" => {
                let value = args
                    .next()
                    .ok_or_else(|| anyhow!("--bool zahtijeva logičku frazu"))?;
                bool_query = Some(value);
            }
            "--cporrm" => {
                let value = args
                    .next()
                    .ok_or_else(|| anyhow!("--cporrm zahtijeva vrijednost 'copy' ili 'remove'"))?;
                cporrm = Some(value);
            }
            "--folder" => {
                let value = args
                    .next()
                    .ok_or_else(|| anyhow!("--folder zahtijeva naziv direktorija"))?;
                target_folder = Some(PathBuf::from(value));
            }
            "--help" => {
                print_help();
                std::process::exit(0);
            }
            other => {
                return Err(anyhow!(
                    "Nepoznata opcija: {}. Pokreni s --help za pomoć.",
                    other
                ));
            }
        }
    }

    let base_dir =
        dir.unwrap_or(std::env::current_dir().context("Ne mogu dobiti trenutni direktorij")?);
    let out_csv = out_csv.unwrap_or_else(|| base_dir.join("rezultati_pdfgreper.csv"));
    let bool_query = bool_query.ok_or_else(|| anyhow!("Argument --bool je obavezan"))?;

    let cporrm_value = cporrm.unwrap_or_else(|| "copy".to_string());
    if cporrm_value != "copy" && cporrm_value != "remove" {
        return Err(anyhow!("--cporrm mora biti 'copy' ili 'remove'"));
    }

    let target_folder_path = target_folder.unwrap_or_else(|| base_dir.join("SELECTED_PDFs"));

    Ok(Config {
        dir: base_dir,
        out_csv,
        recursive,
        bool_query,
        cporrm: cporrm_value,
        target_folder: target_folder_path,
    })
}

fn main() -> Result<()> {
    let config = parse_args()?;
    println!("Radni direktorij: {}", config.dir.display());
    println!("Bool fraza: {}", config.bool_query);

    let pdf_files = collect_pdfs(&config.dir, config.recursive)?;
    println!("Nađeno PDF datoteka: {}", pdf_files.len());

    if pdf_files.is_empty() {
        eprintln!(
            "Nema PDF datoteka u: {} (recursive: {})",
            config.dir.display(),
            config.recursive
        );
        return Ok(());
    }

    // Parsiraj bool frazu u AST
    let expr = parse_bool_query(&config.bool_query)?;

    let mut wtr = Writer::from_path(&config.out_csv)?;
    wtr.write_record(&["file", "matched", "snippet"])?;

    // osiguraj da ciljni folder postoji
    if !config.target_folder.exists() {
        fs::create_dir_all(&config.target_folder).with_context(|| {
            format!(
                "Ne mogu kreirati ciljni direktorij: {}",
                config.target_folder.display()
            )
        })?;
    }

    let mut failed_pdftotext: Vec<(PathBuf, String)> = Vec::new();

    let total = pdf_files.len();

    for (idx, pdf_path) in pdf_files.iter().enumerate() {
        let filename = pdf_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("<unknown>");

        let percent = ((idx + 1) as f64 / total as f64) * 100.0;
        println!(
            "[{}/{}] ({:.1}%) obrađujem: {}",
            idx + 1,
            total,
            percent,
            filename
        );

        let text = match pdf_to_text_via_pdftotext(pdf_path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("  pdftotext neuspješan za '{}': {}", filename, e);
                failed_pdftotext.push((pdf_path.clone(), e.to_string()));
                continue;
            }
        };

        let lower = text.to_lowercase();
        let bytes = text.as_bytes();

        if eval_expr(&expr, &lower) {
            // jednostavan snippet: prvih 200 znakova
            let end = std::cmp::min(bytes.len(), 200);
            let snippet = String::from_utf8_lossy(&bytes[0..end]).to_string();
            wtr.write_record(&[filename, "true", &snippet.replace('\n', " ")])?;

            // kopiraj ili premjesti PDF u ciljni folder
            let dest_path = config.target_folder.join(filename);
            if config.cporrm == "copy" {
                fs::copy(pdf_path, &dest_path).with_context(|| {
                    format!(
                        "Ne mogu kopirati '{}' u '{}'",
                        pdf_path.display(),
                        dest_path.display()
                    )
                })?;
            } else {
                // remove: premjesti (rename) u ciljni folder
                fs::rename(pdf_path, &dest_path).with_context(|| {
                    format!(
                        "Ne mogu premjestiti '{}' u '{}'",
                        pdf_path.display(),
                        dest_path.display()
                    )
                })?;
            }
        } else {
            wtr.write_record(&[filename, "false", ""])?;
        }
    }

    wtr.flush()?;

    if !failed_pdftotext.is_empty() {
        eprintln!(
            "pdftotext nije uspio za {} datoteka. Popis:",
            failed_pdftotext.len()
        );
        for (p, msg) in &failed_pdftotext {
            eprintln!("  {} -- {}", p.display(), msg);
        }

        let log_path = config.dir.join("pdftotext_failures_pdfgreper.txt");
        let mut log_file = fs::File::create(&log_path)
            .with_context(|| format!("Ne mogu kreirati log datoteku: {}", log_path.display()))?;
        use std::io::Write;
        for (p, msg) in &failed_pdftotext {
            writeln!(log_file, "{}\t{}", p.display(), msg)?;
        }
        eprintln!(
            "Popis neuspjelih pdftotext poziva zapisan u: {}",
            log_path.display()
        );
    }

    Ok(())
}

#[derive(Debug, Clone)]
enum Token {
    LParen,
    RParen,
    And,
    Or,
    Word(String),
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buf = String::new();

    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' | '\n' => {
                if !buf.is_empty() {
                    tokens.push(Token::Word(buf.to_lowercase()));
                    buf.clear();
                }
            }
            '(' => {
                if !buf.is_empty() {
                    tokens.push(Token::Word(buf.to_lowercase()));
                    buf.clear();
                }
                tokens.push(Token::LParen);
            }
            ')' => {
                if !buf.is_empty() {
                    tokens.push(Token::Word(buf.to_lowercase()));
                    buf.clear();
                }
                tokens.push(Token::RParen);
            }
            '"' => {
                // početak fraze u navodnicima
                if !buf.is_empty() {
                    tokens.push(Token::Word(buf.to_lowercase()));
                    buf.clear();
                }
                let mut phrase = String::new();
                while let Some(&ch) = chars.peek() {
                    chars.next();
                    if ch == '"' {
                        break;
                    }
                    phrase.push(ch);
                }
                if !phrase.is_empty() {
                    tokens.push(Token::Word(phrase.to_lowercase()));
                }
            }
            _ => {
                buf.push(c);
            }
        }
    }
    if !buf.is_empty() {
        tokens.push(Token::Word(buf.to_lowercase()));
    }

    // zamijeni AND/OR tokenima
    let mut normalized = Vec::new();
    for t in tokens {
        match t {
            Token::Word(ref w) if w.eq_ignore_ascii_case("and") => normalized.push(Token::And),
            Token::Word(ref w) if w.eq_ignore_ascii_case("or") => normalized.push(Token::Or),
            other => normalized.push(other),
        }
    }

    normalized
}

#[derive(Debug, Clone)]
enum Expr {
    Term(String),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

fn parse_bool_query(input: &str) -> Result<Expr> {
    let tokens = tokenize(input);
    let (expr, pos) = parse_or(&tokens, 0)?;
    if pos != tokens.len() {
        return Err(anyhow!("Neiskorišteni tokeni u izrazu od pozicije {}", pos));
    }
    Ok(expr)
}

fn parse_or(tokens: &[Token], pos: usize) -> Result<(Expr, usize)> {
    let (mut left, mut p) = parse_and(tokens, pos)?;
    while p < tokens.len() {
        if let Token::Or = tokens[p] {
            let (right, p2) = parse_and(tokens, p + 1)?;
            left = Expr::Or(Box::new(left), Box::new(right));
            p = p2;
        } else {
            break;
        }
    }
    Ok((left, p))
}

fn parse_and(tokens: &[Token], pos: usize) -> Result<(Expr, usize)> {
    let (mut left, mut p) = parse_primary(tokens, pos)?;
    while p < tokens.len() {
        if let Token::And = tokens[p] {
            let (right, p2) = parse_primary(tokens, p + 1)?;
            left = Expr::And(Box::new(left), Box::new(right));
            p = p2;
        } else {
            break;
        }
    }
    Ok((left, p))
}

fn parse_primary(tokens: &[Token], pos: usize) -> Result<(Expr, usize)> {
    if pos >= tokens.len() {
        return Err(anyhow!("Neočekivani kraj izraza"));
    }
    match &tokens[pos] {
        Token::LParen => {
            let (expr, p) = parse_or(tokens, pos + 1)?;
            if p >= tokens.len() {
                return Err(anyhow!("Nedostaje zatvorena zagrada"));
            }
            match &tokens[p] {
                Token::RParen => Ok((expr, p + 1)),
                _ => Err(anyhow!("Očekivana zatvorena zagrada")),
            }
        }
        Token::Word(w) => Ok((Expr::Term(w.clone()), pos + 1)),
        _ => Err(anyhow!("Neočekivan token u izrazu")),
    }
}

fn eval_expr(expr: &Expr, text_lower: &str) -> bool {
    match expr {
        Expr::Term(t) => text_lower.contains(t),
        Expr::And(a, b) => eval_expr(a, text_lower) && eval_expr(b, text_lower),
        Expr::Or(a, b) => eval_expr(a, text_lower) || eval_expr(b, text_lower),
    }
}

fn collect_pdfs(base: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if recursive {
        for entry in walkdir::WalkDir::new(base)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if ext.eq_ignore_ascii_case("pdf") {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }
    } else {
        for entry in fs::read_dir(base)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if ext.eq_ignore_ascii_case("pdf") {
                        files.push(path);
                    }
                }
            }
        }
    }

    Ok(files)
}

fn pdf_to_text_via_pdftotext(pdf_path: &Path) -> Result<String> {
    let txt_path = pdf_path.with_extension("tmp_txt");

    let status = Command::new("pdftotext")
        .arg("-layout")
        .arg(pdf_path)
        .arg(&txt_path)
        .status()
        .with_context(|| format!("Neuspješno pokretanje pdftotext za {}", pdf_path.display()))?;

    if !status.success() {
        let _ = fs::remove_file(&txt_path);
        return Err(anyhow!(
            "pdftotext vratio grešku (exit code: {:?}) za {}",
            status.code(),
            pdf_path.display()
        ));
    }

    let mut file = fs::File::open(&txt_path)
        .with_context(|| format!("Ne mogu otvoriti privremeni txt {}", txt_path.display()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| format!("Ne mogu pročitati privremeni txt {}", txt_path.display()))?;

    fs::remove_file(&txt_path)
        .with_context(|| format!("Ne mogu obrisati privremeni txt {}", txt_path.display()))?;

    Ok(contents)
}
