use std::collections::HashMap;

fn main() {

    println!("TODO App!");

    let _action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");



    let mut todo = Todo {
        map: HashMap::new(),
    };

    if 1 == 1 {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }

    // println!("{:?}, {:?}", action, item)
}

struct Todo {
    map: HashMap<String, bool>
}

impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }

        std::fs::write("db.txt", content)

    }
}