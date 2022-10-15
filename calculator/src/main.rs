use std::io;

fn main() {
    println!("Enter what type of calc u want: ");

    let mut calc = String::new();

    io::stdin().read_line(&mut calc).expect("Failed to read");

    /* So, let's go, i spend like 3 hrs in this, but, he is the ting:
        the read_line fn, includes the terminating newline in the returned string from the input.
        so if we not trim the string, the value will never match, in match calc.
        Also this helps with match, since transform in &str already
    */
    let calc = calc.trim();

    let result = match calc {
        "sum" => sum(1, 2),
        "subtraction" => subtraction(1, 2),
        "division" => division(1, 2),
        "multiplication" => multiplication(1, 2),
        &_ => 0, // we need to pass a default value, like in swtich case
    };

    println!("{}", result);
}

fn sum(n1: i32, n2: i32) -> i32 {
    // typing the return
    return n1 + n2;
}

fn subtraction(n1: i32, n2: i32) -> i32 {
    return n1 - n2;
}

fn division(n1: i32, n2: i32) -> i32 {
    return n1 / n2;
}

fn multiplication(n1: i32, n2: i32) -> i32 {
    return n1 * n2;
}
