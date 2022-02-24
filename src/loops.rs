// Loops - used to iterate until a condition is met

pub fn run(){
    let mut count = 0;

    // Infinite Loop
    loop {
        println!("Number: {}", count);
        count += 1;

        if count == 20 {
            break;
        }
    }

    let mut i =0;
    // While Loop (SpumGamer)
    while i <= 100 {
        if i % 15 == 0 {
            println!("Spum Gamer")
        }
        else if i % 3 == 0{
            println!("Spum")
        }
        else if i % 5 == 0 {
            println!("Gamer")
        }
        else {
            println!("{}", i);
        }
        i += 1;
    }

    // For range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("Spum Gamer")
        }
        else if x % 3 == 0{
            println!("Spum")
        }
        else if x % 5 == 0 {
            println!("Gamer")
        }
        else {
            println!("{}", x);
        }
    }
}