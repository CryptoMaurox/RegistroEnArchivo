
use std::io;
use std::fs::OpenOptions;
use std::io::prelude::*;

struct Person {
    name: String,
    age: u8,
    address: String,
    bitcoin_address: String,
}

impl Person {
    fn new(name: &str, age: u8, address: &str, bitcoin_address: &str) -> Person {
        Person {
            name: name.to_string(),
            age,
            address: address.to_string(),
            bitcoin_address: bitcoin_address.to_string(),
        }
    }

    fn print_info(&self) {
        println!("Nombre: {}", self.name);
        println!("Edad: {}", self.age);
        println!("Ciudad de residencia: {}", self.address);
        println!("Dirección de bitcoin: {}", self.bitcoin_address);
    }

    fn to_string(&self) -> String {
        format!("Nombre: {}\nEdad: {}\nDirección: {}\nDirección de bitcoin: {}\n",
                self.name, self.age, self.address, self.bitcoin_address)
    }
}

fn main() {
    // Abrimos el archivo de salida
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("registro.txt")
        .expect("No se pudo abrir el archivo");

    let mut name = String::new();
    println!("Introduce el nombre:");
    io::stdin().read_line(&mut name).expect("No se pudo leer el nombre");

    let mut age_str = String::new();
    println!("Introduce la edad:");
    io::stdin().read_line(&mut age_str).expect("No se pudo leer la edad");
    let age: u8 = age_str.trim().parse().expect("La edad debe ser un número");

    let mut address = String::new();
    println!("Introduce la dirección:");
    io::stdin().read_line(&mut address).expect("No se pudo leer la dirección");

    let mut bitcoin_address = String::new();
    println!("Introduce la dirección de bitcoin:");
    io::stdin().read_line(&mut bitcoin_address).expect("No se pudo leer la dirección de bitcoin");

    let person = Person::new(&name.trim(), age, &address.trim(), &bitcoin_address.trim());

    // Escribimos la información de la persona en el archivo de salida
    file.write_all(person.to_string().as_bytes())
        .expect("No se pudo escribir la información en el archivo");

    // Mostramos la información de la persona por pantalla
    person.print_info();
}