// 1. Define a trait named `Bite`
//
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.  Once this trait is defined, you should be able to run the program with
// `cargo run` without any errors.
//
//  trait Bite...

trait Bite {
    fn bite(self: &mut Self);
}



// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// use a different field, though).
//

struct Grapes {
    total_left: i32,
}

// 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// If you need a hint, look at how it was done for Carrot at the bottom of this file.

impl Bite for Grapes {
    // reduce by 1
    fn bite(self: &mut Self) {
        self.total_left -= 1;
    }
}

fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // 4. Uncomment and adjust the code below to match how you defined your
    // Grapes struct.
    
    // constructs a new Grapes Instance
    let mut grapes = Grapes { total_left: 100 };

    // execute the grapes bite function
    grapes.bite();

    // after grapes have been eaten
    println!("Eat a grape: {:?}", grapes.total_left);

    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)
    //

    fn bunny_nibbles<T:Bite>(refer: &mut T) {
        refer.bite();
        refer.bite();
        refer.bite();
        refer.bite();
    }
    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}