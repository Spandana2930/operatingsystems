fn main() {
    println!("Hello, world!");

    #[derive(Debug)]
    #[allow(dead_code)]  // This suppresses the "dead code" warning
    struct Student {
        name: String,
        major: String,
    }

    impl Student {
        fn new_student() -> Student {
            Student {
                name: "Spandana".to_string(),
                major: "Computer Science".to_string(),
            }
        }
    }

    let s = Student::new_student();
    println!("{:?}", s);  // Print the student details
}