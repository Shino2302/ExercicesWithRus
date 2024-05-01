//Invocamos como módulo el archivo my_functions.rs para utilizar todas las funciones publicas que tenga
mod my_functions;

// * NOTA:
// * A la hora de crear archivos con funciones separadas,
// * preferentemente crearlas a la altura de archivo main.rs. Para mayor facilidad...


fn main() {
    println!("Bienvenido!");
    println!("¿Qué operación deseas realizar?");
    println!("1. Suma ");
    println!("2. Resta");
    println!("3. División");
    println!("4. Multiplicación");
    print!(":");
    let _eleccion: i32 = 0;
    println!("Tu resultado es {}", my_functions::sumar_dos_numeros(1.0, 10.0));
}
