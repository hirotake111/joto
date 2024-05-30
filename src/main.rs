use joto::decode_jwt;

fn main() {
    let mut argv = std::env::args();
    argv.next().unwrap();
    let input = match argv.next() {
        Some(input) => input,
        None => {
            // read input from stdin
            let mut line = String::from("");
            std::io::stdin().read_line(&mut line).unwrap();
            line
        }
    };
    print_jwt(decode_jwt(input));
}

fn print_jwt(data: Vec<String>) {
    let tab = "    ".to_string();
    let mut tn = 0;
    for part in data {
        for ch in part.chars() {
            match ch {
                '{' => {
                    tn += 1;
                    print!("{}\n{}", ch, tab.repeat(tn));
                }
                '}' => {
                    tn -= 1;
                    print!("\n{}{}", ch, tab.repeat(tn));
                }
                ',' => {
                    print!("{}\n{}", ch, tab.repeat(tn));
                }
                ':' => print!(": "),
                _ => print!("{ch}"),
            }
        }
        println!("");
    }
}
