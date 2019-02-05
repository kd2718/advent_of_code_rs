use std::fs::File;
use std::io::Read;

fn main() -> Result<(),std::io::Error> {
    let mut file = File::create("input.txt")?;
    //let file = file!("../data/input.txt");


    let mut input = vec![];
    file.read_to_end(&mut input)?;
    for line in input {
        println!("{}", line)
    }

    Ok(())
}
