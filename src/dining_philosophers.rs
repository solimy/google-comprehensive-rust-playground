// Five philosophers dine together at the same table.
// Each philosopher has their own place at the table.
// There is a fork between each plate.
// The dish served is a kind of spaghetti which has to be eaten with two forks.
// Each philosopher can only alternately think and eat.
// Moreover, a philosopher can only eat their spaghetti when they have both a left and right fork.
// Thus two forks will only be available when their two nearest neighbors are thinking, not eating.
// After an individual philosopher finishes eating, they will put down both forks.

use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Fork;


struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: SyncSender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...
        loop {
            // println!("{} is hungry...", &self.name);
            if let Ok(_) = self.left_fork.try_lock() {
                // println!("{} has picked up their left fork. ({:?})", self.name, self.left_fork);
                if let Ok(_) = self.right_fork.try_lock() {
                    // println!("{} has picked up their right fork. ({:?})", self.name, self.right_fork);
                    println!("{} is eating...", &self.name);
                    thread::sleep(Duration::from_millis(300));
                    break;
                }
            }
        }
        // println!("{} released his forks ({:?}, {:?})", self.name, self.left_fork, self.right_fork);
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

pub fn main() {
    // Create forks
    let forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<Arc<Mutex<Fork>>>>();
    
    // Create channels
    let (tx, rx) = mpsc::sync_channel::<String>(1);
    
    // Create philosophers
    let philosophers = PHILOSOPHERS
        .iter()
        .enumerate()
        .map(|(i, &name)| {
            let left_fork = forks[i].clone();
            let right_fork = forks[(i + 1) % forks.len()].clone();
            Arc::new(Philosopher {
                name: name.to_string(),
                left_fork,
                right_fork,
                thoughts: tx.clone(),
            })
        })
        .collect::<Vec<Arc<Philosopher>>>();

    // Make them think and eat
    philosophers
    .into_iter()
    .for_each(|p| {
        thread::spawn(move || {
            loop {
                println!("{} is up to something...", &p.name);
                p.think();
                p.eat();
            }
        });
    });

    // print the philosophers' and forks •Φ• Φ •Φ• Φ •Φ•
    thread::spawn(move || {
        
    });

    // Output their thoughts
    loop {
        // println!("waiting for thoughts...");
        match rx.recv() {
            Ok(msg) => println!("{}", msg),
            Err(_) => continue,
        }
    }
}