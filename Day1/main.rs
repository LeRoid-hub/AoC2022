use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please supply a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    
    part1(&contents);
    part2(&contents); 
}

fn part1(contents: &str){

    let mut most_Calories = 0;
    let mut current_Calories = 0;
    for line in contents.lines() {
        if line.trim().is_empty(){
            if current_Calories > most_Calories {
                most_Calories = current_Calories;
            }
            current_Calories = 0;
            continue;
        }
        
        current_Calories += line.parse::<i32>().unwrap();


    }
    println!("Most Calories: {}", most_Calories);
}

fn part2 (contents: &str){
    let mut most_Calories: Vec<i32> = vec![0,0,0,0] ;
    
    for line in contents.lines(){
        if line.trim().is_empty(){
            most_Calories.sort();
            most_Calories[0] = 0;
            continue;
        }

        let current_Calories = line.parse::<i32>().unwrap();
        most_Calories[0] += current_Calories;
    }
    most_Calories.sort();
    let combined_calories = most_Calories[1] + most_Calories[2] + most_Calories[3];
    println!("Combined Calories: {}", combined_calories);

}
