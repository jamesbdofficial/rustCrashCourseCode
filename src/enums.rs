// Enums are types which have a few definite values

enum Movements {
    //Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movements){
    // Perform action depending on info
    match m {
        Movements::Up => println!("You have moved up"),
        Movements::Down => println!("You have moved down"),
        Movements::Left => println!("You have moved left"),
        Movements::Right => println!("You have moved right")
    }
}

pub fn run(){
    let avatar1 = Movements::Left;
    let avatar2 = Movements::Up;
    let avatar3 = Movements::Right;
    let avatar4 = Movements::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}