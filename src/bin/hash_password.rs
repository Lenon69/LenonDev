// src/bin/hash_password.rs
use bcrypt::{DEFAULT_COST, hash};
use std::io::{self, Write};

fn main() {
    print!("Wpisz hasło, które chcesz zahashować: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    let hashed_password =
        hash(password.trim(), DEFAULT_COST).expect("Błąd podczas hashowania hasła.");

    println!("\nTwój bezpieczny hash hasła:");
    println!("{}", hashed_password);
}
