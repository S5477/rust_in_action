use regex::Regex;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg[short, long]]
    pattern: String,
}
fn main() {
    let args = Args::parse();

    let re = Regex::new(&*args.pattern).unwrap();
    let quote = "\
        Every face, every shop, beadroom window, public-house, and
        dark square is a picture feverishly turned--in search of what?
        It is the same with books.
        What do we seek through millions of pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);


        match contains_substring {
            Some(_) => println!("{}", line),
            None => {}
        }
    }
}
