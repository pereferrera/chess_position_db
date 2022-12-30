End goal: database of FEN positions (big tree) from all games parsed
given one position, it gives you the stats (white/black games won, most typical
continuing moves, etc) - can be used as an openings book

-----

database: http://rebel13.nl/dl.html?file=dl/MillionBase%202.5%20(PGN).7z

https://github.com/pleco-rs/Pleco/issues/71
https://crates.io/crates/pgnparse

One can probably implement a Visitor that uses Pleco to update a Board:
maybe we don't even need to use Pleco , use shakmaty instead

https://docs.rs/pgn-reader/latest/pgn_reader/#structs

Basic Rust questions:
- How to run multiple main.rs on a Cargo project
- What are structs, traits, etc

------

To open a file and read it line by line in Rust, you can use the std::fs::File type from the standard library and the BufReader type from the bufreader crate. Here's an example of how you can use these types to read a file line by line:

Copy code
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("file.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}
In this example, we open the file using the File::open method, which returns a Result that indicates whether the file was opened successfully. If the file was opened successfully, we create a new BufReader to read from the file. The BufReader has an Iterator implementation that allows us to iterate over the lines in the file using a for loop.

The line variable in the loop is a Result that contains the line as a String if the line was read successfully, or an error if there was a problem reading the line. We use the ? operator to unwrap the Result and return any error that occurred.

This example assumes that the file is located in the same directory as the Rust program. If the file is located in a different directory, you can specify the full path to the file.

------

To call a function that accepts a string as an argument in Rust, you can pass a string literal or a string variable as the argument.

Here's an example of how to call a function that accepts a string as an argument using a string literal:

Copy code
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("Alice");
}
In this example, the greet function takes a string slice (&str) as an argument. A string literal can be passed directly as a string slice, so we can pass the string literal "Alice" as the argument to the greet function.

Here's an example of how to call a function that accepts a string as an argument using a string variable:

Copy code
fn greet(name: String) {
    println!("Hello, {}!", name);
}

fn main() {
    let name = "Alice".to_string();
    greet(name);
}
In this example, the greet function takes a String as an argument. We create a String variable using the to_string method on the string literal "Alice", and then pass the String variable as the argument to the greet function.

Note that in both of these examples, the greet function takes ownership of the string passed as an argument. If you need to retain ownership of the string, you can pass a reference to the string instead. For example:

Copy code
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let name = "Alice".to_string();
    greet(&name);
}
In this example, the greet function takes a string slice (&str) as an argument. We pass a reference to the String variable name as the argument to the greet function. This allows us to retain ownership of the String while still passing it as an argument to the greet function.

---

If you want to minimize the amount of memory used and have a data structure that can be easily serialized to a memory-mapped file, you might consider using a B-tree.

A B-tree is a self-balancing tree data structure that stores keys and values and allows you to efficiently search, insert, and delete elements. B-trees are well-suited for storing large amounts of data on disk and are often used as the underlying data structure for databases and file systems.

In Rust, you can use the btree crate to create and manipulate B-trees. Here's an example of how to use the btree crate to create a B-tree and insert some elements:

Copy code
extern crate btree;

use btree::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();

    map.insert(1, "foo");
    map.insert(2, "bar");
    map.insert(3, "baz");

    println!("{:?}", map);
}
This will print {1: "foo", 2: "bar", 3: "baz"}.

You can use the get method to look up the value associated with a given key:

Copy code
let value = map.get(&2);
println!("The value associated with the key 2 is {:?}", value);
This will print The value associated with the key 2 is Some("bar").

You can also use the iter method to iterate over the elements of the B-tree in key order:

Copy code
for (key, value) in map.iter() {
    println!("{}: {}", key, value);
}
This will print:

Copy code
1: foo
2: bar
3: baz
To serialize a B-tree to a memory-mapped file, you can use the mmap crate to create a memory-mapped file and then use the serialize and deserialize functions provided by the btree crate to write the B-tree to the file and read it back.

Here's an example of how to serialize a B-tree to a memory-mapped file and read it back:

Copy code
extern crate btree;
extern crate mmap;

use btree::BTreeMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};

fn main() {
    // Create a B-tree and insert some elements
    let mut map = BTreeMap::new();
    map.insert(1, "foo");
    map.insert(2, "bar");
    map.insert(3, "baz");

    // Open a memory-mapped file for writing
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("tree.bin")
        .unwrap();

    // Serialize the B-tree to the file
    btree::serialize(&map, file).unwrap();

    // Open the file for reading
    let file = OpenOptions::new().read(true).open("tree.bin").unwrap();

    // Deserialize the B-tree