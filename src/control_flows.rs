pub fn run() {
    if_statement();
}

fn if_statement() {
    let temp = 29;
    // expand style
    if temp < 10 {
        println!("Today is cool.");
    } else if temp > 30 {
        println!("Today is host")
    } else {
        println!("Today is OK.");
    }
    // inline style
    let today = if temp < 10 {
        "Cool"
    } else if temp > 30 {
        "Sunny"
    } else {
        "Ok"
    };
    println!("Today is {}", oday);
}
