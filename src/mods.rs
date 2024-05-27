pub mod univer_manager {
    #[derive(Debug)]
    pub struct Pupil {
        pub name: String,
        pub surname: String,
        pub age: i32,
        pub class: i32,
        pub class_letter: char,
        pub teacher: Teacher
    }
    
    impl Pupil {
        pub fn new(name1: String, surname1: String, age1: i32, class1: i32, class_letter1: char, teacher1: Teacher)->Pupil {
            Pupil {name: name1, surname: surname1, age: age1, class: class1, class_letter: class_letter1, teacher: teacher1}
        }

        pub fn set_name(&mut self, name: String) {
            self.name = name.to_string()
        }
        pub fn set_surname(&mut self, surname: String){ 
            self.surname = surname.to_string()
        }
        pub fn set_age(&mut self, _age: i32) {
            self.age = _age
        }
        pub fn set_class(&mut self, class: i32) {
            self.class = class;
        }
        pub fn set_class_letter(&mut self, class_letter: char) {
            self.class_letter = class_letter;
        }
        pub fn set_teacher(&mut self, teacher: Teacher) {
            self.teacher = teacher;
        } 
        pub fn get_name(&self) -> String {
            self.name.to_string()
        }
        pub fn get_surname(&self) -> String{
            self.surname.to_string()
        }
        pub fn get_age(&self)->i32 {
            self.age
        }
        pub fn get_class(&self) -> String {
            self.class.to_string()
        }
        pub fn get_class_letter(&self) -> char{
            self.class_letter
        }
        pub fn get_teacher(&self) {
            self.teacher.about()
        }
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

        pub fn about(&self){
            println!("Name: {}", self.name);
            println!("Surname: {}", self.surname);
            println!("Age: {}", self.age);
            println!("Subject: {}", self.subject);
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

        pub fn get_subject(&self) ->String{
            self.subject.to_string()
        }

        pub fn set_surname(&mut self, surname: String){
            self.surname = surname.to_string()
        }
        pub fn set_age(&mut self, _age: i32){
            self.age = _age;
        }
        pub fn set_name(&mut self, name: String){
            self.name = name.to_string()
        }
        pub fn set_subject(&mut self, subject: String){
            self.subject = subject
        }
    }
}
