// use std::collections::HashMap;
// type JSONMap = HashMap<String, JSONValue>;
//
// enum JSONValue {
//     Str(String),
//     Int(i32),
//     Bool(bool),
// }
//
pub fn decode_jwt(text: String) -> Vec<String> {
    let jwt: Vec<String> = text.split(".").take(2).map(decode_part).collect();
    jwt
}

fn decode_part(text: &str) -> String {
    let mut result: Vec<u8> = Vec::new();
    for chunks in text.as_bytes().chunks(8) {
        // (Padding) If the length of chunks is less than 8, then append 'a' as it will
        // be converted into 0 eventually.
        let mut chunks = chunks.to_vec();
        for _ in 0..(8 - chunks.len()) {
            chunks.push(65);
        }
        let mut n: u64 = 0;
        for chunk in chunks {
            n = (n << 6) + lookup(chunk as char);
        }
        result.push((n >> 40) as u8);
        result.push(((n >> 32) & 255) as u8);
        result.push(((n >> 24) & 255) as u8);
        result.push(((n >> 16) & 255) as u8);
        result.push(((n >> 8) & 255) as u8);
        result.push((n & 255) as u8);
    }
    // Remove padding
    while result.len() > 0 && result[result.len() - 1] == 0 {
        result.pop();
    }
    result.into_iter().map(|b| b as char).collect()
}

fn lookup(ch: char) -> u64 {
    match ch {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'J' => 9,
        'K' => 10,
        'L' => 11,
        'M' => 12,
        'N' => 13,
        'O' => 14,
        'P' => 15,
        'Q' => 16,
        'R' => 17,
        'S' => 18,
        'T' => 19,
        'U' => 20,
        'V' => 21,
        'W' => 22,
        'X' => 23,
        'Y' => 24,
        'Z' => 25,
        'a' => 26,
        'b' => 27,
        'c' => 28,
        'd' => 29,
        'e' => 30,
        'f' => 31,
        'g' => 32,
        'h' => 33,
        'i' => 34,
        'j' => 35,
        'k' => 36,
        'l' => 37,
        'm' => 38,
        'n' => 39,
        'o' => 40,
        'p' => 41,
        'q' => 42,
        'r' => 43,
        's' => 44,
        't' => 45,
        'u' => 46,
        'v' => 47,
        'w' => 48,
        'x' => 49,
        'y' => 50,
        'z' => 51,
        '0' => 52,
        '1' => 53,
        '2' => 54,
        '3' => 55,
        '4' => 56,
        '5' => 57,
        '6' => 58,
        '7' => 59,
        '8' => 60,
        '9' => 61,
        '+' => 62,
        '/' => 63,
        _ => 64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let s = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string();
        assert_eq!(decode_part(&s), "{\"alg\":\"HS256\",\"typ\":\"JWT\"}");
        let s = "eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ";
        assert_eq!(
            decode_part(&s),
            "{\"sub\":\"1234567890\",\"name\":\"John Doe\",\"iat\":1516239022}"
        );
    }

    #[test]
    fn test_decode_jwt() {
        let raw_data = std::fs::read_to_string("./test/sample.txt").unwrap();
        assert_eq!(
            decode_jwt(raw_data),
            vec![
                "{\"alg\":\"HS256\",\"typ\":\"JWT\"}",
                "{\"sub\":\"1234567890\",\"name\":\"John Doe\",\"iat\":1516239022}",
            ]
        )
    }
}
