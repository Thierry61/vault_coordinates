extern crate rustc_serialize;

use std::env;
use std::process::exit;
use rustc_serialize::hex::FromHex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: vault_cordinate <hex-string>");
        exit(1);
    }
    let ref arg = args[1];
    if arg.len() < 4 {
        println!("<hex-string> {} should countain at least 4 characters", arg);
        exit(1);
    }
    let arg = &arg[0..4];
    let v = arg.from_hex().unwrap_or_else(|e| {
        println!("Invalid <hex-string> {}: {}", arg, e);
        exit(1);
    });
    let w1 = v[0];
    let w2 = v[1];
    let y =  (w1&128)    + ((w1&32)<<1) + ((w1&8)<<2) + ((w1&2)<<3) + ((w2&128)>>4) + ((w2&32)>>3) + ((w2&8)>>2) + ((w2&2)>>1);
    let x = ((w1&64)<<1) + ((w1&16)<<2) + ((w1&4)<<3) + ((w1&1)<<4) + ((w2& 64)>>3) + ((w2&16)>>2) + ((w2&4)>>1) +  (w2&1);
    println!("(x, y) = ({}, {})", x, y);
}
