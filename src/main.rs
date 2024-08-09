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
        //println!("{} {}", player1, player2);
        if player1 == player2 {
            points += player2 + 3;
        } else if *player1 == 1 {
            if *player2 == 2 {
                points += 6 + player2;
            } else {
                points += *player2;
            }
        } else if *player1 == 2 {
            if *player2 == 3 {
                points += 6 + player2;
            } else {
                points += *player2;
            }
        } else {
            if *player2 == 1 {
                points += 6 + player2;
            } else {
                points += *player2;
            } 
        }
    }
    println!("Total points: {points}");

    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    Ok(())
}
