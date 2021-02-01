use parser::tokenizer::Tokenizer;
use parser::token::TokenType;
use parser::parser::Parser;
use std::fs::{File, metadata, read_dir};
use std::path::Path;
use std::env;
use std::io::{BufWriter};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(String::from("Expected path arg"));
    }
    let path = &args[1];
    match metadata(path) {
        Ok(md) => {
            if md.is_dir() {
                parse_dir(&path)
            } else {
                parse_file(&path)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

fn parse_file(path: &str) -> Result<(), String> {
    println!("Processig file {}", path);
    let t = Tokenizer::new(String::from(path)).filter(|b| match b.t {
        TokenType::ICOMMENT(_) | TokenType::COMMENT(_) | TokenType::APICOMMENT(_) => false,
        _ => true,
    } );
    let f = File::create(Path::new(path).with_extension("xml")).expect("Unable to create file");
    let f = BufWriter::new(f);
    let mut p = Parser::new(t, f);
    p.parse()?;
    Ok(())
}

fn parse_dir(path: &str) -> Result<(), String> {
    for entry in read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        println!("file {}", path.to_str().unwrap());
        if path.is_file() {
            if path.extension().unwrap() == "jack" {
                println!("file {} is ok", path.to_str().unwrap());
                parse_file(path.to_str().unwrap())?;
            }
        }
    }
    Ok(())
}
