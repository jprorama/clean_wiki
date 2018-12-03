extern crate parse_wiki_text as pwt;
extern crate parse_mediawiki_dump as pmd;
extern crate bzip2;

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!("invalid use");
        std::process::exit(1);
    }
    let path = args.nth(1).unwrap();
    let file = match std::fs::File::open(&path) {
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
                println!("{:#?}", page);
            }
        }
    }
}
