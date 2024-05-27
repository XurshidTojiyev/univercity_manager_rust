pub mod univer_manager {
    pub struct pupil {
        pub name: String,
        pub surname: String,
        pub age: i32,
        pub class: i32,
        pub class_letter: char,
        pub teacher: Teacher
    }

    impl pupil {
        fn new() {}
    }

    #[derive(Debug)]
    pub struct Teacher {
        pub name: String,
        pub surname: String,
        pub age: i32,
        pub subject: String
    }

    impl Teacher {
        pub fn new(name1: String,surname1: String, age1: i32, subject1: String) -> Teacher {
            Teacher{name: name1, surname: surname1, age: age1, subject: subject1}
        }

        pub fn get_name(&self) -> String {
            self.name.to_string()
        }

        pub fn get_surname(&self) -> String {
            self.surname.to_string()
        }

        pub fn get_age(&self) -> i32{
            self.age
        }

        pub fn get_subject(&self) -> String{
            self.subject.to_string()
        }

        pub fn set_surname(&mut self, surname: String){
            self.surname = surname.to_string()
        }
        pub fn set_age(&self, age: i32){}
        pub fn set_name(&mut self, name: String){
            self.name = name.to_string()
        }
        pub fn set_subject(&mut self, subject: String){
            self.subject = subject.to_string()
        }
    }
}