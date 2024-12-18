use std::fmt::Display;

use rand::Rng;
use serde::Serialize;

#[derive(Serialize)]
pub struct Customer {
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "firstName")]
    first_name: String,
    age: u8,
}

impl Customer {
    pub fn new_random(count: usize) -> Vec<Self> {
        let mut result: Vec<Self> = vec![];
        for _ in 0..count {
            result.push(Self {
                first_name: FIRST_NAMES[rand::thread_rng().gen_range(0..FIRST_NAMES.len())]
                    .to_string(),
                last_name: LAST_NAMES[rand::thread_rng().gen_range(0..LAST_NAMES.len())]
                    .to_string(),
                age: rand::thread_rng().gen_range(2..50),
            })
        }
        result
    }
}

impl Display for Customer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ({})", self.first_name, self.last_name, self.age)
    }
}

const FIRST_NAMES: &[&str] = &[
    "John",
    "Jane",
    "Alice",
    "Bob",
    "Joe",
    "Jade",
    "Sarah",
    "Jeremy",
    "Michael",
    "Emily",
    "David",
    "Emma",
    "Daniel",
    "Olivia",
    "Matthew",
    "Sophia",
    "James",
    "Isabella",
    "Joseph",
    "Mia",
    "William",
    "Charlotte",
    "Alexander",
    "Amelia",
    "Henry",
    "Evelyn",
    "Samuel",
    "Abigail",
    "Benjamin",
    "Harper",
    "Lucas",
    "Avery",
    "Jack",
    "Ella",
    "Sebastian",
    "Scarlett",
    "Owen",
    "Grace",
    "Gabriel",
    "Chloe",
    "Elijah",
    "Lily",
    "Logan",
    "Hannah",
    "Isaac",
    "Lillian",
    "Nathan",
    "Addison",
    "Caleb",
    "Aubrey",
    "Ryan",
    "Eleanor",
    "Joshua",
    "Natalie",
    "Andrew",
    "Zoe",
    "Ethan",
    "Leah",
    "Aaron",
    "Hazel",
    "Christian",
    "Violet",
    "Jonathan",
    "Aurora",
    "Thomas",
    "Savannah",
    "Charles",
    "Penelope",
    "Christopher",
    "Stella",
    "Nicholas",
    "Paisley",
    "Dylan",
    "Ellie",
    "Anthony",
    "Nora",
    "Isaiah",
    "Skylar",
    "Adam",
    "Lucy",
    "Connor",
    "Anna",
    "Hunter",
    "Samantha",
    "Cameron",
    "Caroline",
    "Adrian",
    "Madelyn",
    "Evan",
    "Kennedy",
    "Jordan",
    "Aria",
    "Brayden",
    "Ariana",
    "Tyler",
    "Maya",
    "Austin",
    "Autumn",
    "Zachary",
    "Aaliyah",
];

const LAST_NAMES: &[&str] = &[
    "Smith",
    "Johnson",
    "Williams",
    "Brown",
    "Jones",
    "Garcia",
    "Miller",
    "Davis",
    "Rodriguez",
    "Martinez",
    "Hernandez",
    "Lopez",
    "Gonzalez",
    "Wilson",
    "Anderson",
    "Thomas",
    "Taylor",
    "Moore",
    "Jackson",
    "Martin",
    "Lee",
    "Perez",
    "Thompson",
    "White",
    "Harris",
    "Sanchez",
    "Clark",
    "Ramirez",
    "Lewis",
    "Robinson",
    "Walker",
    "Young",
    "Allen",
    "King",
    "Wright",
    "Scott",
    "Torres",
    "Nguyen",
    "Hill",
    "Flores",
    "Green",
    "Adams",
    "Nelson",
    "Baker",
    "Hall",
    "Rivera",
    "Campbell",
    "Mitchell",
    "Carter",
    "Roberts",
    "Gomez",
    "Phillips",
    "Evans",
    "Turner",
    "Diaz",
    "Parker",
    "Cruz",
    "Edwards",
    "Collins",
    "Reyes",
    "Stewart",
    "Morris",
    "Morales",
    "Murphy",
    "Cook",
    "Rogers",
    "Gutierrez",
    "Ortiz",
    "Morgan",
    "Cooper",
    "Peterson",
    "Bailey",
    "Reed",
    "Kelly",
    "Howard",
    "Ramos",
    "Kim",
    "Cox",
    "Ward",
    "Richardson",
    "Watson",
    "Brooks",
    "Chavez",
    "Wood",
    "James",
    "Bennett",
    "Gray",
    "Mendoza",
    "Ruiz",
    "Hughes",
    "Price",
    "Alvarez",
    "Castillo",
    "Sanders",
    "Patel",
    "Myers",
    "Long",
    "Ross",
    "Foster",
    "Jimenez",
];