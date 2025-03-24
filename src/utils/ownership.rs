// Ownership Rules
// Each value in Rust has a single owner (variable).
// When the owner goes out of scope, the value is dropped (freed from memory).
// You can transfer ownership (move), borrow (immutable or mutable), or clone data.

// Memory Safety: No need for garbage collection.
// Performance: Prevents unnecessary memory allocations.
// Concurrency: Eliminates data races at compile time.

pub fn ownership() {
    println!("-----OWNERSHIP-----");

    // Move Semantics (Ownership Transfer)
    {
        let s1 = String::from("Hello from s1 to s2");
        let s2 = s1; // Ownership moves to s2, s1 is no longer valid
        // println!("{}", s1); ❌ ERROR! s1 no longer owns the value
        print!("Onership to {}", s2);
    };

    println!("\n \n");
    // Clone (Deep Copy)
    {
        let s1 = String::from("AWESOME");
        let s2 = s1.clone(); // Creates a deep copy
        println!("Data from S1 {} , Copied data in s2 {}", s1, s2); // ✅ Works, because s1 still owns its data
    }

    println!("\n \n");
    // Borrowing (References)
    {
        fn print_length(s: &String) {
            println!("Length: {}", s.len());
        }

        let s = String::from("Hello");
        print_length(&s); // Borrowing s (read-only)
        println!("{}", s); // ✅ Works, ownership not moved
    }

    println!("\n \n");
    // Mutable Borrowing
    {
        fn make_uppercase(s: &mut String) {
            s.push_str(" WORLD!");
        }

        let mut s = String::from("Hello");
        make_uppercase(&mut s); // Mutable borrow
        println!("{}", s); // ✅ Hello WORLD!

        //You can have multiple immutable references (&T) OR only one mutable reference (&mut T).
        // Prevents data races in concurrent programs.
    }

    println!("\n \n");
    //Slices (References to Parts of Data)
    {
        let s = String::from("hello world");
        let hello = &s[0..5]; // "hello"
        let world = &s[6..]; // "world"

        println!("{} {}", hello, world)
    }
}
