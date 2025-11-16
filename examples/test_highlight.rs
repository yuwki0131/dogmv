// Test file for syntax highlighting

use std::collections::HashMap;

/// A simple function to demonstrate syntax highlighting
fn main() {
    let mut map = HashMap::new();
    map.insert("key", "value");

    // Print the map
    println!("Map contents: {:?}", map);

    // Test control flow
    for i in 0..5 {
        if i % 2 == 0 {
            println!("{} is even", i);
        } else {
            println!("{} is odd", i);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
