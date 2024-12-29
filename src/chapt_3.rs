fn main(){
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x now is {x}");

    let tup: (i32, f64, bool) = (4, 2.2, false);

    let four = tup.0;
    let two_dot_two = tup.1;
    let False = tup.2;

    let months = ["January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // commonly used to set known number of fixed collection. They are saved in stack


    let mut count = 0;

    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1
        }
        count += 1
    }
    println!("End count = {count}");

    for number in (1..5).rev(){
        println!("{number}");
    }
    let mut s1 = String::from("Hey there!");
    let length = calculate_len(&mut s1);
    println!("The length of the string is {length}");

    let test_sentence = String::from("Hey there hope you are doing good");
    first_word(&test_sentence);
}

fn calculate_len(someString: &mut String) -> usize{
    someString.push_str("How have you been?");
    someString.len()
} // Here s goes out of scope. But because it does not have ownership of what it refers to, the String is not dropped.

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    println!(bytes.iter());
    for (i, &item) in bytes.iter().enumerate(){ // it's as per what enumerate returns; it returns index and reference to the element
        if item == b' '{
            return i;
        }
    }
    s.len()
}

enum Option<T>{
    None,
    Some(T),
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn Plus_One(val : Option<i32>) -> Option<i32>{
    match val{
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = Plus_One(five);
let none = Plus_One(None);


// In the case when we need the value

let dice_roll = 9;
match dice_roll{
    1 => get_hat(),
    2 => remove_hat(),
    other => do_something(other),
}

// In the case when we don't need the value
match dice_roll{
    1 => get_hat(),
    2 => remove_hat(),
    _ => (), // for the case when we need to implement nothing happens
}

fn get_hat(){}
fn remove_hat(){}
fn rerell(){}




