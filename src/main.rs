mod csv;
use crate::csv::*;

use scraper::{Html, Selector};

fn main() {
    let mut rgx = csv::get_csv_itens();

    scrape(&mut rgx);
}

fn scrape(rgx: &mut Vec<Item>) {
    let mut links: Vec<String> = Vec::new();

    let url_request: String = "https://www.unoesc.edu.br/".to_string();

    let loader = reqwest::get(&url_request).unwrap().text().unwrap();
    let output = String::from(loader);

    let document = Html::parse_document(&output);

    let selector_link = Selector::parse("a").unwrap();

    for element in document.select(&selector_link) {
        let l = element
                .value()
                .attr("href");
             
        links.push(
            match l{
                Some(x) => x.to_string(),
                None => {String::new()}
            }
        );
    }

    for (i,l) in links.iter().enumerate() {
        if i > 10  {break}
        if !l.is_empty() && l.starts_with("http") {
            let loader = match reqwest::get(l){
                Ok(mut s) => s.text().unwrap(),
                Err(_) => String::new(),
            };

            for e in rgx.iter_mut() {
                for _ in e.rgx.clone().unwrap().find_iter(&loader) {
                    e.qtd += 1;
                }
            }
        }
    }

    let mut fin: Vec<&Item> = Vec::new();

    for e in rgx.iter() {
        if e.qtd > 0 {
            fin.push(e);
        }
    }

    for e in fin {
        println!("{}", e);
    }
}
