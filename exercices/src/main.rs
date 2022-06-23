use std::{env::args, process::exit};

fn main() {
    match args().nth(1).as_deref() {
        None => {
            eprintln!("Indiquez l'exercice à exécuter (ex: cargo run exo1).");
            exit(-1);
        }
        Some("exo1") => exo1::main(),
        Some(other) => {
            eprintln!("Je ne connais pas {other}.");
            exit(-2);
        }
    }
}

mod exo1;
