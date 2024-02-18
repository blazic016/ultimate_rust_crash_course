use std::thread;
use std::time::Duration;

fn main() {
    // Kreiranje nove niti i pokretanje funkcije u toj niti
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Glavna nit nastavlja sa radom
    for i in 1..=3 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1000));
    }

    // Čekanje da se nit završi (često se koristi kako bi se sačekalo da se sve niti završe pre nego što se program završi)
    handle.join().unwrap();
}