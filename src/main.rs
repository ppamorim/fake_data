use fake::{Dummy, Fake, Faker};
use rand::rngs::StdRng;
use rand::SeedableRng;
use fake::faker::name::en::FirstName;
use fake::faker::name::en::LastName;
use fake::faker::number::en::Digit;
use fake::faker::name::raw::*;
use fake::locales::*;
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Patient {
    id: usize,
    subject_first_name: String,
    subject_last_name: String,
    subject_id: String,
}

impl Patient {
    fn dummy(id: usize) -> Patient {
        let subject_first_name: String = Name(EN).fake();
        let subject_last_name: String = Name(EN).fake();
        let subject_id: String = format!("{}", id);
        Patient {
            id,
            subject_first_name,
            subject_last_name,
            subject_id
        }
    }
}

fn main() {

    let mut vec = Vec::new();
    for i in 0..1000000 {
        let f: Patient = Patient::dummy(i);
        // println!("{:?}", f);
        vec.push(f);
    }
    let serialized = serde_json::to_string(&vec).unwrap();

    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create("foo.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();

}