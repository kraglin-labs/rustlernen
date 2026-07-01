use std::io::{self, Write}; 
enum Action {
    Pick(Item),
    Move(Grid),
}

#[derive(Debug, Clone, PartialEq)]
enum Directions {
    North,
    South,
    West,
    East,
}
#[derive(Debug, Clone)]
enum ItemKind {
    Potion {description: String, health: u32},
    Weapon {description: String, ap: u32},
    Poison {description: String, loss: u32},
    None,
}

#[derive(Debug, Clone)]
enum Rooms {
    StartRoom,
    SaunaOfDeath,
    MuseumOfAwesomeStuff,
}

#[derive(Debug, Clone)]
struct Grid {
    x: u8,
    y: u8,
}

#[derive(Debug, Clone)]
struct Room {
    name: String,
    kind: Rooms,
    location: Grid,
    item: ItemKind,
    doors: Vec<Directions>,
}

#[derive(Debug)]
struct Item {
    name: String,
    kind: ItemKind,
}

#[derive(Debug)]
struct Player {
    name: String,
    current_room: Room,
    inventory: Vec<Item>,
    health: u32,
}

impl Player {
    fn new( name: &str, room: Room ) -> Self {
        Self {
            name: name.to_string(),
            current_room: room,
            inventory: Vec::new(),
            health: 0,
        }
    }
    fn take_potion(&mut self) {
        if let ItemKind::Potion {health, ..} = self.current_room.item {
            self.health += health;
            println!("Added +{} health", health); 
        } else {
            println!("There is no potion here");
        }
        
    }
    fn pickupweapon(&mut self, item: Item) {
        if let ItemKind::Weapon{description, ap} = &item.kind {
            println!{"picked up {} with {} AP", &description, ap};
            self.inventory.push(item);
        } else {
            println!("no weapons found here");
        }
    }
    fn move_to(&mut self, dir: Directions, world_map: &Vec<Room>) {
        if !self.current_room.doors.contains(&dir) {
            println!("That's a fucking wall, You lose 8 health points");
            self.health = self.health.saturating_sub(8); 
            return;
        }

        let mut next_x = self.current_room.location.x;
        let mut next_y = self.current_room.location.y;

        match dir {
            Directions::North => next_y += 1,
            Directions::South => next_y = next_y.saturating_sub(1), 
            Directions::East => next_x += 1,
            Directions::West => next_x = next_x.saturating_sub(1),
        }

        let destination = world_map.iter().find(|room| {
            room.location.x == next_x && room.location.y == next_y
        });

        match destination {
            Some(neuer) => {
                self.current_room = neuer.clone();
                println!("You have successfully moved to {}", self.current_room.name);
            } 
            None => { println!("There is a door, but no room behind it"); }
        }

        if let ItemKind::Poison{ ref description, loss } = self.current_room.item {
            println!("{}", description);
            println!("You lost {} health points", loss);
            self.health = self.health.saturating_sub(loss);
        }
        println!("Welcome to {}", self.current_room.name);
    }
}

fn turn(user: &mut Player, world: &Vec<Room>) {
    let mut input = String::new();
    
    print!("\nWhat do you want to do? (take, move, show, quit): ");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    match input.trim().to_lowercase().as_str() {
        "take" => user.take_potion(),
        "move" => {
            let mut input1 = String::new();
            print!("How do you want to move? (north, south, east, west): ");
            io::stdout().flush().unwrap();

            io::stdin().read_line(&mut input1).expect("Failed to read line");
            
            match input1.trim().to_lowercase().as_str() {
                "north" => user.move_to(Directions::North, world),
                "west" => user.move_to(Directions::West, world), 
                "east" => user.move_to(Directions::East, world),
                "south" => user.move_to(Directions::South, world),
                _ => println!("That's not a valid direction!"),
            }
        }
        "show" => {
            println!("\n---- Player Status ----");
            println!("Name: {}", user.name);
            println!("Health: {}", user.health);
            println!("Current room: {:#?}", user.current_room);
        }
        "quit" => {
            println!("Thanks for playing!");
            std::process::exit(0);
        }
        _ => println!("Unknown action!"),
    }
}

fn main(){
    let start_room = Room {
        name: String::from("Starting room"),
        kind: Rooms::StartRoom,
        location: Grid {x: 0, y: 0},
        item: ItemKind::Potion {
            description: String::from("The beginning of the journey"),
            health: 100,
        },
        doors: vec![Directions::West, Directions::North],
    };

    let sauna = Room {
        name: String::from("Sauna room"),
        kind: Rooms::SaunaOfDeath,
        location: Grid {x: 0, y: 1},
        item: ItemKind::Poison {
            description: String::from("The fucking worse heatstroke ever"),
            loss: 31,
        },
        doors: vec![Directions::West, Directions::South],
    };

    let mut world: Vec<Room> = Vec::new();
    world.push(start_room.clone());
    world.push(sauna);
    
    let mut user = Player::new("Sakis", start_room);

    println!("Game Started! You are in the Starting room.");
    
    loop {
        turn(&mut user, &world);
    }
}