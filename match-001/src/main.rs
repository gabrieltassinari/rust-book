#[derive(Debug)]
enum Estado {
    Alabama,
    Alaska,
    // ... etc
}

enum Moeda {
    Penny,
    Nickel,
    Dime,
    Quarter(Estado),
}

fn valor_em_cents(moeda: Moeda) -> u32 {
    match moeda {
        Moeda::Penny => {
            println!("Moeda da sorte!");
            1
        },
        Moeda::Nickel => 5,
        Moeda::Dime => 10,
        Moeda::Quarter => {
            println!("Quarter do estado {:#:?}!", estado);
            25
        },
    }
}

fn main() {
    println!("Hello, world!");
}
