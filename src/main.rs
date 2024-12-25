extern crate yaml_rust;

use std::env;
use std::io;
use std::io::Read;

fn usage() {
    println!("blharg! {:#?}", env::args());
}

fn dirty() {
    let mut ringbuf: [u8;32] = [0xaf; 32];
    let  rbp:u8 =  31; 
    let mut stdin = io::stdin();
    while true{
        let mut rdsiz: usize = 0;
        rdsiz = stdin.read(&mut ringbuf).unwrap();
        println!("got{:?}>{:?}",rdsiz,ringbuf);
    }

}
fn main() {
    // 
    let argcl: Vec<String>  = env::args().collect();
    println!("lenarg{:?}", argcl.len());
    if (argcl.len() < 2 ){
        usage();
    }
    dirty(); 
}
