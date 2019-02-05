use std::fs::File;
use std::io::Read;
use std::io::BufReader;

fn main() -> Result<(),std::io::Error> {
    let file = File::open("input.txt")?;
    //let file = file!("../data/input.txt");


    //let mut input = vec![];
 
    let mut reader = BufReader::new(file);

    let mut input = vec![];

    for ln in reader.read_line()

    reader.read_to_end(&mut input)?;

    println!("{:?}", input);
    

    Ok(())
    
}
