fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("Base 10:               {}", 69420);
    println!("Base 2 (binary):       {:b}", 69420);
    println!("Base 8 (octal):        {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);

    // 5文字の幅になるように空白で左埋め
    println!("{number:>5}", number = 1);
    // 5文字の幅になるように0で左埋め
    println!("{number:0>5}", number = 1);
    // 5文字の幅になるように0で右埋め
    println!("{number:0<5}", number = 1);
    // `width` 文字の幅になるように0で左埋め
    println!("{number:0>width$}", number = 1, width = 5);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    let number = 1.0;
    let width: usize = 5;
    // `width` 文字の幅になるように空白で左埋め
    println!("{number:>width$}");

    let pi = 3.141592;

    println!("Pi is roughly {:.2}", pi)
}
