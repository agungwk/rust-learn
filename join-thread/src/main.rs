use std::thread;
use std::time;

#[derive(Debug)]
struct Person {
    name: String,
}

impl Person {
    fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    fn walk(&self) {
        println!("{} is walking", self.name);
    }

    fn stop(&self) {
        thread::sleep(time::Duration::from_secs(5));
        println!("{} stopped walking", self.name);
    }
}

fn main() {
    let persons = vec![
        Person::new("Maya"),
        Person::new("Dhafin")
    ];

    let mut thread_handler = Vec::new();
    for x in persons {
        thread_handler.push(thread::spawn(move || {
            x.walk();
            x.stop();
        }));
    }

    for h in thread_handler {
        h.join().unwrap();
    }
}
