use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec::Vec;
use std::env;


fn main() 
{
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let target_val: u32 = 512;
    let component_num: u8 = 2;
    let component_type: &str = "IND"; // IND, RES, CAP
    let standard_size: Vec<String> = read_file();
    // for x in standard_size
    // {
    //     println!("{}", x);
    // }

    // INDUCTOR IS RESISTORS ARE THE SAME
    // CAPACITANCE IS THE OPPOSITE

    if target_val != 0 && component_num != 0 //then we are calcing closest to the val
    {

    }
    else if target_val != 0 && component_num == 0 // then we are calcing the number of components needed to get the correct value
    {

    }
    
    println!("Done");
}


// parallel for RES and IND, series for CAP
fn calc_parallel_components(values: Vec<u32>) -> f32
{
    let mut sum: f32 = 0.0;
    for x in values
    {
        sum = sum + 1.0/(x as f32);
    }
    return 1.0/sum;
}


// Series for RES and IND, parallet for CAP
fn calc_series_components(values: Vec<u32>) -> f32
{
    let mut sum: f32 = 0.0;
    for x in values
    {
        sum = sum + (x as f32);
    }
    return sum;
}

// fn calc_limitted_components(u32: target_val, u8: component_num, &str: component_type) -> Vec<u32>
// {
    
// }

// fn process_args(Vec<String>) -> (u32, u8, String)
// {
//     return (0, 0);
// }


fn read_file() -> Vec<String>
{
    let path = Path::new("standard_size.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    
    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("Read {} Successfully\n", display),
    }

    let mut vec: Vec<String> = Vec::new();
    for x in s.split(",")
    {
        vec.push(x.trim_start().to_string());
    }

    return vec;
}