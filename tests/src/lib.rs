use std::fs::File;
use std::io::Write;

fn write() {
    let mut file = File::open("nevercalled").unwrap();

    let mystring = "longstring, seyhsdfa, string".to_string();

    file.write(mystring.as_bytes()).unwrap();
}

fn while_let_loop() {
    let x = Some(10);
    loop {
        let y = match x {
            Some(x) => x,
            None => break,
        };
    }
}

fn divide_by_zero() {
    let x = 0 / 0;
}

