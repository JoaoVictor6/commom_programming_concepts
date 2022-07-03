fn main() {
    shadowing();
}

fn shadowing(){
    let x = "   ";

    {
        let x = x.len();
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value ox x is: {x}");
}
