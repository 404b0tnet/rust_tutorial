/*
https://doc.rust-lang.org/std/fmt/

Printing is handled by a series of macros defined in std::fmt some of which include:

format!: write formatted text to String
print!: same as format! but the text is printed to the console (io::stdout).
println!: same as print! but a newline is appended.
eprint!: same as format! but the text is printed to the standard error (io::stderr).
eprintln!: same as eprint!but a newline is appended.

*/

use std::{
    fmt::{self, Formatter},
    fs::write,
    process::Command,
};

pub fn run() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:0>width$}", number = 1, width = 6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    //println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.

    let pi: f32 = 3.141592;
    println!("Pi is roughtly {number:.3}", number = &pi);
    println!("Pi is roughtly {number:.prec$}", number = &pi, prec = 3);
    println!("Pi is roughtly {0:.3}", pi);

    /***********
    fmt::Debug vs fmt::Display
    In order to use {:?} or {:#?} the debug trait must be derived or manually implemented
        ie. #[derive(Debug)] or impl fmt::Debug for structName {...}
    ***********/
    // This structure cannot be printed either with `fmt::Display` or
    // with `fmt::Debug`.
    #[allow(dead_code)]
    struct UnPrintable(i32);

    // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`.
    #[derive(Debug)]
    struct Structure(i32);
    // Put a 'Structure' inside of the structure 'Deep' also makes it printable
    #[derive(Debug)]
    struct Deep(Structure);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    // So fmt::Debug definitely makes this printable but sacrifices some elegance. Rust also provides "pretty printing" with {:#?}.
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    let ast = "******";
    // Print
    println!("{}\nPrint\n\t{:?}", ast, &peter);
    // Pretty print
    println!("{}\nPretty Print\n{:#?}", ast, &peter);

    // Implement std::format::Display manually
    // manually customize the output
    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        // this trait requires 'fmt' with this exact signature
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output stream: 'f'
            // Resturns 'fmt::Result' which indicates whether the operation succeeded or failed.
            // Note that 'write!' uses syntax which is very similar to 'println!()'.
            // Use `self.number` to refer to each positional data point.
            write!(f, "{}, {}", self.0, self.1)
        }
    }


    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );


    // Define a structure where the fields are nameable for comparison.
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    // Similarly, implement `Display` for `Point2D`
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);


    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // The trait Binary is not implemented for f64. Can only be signed/unsigned int
    // println!("What does Point2D look like in binary: {:b}?", point);

    /*
    After checking the output of the above example, use the Point2D struct as a guide to add a Complex struct to the example. When printed in the same way, the output should be:
    */
    #[derive(Debug)]
    struct Complex {
        x: f32,
        y: f32,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

            if self.x < 0.0 && self.y < 0.0 {
                let newx = self.x * -1.0;
                let newy = self.y * -1.0;
                write!(f, "{}i + {}i", newx, newy)
            } else if self.y < 0.0 {
                let newy = self.y * -1.0;
                write!(f, "{} + {}i", self.x, newy)
            } else if self.x < 0.0 {
                let newx = self.x * -1.0;
                write!(f, "{}i + {}", newx, self.y)
            } else {
                write!(f, "{} + {}", self.x, self.y)
            }
        }
    }

    /*
    Display: 3.3 + 7.2i
    Debug: Complex { real: 3.3, imag: 7.2 }
    */
    let mut variable = Complex {x: 3.3, y: -7.2};
    println!("Display: {}", variable);

    variable = Complex {x: -3.3, y: 7.2};
    println!("Display: {}", variable);

    variable = Complex {x: -3.3, y: -7.2};
    println!("Display: {}", variable);
}
