use std::env;
use std::fs;
use std::io::ErrorKind;

fn main (){
    let input:Vec<String>=env::args().collect();

    let filename=&input[1];

    println!("filename : {}",filename);

    let contents=fs::read_to_string(filename).unwrap_or_else(|error|{
            if error.kind()==ErrorKind::NotFound{
               // fs::File::create(filename).unwrap_or_else(|error|{panic!("Error in creating file{}",error);})
               panic!("File not found");
            }else{
                panic!("Error in opening file.");
            }
        });

    println!("Contents : \n {}",contents);
    
}