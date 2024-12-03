use std::collections::HashMap;
use std::io;
use std::process::exit;

fn main() {

    while true {
        let mut process_string = String::new();
        println!("Welcome to the statistics App. Press 1 to exit or 2 to continue.");

        io::stdin().read_line(&mut process_string).expect("Failed to read line");

        if process_string.trim() == "2"{
            return;
        }

        println!("Enter a list of numbers seperated by commas (,). Press enter to get median and mode");
        let mut input_string = String::new();
        let mut vector = Vec::new();
        let mut numbered_list = Vec::new();
        io::stdin().read_line(&mut input_string).expect("Failed to read line");

        vector = input_string.trim().split_terminator(',').collect();

        println!("The vector is a list {vector:?}");

        for i in vector {

            let mut parsed_number = i.parse::<i32>();

            match parsed_number {
                Ok(num) => {
                    numbered_list.push(num)
                },
                Err(e)=>{
                    println!("{i} is not a number");
                    break;
                }
            }
        }

        if numbered_list.len() > 0{

            println!("The list is {numbered_list:?}");

            println!("The median of the list is {}",median(&mut numbered_list));

            println!("The mode of the list is {}",mode(&mut numbered_list));


        }else{
            println!("The list must be populated.")
        }

    }


}

fn median(v:&mut Vec<i32>)-> f32{
    v.sort();

    println!("The sorted vector is {v:?} \n");

    let vec_len = v.len();
    let median: f32;

    if vec_len % 2 != 0 {
        let mid_point = vec_len / 2;
        let first_number = v[mid_point - 1];
        let second_number = v[mid_point];

        let total_median = first_number + second_number;


        median = (total_median) as f32 / 2.0;
    }else{
        let mid_point = vec_len / 2;

        median = v[mid_point] as f32;
    }

    return median;
}

fn mode(v:&mut Vec<i32>)-> i32{

    let mut models = HashMap::new();
    let mut mode_number = 0;

    for i in v {
      let count =  models.entry(*i).or_insert(0);
        *count += 1
    }

    for (k,v) in models{
        if v > mode_number {
            mode_number = v;
        }
    }

    return mode_number
}