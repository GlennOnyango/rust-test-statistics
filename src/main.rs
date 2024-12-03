use std::collections::HashMap;


fn main() {
    let mut even_vector = vec![1,2,3,4,3,3,7,8,9,10];
    let mut odd_vector = vec![1,2,3,4,5,2,7];

    println!("The Even median number is {}",median(&mut even_vector));
    println!("The Even mode is {}", mode(& mut even_vector));

    println!("\n");

    println!("The odd median number is {}",median(&mut odd_vector));
    println!("The odd mode is {}", mode(& mut odd_vector));

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