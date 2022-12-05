// Feeling things out
// enum with variant values for class/race... make a impl method based on sub class/race?
// use the match enum binding... somewhere. Maybe with the read_lines return?
use mariadb::connect_to_database;
use sqlx::Result;
mod mariadb;

enum Class {
    Fighter,
    Ranger,
    Rogue,
    Cleric,
}

struct Character {
    name: String,
    class: String,
    race: String,
    level: i32,
    age: i32,
    size: String,
    height: String,
    weight: i32,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
}

impl Character {
    fn armor_class(&self) -> i32 {
        self.dexterity + 10
    }
    // NOTE: add conditional for class/race etc
    fn hit_points(&self) -> i32 {
        self.constitution + 10
    }
}

fn parse_args() -> Result<Character, String> {
    let mut character = Character::default();
    let mut fields = vec![
        "Name",
        "Class",
        "Race",
        "Level",
        "Age",
        "Size",
        "Height",
        "Weight",
        "Strength",
        "Dexterity",
        "Constitution",
        "Intelligence",
        "Wisdom",
        "Charisma"];

    loop {
        println!("Please enter your Character's Stats:");
        for field in fields {
            println!("{}:",  field);
            let value = read!("{}\n");
            match field {
                "Name" => character.name = value,
                "Class" => character.class = value,
                "Race" => character.race = value,
                "Level" => character.level = value,
                "Age" => character.age = value,
                "Size" => character.size = value,
                "Height" => character.height = value,
                "Weight" => character.weight = value,
                "Strength" => character.strength = value,
                "Dexterity" => character.dexterity = value,
                "Constitution" => character.constitution = value,
                "Intelligence" => character.intelligence = value,
                "Wisdom" => character.wisdom = value,
                "Charisma" => character.charisma = value,
                _ => (),
            }
            fields.remove(field);
        }
        if field.is_empty() {
            break;
        }
    }
    Ok(character)
}

//Secret Message: my favorite number is 33.




fn main() {
    let char1 = Character {
        name: "Burn Battlehammer".to_string(),
        class: "Fighter".to_string(),
        race: "Dwarf".to_string(),
        level: 1,
        age: 175,
        size: "Medium".to_string(),
        height: "4'6".to_string(),
        weight: 350,
        strength: 3,
        dexterity: 1,
        constitution: 3,
        intelligence: -1,
        wisdom: 1,
        charisma: 0,
    };

    print!("The character created is: {}\n", char1.name);
    print!("They are a level {} {}\n", char1.level, char1.class);
    print!(
        "And as such, their armor class is {} and their HP are {}\n",
        char1.armor_class(),
        char1.hit_points()
    );

    connect_to_database();
}
