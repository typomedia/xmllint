extern crate clap;
extern crate xml;

use std::fs::File;
use std::io::BufReader;

use clap::{App, Arg};
use xml::reader::{EventReader, XmlEvent};

fn main() {
    let matches = App::new("xmllint")
        .version("1.0.0")
        .author("Philipp Speck <philipp@typo.media>")
        .about("A clone of xmllint, written in Rust")
        .arg(Arg::new("file")
            .required(true)
            .help("XML file to parse")
        )
        .get_matches();

    let path = matches.value_of("file").unwrap();
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);
    let mut depth = 0;
    
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}