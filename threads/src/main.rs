use std::thread;
use std::time::Duration;

fn main(){
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("this is {i} from thread");
            thread::sleep(Duration::from_millis(1));
            println!("here's a vector {v:?}");
        }
    });

    for i in 1..5{
        println!("this is {i} from main");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}