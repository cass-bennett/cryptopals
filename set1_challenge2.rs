use std::io;
use std::io::Write;

fn main() {
    print!("input 1: 0x");
    io::stdout().flush().unwrap();
    let v1 = get_bytes();
    
    print!("input 2: 0x");
    io::stdout().flush().unwrap();
    let v2 = get_bytes();

    let v:Vec<u8> = v1.iter().zip(v2.iter()).map( |(a,b)| a^b ).collect();

    print!("\noutput:  0x");
    for i in v {
        print!("{:02x}", i);
    }
    println!("");
}
fn get_bytes( ) -> Vec<u8> {
    let mut s = String::new();
    
    io::stdin().read_line(&mut s).unwrap();

    let s_len = s.trim_right().chars().count();
    let s_rem = s_len % 2;
    let mut v: Vec<u8> = Vec::new();
    if s_rem == 1 {
        v.push( u8::from_str_radix( &s[..1] , 16 ).unwrap() )
    }
    for i in (0..s_len/2).map(|x| x*2) {
        v.push( u8::from_str_radix( &s[i+s_rem..i+s_rem+2] , 16 ).unwrap() )
    }
    v
}
