// Rust Variables and Constants Example

// Constants are immutable and must have explicit type annotations
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

// Constants in Rust should always be written in UPPER_SNAKE_CASE
// Even if you use `mut`, constants remain immutable

fn main() {
    println!("=== Rust Variable Declaration and Mutability ===\n");

    // First example: Immutable variables
    {
        let missiles: i32 = 8; // Explicit type declaration (32-bit signed integer)
        let ready = 2; // Type inferred as i32
        println!("Firing {} of my {} missiles...", ready, missiles);
        // Cannot mutate immutable variable `missiles`
        // missiles = missiles - ready; // This would cause a compilation error
        println!("{} missiles left (immutable variable, so no change)", missiles);
    }

    println!("\n----------------------------------\n");

    // Second example: Mutable variables
    {
        let mut missiles: i32 = 8; // `mut` makes a variable mutable
        let ready: i32 = 2;
        println!("Firing {} of my {} missiles...", ready, missiles);
        missiles = missiles - ready; // Mutating the value of `missiles`
        println!("{} missiles left (mutable variable, value updated)", missiles);
    }

    println!("\n----------------------------------\n");

    // Third example: Using constants and multiple variable binding
    {
        let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
        // Binding the variables all at once using pattern matching
        println!("Firing {} of my {} missiles...", ready, missiles);
        missiles -= ready; // Shorter syntax for subtraction
        println!("{} missiles left (using constants and pattern binding)", missiles);
    }
}
