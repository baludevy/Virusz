#![allow(unused_must_use)]

extern crate msgbox;

use msgbox::IconType;

fn main() {
    let title: String = "Vírusz".to_string();
    let content: String = "Hello. Egy Vírusz vagyok a számítódról. 
    Sajnos nem jól tudtak beprogramozni, ezért sok mindenre nem vagyok képes. 
    Kérlek törölj le valami fontosat helyettem is, és ezt küld tovább valakinek.
    Köszönöm szépen, Vírusz".to_string();

    msgbox::create(&title, &content, IconType::Error);
}