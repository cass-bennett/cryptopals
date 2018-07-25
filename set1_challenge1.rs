use std::io;
use std::io::BufRead;
use std::str;
use std::u16;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();

    let padding = match input.trim_right().chars().count() % 3 {
        0 => "",
        1 => "00",
        _ => "0",
    };

    // The hexadecimal input string, with appropriate padding
    let hex = format!("{}{}",padding,input.trim_right());
    let len = hex.chars().count();

    let mut v: Vec<String> = Vec::new();
    for i in (0..len/3).map(|x| x*3) {
        v.push( hex_to_64( &hex[i..i+3] ) );
    }
    let s = v.join("");
    let start = s.find(|c: char| c!='A').unwrap();
    println!("{}",&s[start..]);
}

// Converts a value from 3 characters of hex to 2 characters of base64
fn hex_to_64 ( s: &str ) -> String {
    let num = match u16::from_str_radix( s, 16 ) {
        Ok(n) => n,
        Err(error) => {
            println!("error: {}", error);
            0
        }
    };

    let digit1 = find_ascii_value( num >> 6 );
    let digit2 = find_ascii_value( num & 0x3f );
    let digits = vec![ digit1, digit2 ];
    
    str::from_utf8( digits.as_slice() ).unwrap().to_string()
}

// Takes an int and returns the ascii codepoint of the character that would
// represent that int in base64 notation
fn find_ascii_value ( n: u16 ) -> u8 {
    let r = {
        if n<26 { n+65 }      // capital letters
        else if n<52 { n+71 } // lowercase letters
        else if n<62 { n-4 }  // digits
        else if n<63 { 43 }   // plus sign
        else {47}             // slash
    };
    r as u8
}

