extern crate yaml_rust;

use std::env;
use std::io;
use std::io::Read;
use std::process::Command;

fn usage() {
    println!("blharg! {:#?}", env::args());
}


fn barf_nbytes ( b:[u8;32], n:usize) {

    print!("in>{:?}:",n);
    for i in 0..n{
        print!("{:x},", b[i])
    }
    print!("\n")

}

fn scroll_up(){
    let outs = Command::new("xdotool")
    .arg("click")
    .arg("4") //down
    .output()
    .expect("failed to execute process");
    println!("{:?}",outs);
}
fn scroll_down(){
    let outs = Command::new("xdotool")
    .arg("click")
    .arg("5") //down
    .output()
    .expect("failed to execute process");
    println!("{:?}",outs);
}

fn filter_n_link ( b:[u8;32], n:usize){
    if b[0] == 0x90 { //keydown
        match (b[1]){
            0x48 => scroll_down(),
            0x47 => scroll_up(), 
            _ => {}
        }
    }
}

fn dirty() {
    let mut ringbuf: [u8;32] = [0x0; 32];
    let  rbp:u8 =  31; 
    let mut stdin = io::stdin();
    while true{
        let mut rdsiz: usize = 0;
        rdsiz = stdin.read(&mut ringbuf).unwrap();
        barf_nbytes(ringbuf, rdsiz);
        filter_n_link(ringbuf, rdsiz);
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
