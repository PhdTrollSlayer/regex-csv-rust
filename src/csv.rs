use std::fmt;
use std::fs::File;
use std::io::BufReader;

use regex::Regex;

#[derive(Debug)]
pub struct Item {
    pub raw: String,
    pub rgx: Option<Regex>,
    pub descricao: String,
    pub qtd: i32,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Padrão {} se repetiu {} vezes ({})",
            self.raw, self.qtd, self.descricao
        )
    }
}

pub fn get_csv_itens() -> Vec<Item> {
    let mut itens: Vec<Item> = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(BufReader::new(File::open("catalog.csv").unwrap()));
    let mut v: Vec<String> = Vec::new();

    for result in reader.records() {
        let record = result.unwrap();

        let i = Item {
            raw: record.get(0).unwrap().to_string(),
            rgx: None,
            descricao: record.get(1).unwrap().to_string(),
            qtd: 0,
        };
        v.push(record.get(0).unwrap().to_string());
        itens.push(i);
    }

    let mut fin: Vec<Item> = Vec::new();

    for (i, x) in itens.iter().enumerate() {
        match Regex::new(&x.raw) {
            Ok(r) => {
                fin.push(Item {
                    raw: x.raw.clone(),
                    rgx: Some(r),
                    descricao: x.descricao.clone(),
                    qtd: 0,
                });
            }
            Err(_) => println!("Erro no padrão {}", i),
        }
    }
    fin
}
