pub fn count(arg: String) {
    let mut count = 0;

    //conditional loop, flow control
    loop {
        count += 1;
        println!("Executed {} times ", count);

        if count >= 8 {
            break; 
        }
    }

    print!("End of flow"); 
}

