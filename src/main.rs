mod mods;
use mods::univer_manager::pupil;
use crate::mods::univer_manager::Teacher;

fn main() {
    let mut p1: Teacher = Teacher::new("Matuna".to_string(), "Karimova".to_string(), 19, "Math".to_string());
    p1.set_name("Halima".to_string());

    println!("{:?}", p1);
    println!("Xurshid salom")
}