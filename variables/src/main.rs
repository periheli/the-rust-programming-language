fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    let mut mut_spaces = "   ";
    // mut_spaces = mut_spaces.len(); // This will cause compile error
}
