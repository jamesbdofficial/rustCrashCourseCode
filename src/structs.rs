// Structs - Used to create custom data types

// Traditional Struct
struct Colour {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct Colours(u8, u8, u8);

struct Person{
    first_name: String,
    last_name: String
}

impl Person {
    // Construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}

pub fn run(){
    // Traditional
    let mut c = Colour{
        red: 255,
        green: 0,
        blue: 0
    };
    println!("Colour: {} {} {}", c.red, c.green, c.blue);

    c.red = 200;
    println!("Colour: {} {} {}", c.red, c.green, c.blue);

    // Tuple
    let mut cs = Colours(0, 0, 255);
    println!("Colours: {} {} {}", cs.0, cs.1, cs.2);

    cs.0 = 150;
    println!("Colours: {} {} {}", cs.0, cs.1, cs.2);

    // using new to create person
    let mut p1 = Person::new("James", "Spum");
    println!("Person: {} {}", p1.first_name, p1.last_name);

    // Get full_name 
    let mut p2 = Person::new("Felix", "Spum");
    println!("Person: {} ", p2.full_name());

    // Set last name
    let mut p3 = Person::new("Sabina", "Gamer");
    println!("Person: {} ", p3.full_name());
    p3.set_last_name("Spum");
    println!("Person: {} ", p3.full_name());

    // Name to tuple
    println!("Person Tuple: {:?} ", p3.to_tuple());
}