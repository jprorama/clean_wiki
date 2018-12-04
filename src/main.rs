extern crate parse_wiki_text as pwt;
extern crate parse_mediawiki_dump as pmd;
extern crate bzip2;

use std::io::Read;
use pmd::Page;
use pwt::{Configuration, Node};

const CHARACTER_SAMPLE: usize = 1000;

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!("invalid use");
        std::process::exit(1);
    }
    let path = args.nth(1).unwrap();
    let mut file = match std::fs::File::open(&path) {
        Err(error) => {
            eprintln!("Failed to open input file: {}", error);
            std::process::exit(1);
        }
        Ok(file) => std::io::BufReader::new(file),
    };
    if path.ends_with(".bz2") {
        parse(std::io::BufReader::new(bzip2::bufread::BzDecoder::new(
            file,
        )));
    } else {
        parse(file);
    }
    //
    //let mut buf = String::new();
    //file.read_to_string(&mut buf).unwrap();
    //println!("{}", clean_text(&buf));
}

fn clean_page(page: &Page) -> String {
    clean_text(&page.text)
}

fn clean_text(string: &String) -> String {
    let mut s = String::new();

    for node in Configuration::default().parse(&string).nodes {
        s += &fold_text_nodes(&node);
    }

    if s.len() > CHARACTER_SAMPLE {
        s[0..CHARACTER_SAMPLE].to_string()
    } else {
	s[0..s.len()].to_string()
    }
}

fn fold_text_nodes(node: &Node) -> String {
    match node {
        Node::Text { value, .. } => {
            //println!("textnode {:?}\n\n", node);
            value.to_string()
        },
        _ => { //println!("skipping {:?}", node);
               "".to_string() }
    }
}

fn parse(source: impl std::io::BufRead) {
    let mut i = -1;

    for result in parse_mediawiki_dump::parse(source) {
        i += 1;

        if i % 1000 != 0 { continue; }

        match result {
            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
            Ok(page) => {
                println!("{}", clean_page(&page));
                //println!("{:#?}", page);
            }
        }
    }
}
