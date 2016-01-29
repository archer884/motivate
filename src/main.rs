extern crate rand;
extern crate rustc_serialize;
extern crate strsim;
extern crate toml;

mod quotes;

use rand::Rng;
use quotes::{Source, Quote};

enum Author {
    Name(String),
    Any,
}

fn main() {
    let sources = quotes::load("quotes.toml").ok().expect(
        "unable to load quotes.toml"
    );

    let selected_quote = select_quote(&sources, read_name(&sources)).expect(
        "There is a problem with the quote file. Be sure you don't have any empty headings?"
    );

    println!("{}", selected_quote);
}

fn select_quote<'s>(sources: &'s Vec<Source>, author: Author) -> Option<Quote<'s>> {
    let all_quotes: Vec<_> = sources.iter()
        .filter(|source| match author {
            Author::Any => true,
            Author::Name(ref name) => name == source.name(),
        })
        .flat_map(
            |source| (0..source.len()).map(move |idx| Quote::new(&source, idx))
        ).collect();

    rand::thread_rng().choose(&all_quotes).map(|&quote| quote)
}

fn read_name(sources: &Vec<Source>) -> Author {
    match std::env::args().nth(1) {
        None => Author::Any,
        Some(ref requested_name) => {
            let mut names: Vec<_> = sources.iter()
                .filter(|source|
                    source.name().to_lowercase().contains(&requested_name.to_lowercase())
                )
                .map(|source| (
                    strsim::levenshtein(requested_name, source.name()),
                    source.name(),
                ))
                .collect();
            names.sort_by(|a, b| a.0.cmp(&b.0));

            match names.first() {
                None => Author::Any,
                Some(ref name) => Author::Name(name.1.to_owned())
            }
        }
    }
}
