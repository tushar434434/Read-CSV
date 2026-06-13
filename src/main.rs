use std::error::Error;//error ko handle ke ley
use csv; //csv ka use kr rhe hai 

fn read_from_file(path : &str) -> Result<(), Box<dyn Error>> {// it return error if file not found
    let mut reader =csv::Reader::from_path(path)?;// it return error if file not found
    for result in reader.records(){//loop chala diya hai records ke liye
        let record = result?;// it return error if file not found
        println!("{:?}",record);// it print the record in the file
    }
    Ok(())
}
fn main(){
    if let Err(e) =read_from_file("./customers.csv"){ // it give error if file not found 
        eprintln!("{}",e); //explain the error with discription
    }
}