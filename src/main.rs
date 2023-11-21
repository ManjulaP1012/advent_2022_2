use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufReader, BufRead };
fn main() -> std::io::Result<()> {
    let file: File = File::open("data/text.txt")?;
    
    let reader = BufReader::new(file);
    let mut points = 0;
    let mut shape_value = HashMap::new();
    shape_value.insert("A",1);
    shape_value.insert("B",2);   
    shape_value.insert("C",3);

    let mut shape_value2 = HashMap::new();
    shape_value2.insert("X",1);
    shape_value2.insert("Y",2);   
    shape_value2.insert("Z",3);

    for line in reader.lines() {
        let line: String = line?;
        let mut line = line.split(" ");
        let player1 = shape_value.get(line.next().unwrap()).unwrap();
        let player2 = shape_value2.get(line.next().unwrap()).unwrap();

        if player1 == player2 {
            points += player2 + 3;
        } else if player1 > player2 {
            points += player2 + 0;
        } else {
            points += player2 + 6;
        }
    }
    println!("Total points: {points}");
    Ok(())
}
