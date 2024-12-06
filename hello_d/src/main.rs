fn main() {
    let num = 3;
    let msg = if num == 5 {
        "five"
    } else if num == 4 {
        "four"
    } else {
        "other"
    };
    println!("{}", msg);

    for num in 1..=10 {
        println!("{}", num);
    }

    let word = "test";

    for char in word.chars() {
        println!("{}", char);
    }
    let char = &word.chars().nth(3);
    println!("{}", char.unwrap());
} 
