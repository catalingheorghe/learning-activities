// Rust wraps native threads of the os, so should be cross-platform
//
// spawn (closure) -> handle
// handle -> join -> result (anything that implements the Send trait)
//
// Note:
// async await - more efficient to wait for IO, but more advanced
//
// Sending data between threads - channels
// std::sync::mpsc - not recommended for usage (design features)
// crossbeam::channel - this is the one to use
//
// channel == one way queue
// Send trait - all primitives implement this, and for the rest the compiler
// usually implements it when compiling
//
// - bounded - fix capacity - sender blocks
// - unbounded - size of channel grows
//
// can have multiple receivers; undefined which receiver gets the value
// can have multiple senders
//
// bidirection -> need multiple channels (careful to not block each other)
//
//

// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn expensive_sum(v: Vec<i32>) -> i32 {
    // Pretend our fancy little filter-map-sum is expensive and takes 500ms
    sleep_ms(500);
    println!("Child thread: just about finished");
    v.iter().filter(|&x| x % 2 == 0).map(|x| x * x).sum()
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    // child thread that does something on the vector above
    let handle = thread::spawn(move || expensive_sum(my_vector));

    // While the child thread is running, the main thread will also do some work
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Processing the letter '{}'", letter);
        sleep_ms(200);
    }

    let result = handle.join();
    let sum = result.unwrap();
    println!("The child thread's expensive sum is {}", sum);

    // 3. Time for some fun with channels!
    // - Uncomment the block comment below (Find and remove the `/*` and `*/`).
    // - Create variables `tx` and `rx` and assign them to the sending and receiving ends of an
    // unbounded channel. Hint: An unbounded channel can be created with `channel::unbounded()`

    // unbounded channel - sender, receiver; then, another sender
    let (tx, rx) = channel::unbounded();
    let tx2 = tx.clone();

    // Thread A
    let handle_a = thread::spawn(move || {
        sleep_ms(250);
        tx2.send("Thread A: 1").unwrap();
        sleep_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });

    sleep_ms(100); // Make sure Thread A has time to get going before we spawn Thread B

    // Thread B
    let handle_b = thread::spawn(move || {
        sleep_ms(0);
        tx.send("Thread B: 1").unwrap();
        sleep_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    // Using a Receiver channel as an iterator is a convenient way to get values until the
    // channel gets closed. A Receiver channel is automatically closed once all Sender
    // channels have been closed.
    //
    // Both our threads automatically close their Sender channels when they exit and the
    // destructors for the channels get automatically called.
    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    // 5. Oops, we forgot to join "Thread A" and "Thread B". That's bad hygiene!
    // - Use the thread handles to join both threads without getting any compiler warnings.

    handle_a.join().unwrap();
    handle_b.join().unwrap();

    // Challenge: Make two child threads and give them each a receiving end to a channel.  From the
    // main thread loop through several values and print each out and then send it to the channel.
    // On the child threads print out the values you receive. Close the sending side in the main
    // thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`).  Join
    // the child threads.

    let (tx, rx) = channel::unbounded::<i32>();
    let rx2 = rx.clone();

    fn temp_recv(rx: channel::Receiver<i32>) {
        for msg in rx {
            println!("Child Thread {:?}: Received {}", thread::current().id(), msg);
        }
    }

    let handle_a = thread::spawn(move || {
        temp_recv(rx);
    });
    let handle_b = thread::spawn(move || {
        temp_recv(rx2);
    });

    for x in vec![2, 42, 30, 17, 53, 60, 89, 7] {
        println!("Main thread: Sending {}", x);
        tx.send(x).unwrap();
    }
    drop(tx); // this will make the channel close, when empty

    // note: drop() is implementing just by moving the variable and not using it

    handle_a.join().unwrap();
    handle_b.join().unwrap();

    println!("Main thread: Exiting.")
}
