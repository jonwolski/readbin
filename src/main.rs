use std::io::{self, Read};

fn decode<I>(char_stream: I) -> Option<char>
where
    I: Iterator<Item = char>,
{
    let mut nibble = char_stream.take(4);
    let a = nibble.next();
    let b = nibble.next();
    let c = nibble.next();
    let d = nibble.next();
    match (a, b, c, d) {
        (Some('0'), Some('0'), Some('0'), Some('0')) => Some('0'),
        (Some('0'), Some('0'), Some('0'), Some('1')) => Some('1'),
        (Some('0'), Some('0'), Some('1'), Some('0')) => Some('2'),
        (Some('0'), Some('0'), Some('1'), Some('1')) => Some('3'),
        (Some('0'), Some('1'), Some('0'), Some('0')) => Some('4'),
        (Some('0'), Some('1'), Some('0'), Some('1')) => Some('5'),
        (Some('0'), Some('1'), Some('1'), Some('0')) => Some('6'),
        (Some('0'), Some('1'), Some('1'), Some('1')) => Some('7'),
        (Some('1'), Some('0'), Some('0'), Some('0')) => Some('8'),
        (Some('1'), Some('0'), Some('0'), Some('1')) => Some('9'),
        (Some('1'), Some('0'), Some('1'), Some('0')) => Some('a'),
        (Some('1'), Some('0'), Some('1'), Some('1')) => Some('b'),
        (Some('1'), Some('1'), Some('0'), Some('0')) => Some('c'),
        (Some('1'), Some('1'), Some('0'), Some('1')) => Some('d'),
        (Some('1'), Some('1'), Some('1'), Some('0')) => Some('e'),
        (Some('1'), Some('1'), Some('1'), Some('1')) => Some('f'),
        _ => None,
    }
}

fn decode_binary_string(input: &str) -> String {
    let mut char_stream = input.chars().filter(|&c| c != ' ');
    // Each nibble is one char. Two nibbles per octet. Input is 8 chars + 1 space per octet, thus 2/9 times input length
    let mut output = String::with_capacity(input.len() * 2 / 9);
    while let (Some(decoded), Some(decoded2)) = (decode(&mut char_stream), decode(&mut char_stream))
    {
        output.push(decoded);
        output.push(decoded2);
    }
    output
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;
    println!("{}", decode_binary_string(&buffer));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let expected = "48656c6c6f2c20776f726c6421";
        let input = "01001000 01100101 01101100 01101100 01101111 00101100 00100000 01110111 01101111 01110010 01101100 01100100 00100001";
        assert_eq!(decode_binary_string(input), expected);
    }
}
