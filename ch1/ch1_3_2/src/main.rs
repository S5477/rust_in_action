fn main() {
    greet_world()
}

fn greet_world() {
    println!("Hello, world!");

    let ru = "Привет, мир!";

    let jp = "こんにちは";

    let regions = [ru, jp];

    for region in regions.iter() {
        println!("{}", &region);
    }
}