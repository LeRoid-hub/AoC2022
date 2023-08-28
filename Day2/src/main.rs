use std::env;
use std::fs;
use std::collections::HashMap;
fn main() {
    let file_path = env::args().nth(1).expect("Please supply a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    part1(&contents);
}

fn part1(contents: &str){
    let move1 = HashMap::new();
    move1.insert("X", 1);    //Stein
    move1.insert("Y", 2);    //Papier
    move1.insert("Z", 3);    //Schere
    let move2 = HashMap::new();
    move2.insert("A", 1);
    move2.insert("B", 2);
    move2.insert("C", 3);
    let mut score = 0;
    for line in contents.lines() {
        let players = line.chars().collect();
           if (move1.get(players[2]) == move2.get(players[0])) {
                score += 3;
           }else if (move1.get(players[2]) > move2.get(players[0]) || move1.get(players[2]) == move1.get("X") && move2.get(players[0]) == move2.get("C")){
                score += 6;
           }
        score += move1.get(players[2]);
    }
    println!("Score: {}", score);
}


