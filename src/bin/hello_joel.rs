//Simple adaptation on the hello world default.
//contains some alternate text, a mutable string, and a different way to pass an argument to println.
fn main() {

    //let name: = "Joel";
    let mut msg = "Joel";
    println!("Hello, {}!", msg);

    msg  = "https://doc.rust-lang.org/book/title-page.html";

    println!("Go read this doc {}!", msg);

    //also valid
    //println!("Go read this doc {msg}!");


}
