const STARTING_CHAR: char = 'g';
const STARTING_CHAR_U32: u32 = STARTING_CHAR as u32;
pub const TO_HEX: [char; 16] = [
    '0',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9',
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
];

fn count_to_char(count: usize) -> char {
    let count = count - 3;
    return char::from_u32(STARTING_CHAR_U32 + count as u32).unwrap();
}

fn char_to_count(c: char) -> u32 {
    return (c as u32 - STARTING_CHAR_U32) + 3;
}

fn inflate(data: &String) -> Result<String, std::io::Error> {
    return data
        .chars()
        .try_fold((1, String::new()), |(count, mut buf), c| {
            let mut next_count = 1;
            if c as u32 >= STARTING_CHAR_U32 {
                if count > 1 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid encoding of string provided by twitch, those dick heads.",
                    ));
                } else {
                    next_count = char_to_count(c);
                }
            } else {
                for _ in 0..count {
                    buf.push(c);
                }
            }

            return Ok((next_count, buf));
        })
        .map(|(_, buf)| buf);
}

fn deflate(data: &String) -> String {
    return data
        .chars()
        .collect::<Vec<char>>()
        .as_slice()
        .group_by(|a, b| a == b)
        .map(|group| {
            return group.chunks(22).fold(String::new(), |mut buf, group| {
                if group.len() > 2 {
                    buf.push(count_to_char(group.len()));
                    buf.push(group[0]);
                } else {
                    for c in group {
                        buf.push(*c);
                    }
                }
                return buf;
            });
        })
        .collect::<String>();
}

pub fn encode(data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let enc = hex::encode(data);
    let enc = deflate(&enc);
    return Ok(enc);
}

// TODO: Again, look at error handling...
pub fn decode(data: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = inflate(&data)?;
    println!("DECODE: {}", data);
    return Ok(hex::decode(data)?);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deflate() -> Result<(), Box<dyn std::error::Error>> {
        let s = "aabcccad";
        assert_eq!(deflate(&s.to_string()), "aabgcad");

        let s = "aabcccaaaaaaaaaaaaaaaaaaaaaaad";
        assert_eq!(deflate(&s.to_string()), "aabgczaad");
        return Ok(());
    }

    #[test]
    fn test_inflate() -> Result<(), Box<dyn std::error::Error>> {
        let s = "aabgcad";
        assert_eq!(inflate(&s.to_string())?, "aabcccad");

        return Ok(());
    }

    #[test]
    fn test_both() -> Result<(), Box<dyn std::error::Error>> {
        let s = "aabcccaaaaaaaaaaaaaaaaaaaad";
        assert_eq!(inflate(&deflate(&s.to_string()))?, s);

        return Ok(());
    }

    #[test]
    fn test_encode() -> Result<(), Box<dyn std::error::Error>> {
        let s = hex::decode("aabcccaaaaaaaaaaaaaaaaaaaaaaad")?;

        assert_eq!(encode(&s)?, "aabgczaad");
        return Ok(());
    }
}



