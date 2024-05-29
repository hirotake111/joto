use jot::decode_jwt;

fn main() -> Result<(), std::io::Error> {
    let data = std::fs::read_to_string("test/sample.txt")?;
    print_jwt(decode_jwt(data));

    Ok(())
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
