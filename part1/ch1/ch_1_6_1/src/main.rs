//Висячий указатель

#[derive(Debug)]

enum Cereal {
    Barley, 
    Millet,
    Rice,
    Rye,
    Splet,
    Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];

    grains.push(Cereal::Rye);

    drop(grains);

    println!("{:?}", grains);
}