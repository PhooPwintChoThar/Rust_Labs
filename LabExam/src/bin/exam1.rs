use std::thread;

fn main(){
    let mut handles=vec![];
     for x in 0..5{
        let handle=thread::spawn(move || { 
            println!("Square of {} is {}", x , x*x);

        });

        handles.push(handle);
     }

     for c in handles{
        c.join().unwrap();
     }
}