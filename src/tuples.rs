// Tuples group together values of different types
// Max 12 elements

pub fn run(){
    
    let person: (&str, &str, i8) = ("James", "St. Albans", 22);
    println!("{} is from {} and is {} years old.", person.0, person.1, person.2);


}