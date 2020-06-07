use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub fn mean_median_mode(input: &mut [u64]) -> Vec<u64> {
    let mut result = Vec::with_capacity(3);
    input.sort();
    let sum: u64 = input.iter().sum();
    let mean: u64 = sum / input.len() as u64;
    let median: u64 = input[input.len() / 2];
    let num_values = input
        .iter()
        .fold(HashMap::new(), |mut acc, el| {
            let entry = acc.entry(el).or_insert(0);
            *entry += 1;
            acc
        });
    let (mode, _): (u64, _) = num_values
        .iter()
        .fold((0, 0), |(num, greatest_val), (key, val)| {
            if val > &greatest_val {
                (**key, *val)
            } else {
                (num, greatest_val)
            }
        });

    result.push(mean);
    result.push(median);
    result.push(mode);
    result
}

pub fn pig_latinize(input: &str) -> String {
    let mut output = Vec::new();
    for in_word in input.split_whitespace() {
        let mut word_iter = UnicodeSegmentation::graphemes(in_word, true);
        let first_char = word_iter.next().unwrap();
        if ["a", "e", "i", "o", "u", "y"].iter().any(|c| *c == first_char) {
            output.push(format!("{}-hay", in_word));
        } else {
            let out_word: String = word_iter.take(in_word.len() - 1).collect();
            let out_word = format!("{}-{}ay", out_word, first_char);
            output.push(out_word);
        }
    }
    output.join(" ")
}

pub mod employees {
    use std::collections::HashMap;

    pub struct Departments {
        departments : HashMap<String, Vec<String>>,
    }

    impl Departments {
        pub fn new() -> Departments {
            Departments {
                departments: HashMap::new()
            }
        }
        fn add_person_to_department(&mut self, department: &str, person: &str) {
            let department = self.departments.entry(department.to_string()).or_insert(Vec::new());
            department.push(person.to_string());
            department.sort();
        }

        pub fn parse(&mut self, input: &str) {
            let mut words = input.split_whitespace();
            if words.next().unwrap() != "Add" {
                return;
            }
            let person_name = words.next().unwrap();
            if words.next().unwrap() != "to" {
                return;
            }
            let department_name = words.next().unwrap();
            self.add_person_to_department(department_name, person_name);
        }

        pub fn people_in_departments(&self) -> Vec<(String, String)> {
            let mut result: Vec<(String, String)> = Vec::new();
            for (department, people) in &self.departments {
                for person in people {
                    result.push((department.clone(), person.clone()));
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use fake::Fake;
    use fake::faker::name::raw::*;
    use fake::faker::lorem::raw::*;
    use fake::locales::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn mmm() {
        let mut input: Vec<u64> = (1..11).collect();
        input.push(5);
        let result = mean_median_mode(&mut input);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 5);
        assert_eq!(result[1], 5);
        assert_eq!(result[2], 5);
    }

    #[test]
    fn pl() {
        let input = String::from("first");
        let output = pig_latinize(&input);
        assert_eq!("irst-fay", output);

        let input = String::from("apple");
        let output = pig_latinize(&input);
        assert_eq!("apple-hay", output);

        let input = String::from("first apple");
        let output = pig_latinize(&input);
        assert_eq!("irst-fay apple-hay", output);
    }

    #[test]
    fn departments() {
        let mut departments = crate::employees::Departments::new();
        let mut people: Vec<(String, String)> = Vec::new();
        for _ in 0..10 {
            let name: String = FirstName(EN).fake();
            let industry: String = Word(EN).fake();
            departments.parse(&format!("Add {} to {}", name, industry));
            people.push((industry, name))
        }
        let result = departments.people_in_departments();
        println!("{:#?}", result);
        println!("{:#?}", people);
        for (department, person) in result {
            println!("{}: {}", department, person);
            let index = people.iter().position(|(d, p)| *d == department && *p == person).unwrap();
            people.remove(index);
        }
        assert_eq!(people.len(), 0);
    }
}

