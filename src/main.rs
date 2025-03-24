//Rust variables declaration

// fn main(){
//     //32 bit signed integer type
//     let missiles: i32 = 8;
//     let ready = 2;
//     println!("firing {} of my {} missiles", ready, missiles)

// }


// fn main(){
//     let missiles = 8; 
//     let ready = 2;
//     println!("Firing {} of my {} missiles...", ready, missiles);
//     // cannot mutate immutable variable `missiles`
//     // missiles = missiles - ready;
//     //As missiles are immutable variable
//     println!("{} missiles left", missiles); 
// }


fn main(){
    let mut missiles: i32 = 8; 
    let ready: i32 = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles); 
}