use std::{fs::File, io::{BufRead, BufReader}};
use std::sync::OnceLock;

struct Tables {
    log: [u8; 256],
    exp: [u8; 256],
}

static TABLES: OnceLock<Tables> = OnceLock::new();

fn tables() -> &'static Tables {
    TABLES.get_or_init(|| {
        let file = File::open("log-antilog.csv").expect("Cannot open file");

        let (log, exp): (Vec<u8>, Vec<u8>) = BufReader::new(file)
            .lines()
            .skip(1) // skip header
            .filter_map(Result::ok)
            .map(|line| { 
                    let mut values = line.split(',');

                    let log = values.nth(1).unwrap().parse::<u8>().unwrap();
                    let exp = values.nth(1).unwrap().parse::<u8>().unwrap();

                    (log, exp)
            }).unzip();

        Tables { 
            log: log.try_into().expect("Expected 256 log values"),
            exp: exp.try_into().expect("Expected 256 exp values")
        }
    })
}

fn to_log(val: u8) -> u8 {
    tables().exp[val as usize]
}

fn to_exp(val: u8) -> u8 {
    tables().log[val as usize]
}

pub(super) fn sub(a: u8, b: u8) -> u8 {
    a ^ b
}

pub(super) fn div(a: u8, b: u8) -> u8 {
    assert!(b != 0, "division by zero in GF(256)");

    if a == 0 {
        return 0;
    }

    let a_log = u16::from(to_log(a));
    let b_log = u16::from(to_log(b));

    to_exp(((a_log + 255 - b_log) % 255) as u8)
}

pub(super) fn mul(a: u8, b: u8) -> u8 {
    if a == 0 || b == 0 {
        return 0;
    }

    let a_log = u16::from(to_log(a));
    let b_log = u16::from(to_log(b));

    // Possible overflow wiht u8
    to_exp(((a_log + b_log) % 255) as u8)
}
