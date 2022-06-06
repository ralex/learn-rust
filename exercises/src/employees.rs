use std::collections::HashMap;
use std::io;

fn main() {
    // Using a hash map and vectors, create a text interface to allow a user
    // to add employee names to a department in a company. For example,
    // “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then let the user retrieve a list of all people in a department or
    // all people in the company by department, sorted alphabetically.

    let mut employees = HashMap::new();

    println!("Employees: {:?}", employees);

    loop {
        let mut input = String::new();
        println!("What do you want to do? (example: 'Add Sally to Engineering'");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        add_employee(input.trim().split_whitespace().collect(),
                     &mut employees);
    }
}

fn add_employee(words: Vec<&str>, employees: &mut HashMap<String, String>) {
    if words.len() < 4 {
        println!("Wrong number of arguments!");
        print_employees(employees);
    } else {
        match words.get(0) {
            Some(&"Add") => {},
            Some(_) => println!("Request invalid! First word should be 'Add'"),
            None => println!("Request incomplete!"),
        };
        let employee = match words.get(1) {
            Some(e) => e,
            None => ""
        };
        match words.get(2) {
            Some(&"to") => {},
            Some(_) => {},
            None => println!("Request invalid! Third word should be 'to'"),
        };
        let department = match words.get(3) {
            Some(d) => d,
            None => ""
        };

        employees.insert(employee.to_string(), department.to_string());
        print_employees(employees);
    }
}

fn print_employees(employees: &mut HashMap<String, String>) {
    println!("Employees: {:#?}", employees);
}
