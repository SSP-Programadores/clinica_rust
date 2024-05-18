
use chrono::NaiveDate;

fn main() {
    println!("Hello, world!");
    let paciente = Paciente{ 
        nombre: "Nombre".to_string(), apellido1: "Apellido1".to_string(), 
        apellido2: "Apellido2".to_string(), fecha_nacimiento: 
        NaiveDate::parse_from_str("1974-10-12", "%Y-%m-%d").ok().expect("No es correcto"), 
        dni: "42222254D".to_string(), telefono: "934543434".to_string() 
    };
    println!("Nombre: {}", paciente.nombre);
    println!("Primer apellido: {}", paciente.apellido1);
    println!("Segundo apellido: {}", paciente.apellido2);
    println!("Fecha de nacimiento: {}", paciente.fecha_nacimiento);
    println!("Dni: {}", paciente.dni);
    println!("Teléfono: {}", paciente.telefono);
}

struct Paciente {
    nombre: String,
    apellido1: String,
    apellido2: String,
    fecha_nacimiento: NaiveDate,
    dni: String,
    telefono: String,
}

struct Enfermero {
    nombre: String,
    apellido1: String,
    apellido2: String,

}