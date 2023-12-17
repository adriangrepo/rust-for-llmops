use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

// Mutex that protects the data vector, and then we spawn three threads
//that each acquire a lock on the mutex and modify an element of the vector.

pub fn channel_main() {
    let data = Mutex::new(vec![1, 2, 3]);
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let data_clone = data.lock().unwrap().clone();
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let mut data = data_clone;
            data[i] += 1;
            tx_clone.send(data[i]).unwrap();
        });
    }

    let mut modified_data = Vec::new();
    for _ in 0..3 {
        modified_data.push(rx.recv().unwrap());
    }

    println!("{:?}", modified_data);
}
