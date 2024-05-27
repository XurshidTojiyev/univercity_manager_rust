#[allow(dead_code)]
mod mods;
use mods::univer_manager::Pupil;
use crate::mods::univer_manager::Teacher;

fn main() {
    let mut p1: Teacher = Teacher::new("Matuna".to_string(), "Karimova".to_string(), 19, "Math".to_string());
    p1.set_name("Halima".to_string());

    let pup1: Pupil = Pupil::new("Xurshid".to_string(), "Tojiyev".to_string(), 15, 9, 'B', p1);
    println!("{:?}", pup1);
}