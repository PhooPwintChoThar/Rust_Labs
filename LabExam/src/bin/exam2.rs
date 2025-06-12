use std::fs;
use std::io::{self, BufRead};


fn main()->io::Result<()>{
    let file=fs::File::open("poem.txt").unwrap();
    let contents=io::BufReader::new(file);

    let mut lines=0;
    let mut words=0;
    let mut chars=0;

    for line in contents.lines(){
        let line=line?;
        lines+=1;

        words+=line.split_whitespace().count();
        chars+=line.chars().count();
    
        
    }

    println!("Line counts is {}, Word count is {} chars is {}", lines, words, chars);
    Ok(())
    
}