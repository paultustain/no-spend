use std::fs;
use chrono::{Local, NaiveDate};

slint::slint!{
    
    component Button {
        in property <string> text;
        min-height: 75px;
        min-width: 75px;

        Rectangle {
            background: red;
            border-radius: 4px;
            border-width: 2px;
            border-color: black;
        }

        Text { text: root.text; }

    }
    export component App inherits Window {
        in property <int> value: 0;
        GridLayout {
            padding: 5px;
            spacing: 5px;
            Text { text: "Total Saved" + value; colspan: 2;}
            Row { 
                Button { text: "Stuff Prison";}
                Button { text: "Bank";}
            }
            Row {
                Button { text: "Piggy Bank"; }
                Button { text: "Shop?"; }
            }
        }

    }
}
#[derive(Clone, Debug)]
struct Saving {
    date: String, //NaiveDate,
    item: String, 
    value: String, //f64
}

#[derive(Debug)]
struct PiggyBank {
    savings: Vec<Saving>,
}

impl PiggyBank { 
    pub fn new() -> Self {

        Self { savings: vec![] }
    }

    pub fn fill_from_file(&mut self, path: &str) -> () {
        // TODO - populate from no-spends into the piggy bank.
            
        let contents = fs::read_to_string(path)
            .expect("Couldn't read file");

        for line in contents.lines() {
            let split: Vec<&str> = line.split(",").collect();
            println!("{}", split[1]);
            self.savings.push( Saving {
                date: String::from(split[0]), 
                item: String::from(split[1]), 
                value: String::from(split[2]),
            });
            println!("{:#?}", split);
        }

    }

    pub fn add(&mut self, saving: Saving) {
        // TODO - add a new set of inputs to the piggy bank, and the csv file. 
    }

    // pub fn remove - 
    // TODO - Remove anything from the piggy bank. 

    // pub fn total 
    // TODO - Find the total value stored in the piggy bank and output to the screen. 
}


fn main() {

    // let app = App::new().unwrap();
    let mut pb = PiggyBank::new();
    pb.fill_from_file("no-spends.txt");
    println!("{:#?}", pb);
    // app.run().unwrap();
}
