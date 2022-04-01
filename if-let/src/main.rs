fn main() {
    #[derive(Debug)]
    enum Estado {
        Alabama,
        Alaska,
    }

    enum Moeda {
        Penny,
        Nickel,
        Dime,
        Quarter(Estado),
    }

    let moeda = Moeda::Penny;
    let mut contagem = 0;
    if let Moeda::Quarter(estado) = moeda {
        println!("Quarter do estado {:#?}!", estado);
    } else {
        contagem += 1;
        println!("Contagem está em {}", contagem)
    }
}
