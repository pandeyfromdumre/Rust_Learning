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


}