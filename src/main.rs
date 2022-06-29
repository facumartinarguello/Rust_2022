fn main() {
    println!("Por favor, ingrese su nombre:");

    let mut nombre : String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    
    
    
    // obtener la edad de la consola

    println!("Por favor, ingrese su edad:");
    let mut edad : String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    // convertir la edad a un numero

    let edad_int : u8 = edad.trim().parse().unwrap();
  

    //obtener el pais del usuario
    
    println!("Por favor, ingrese su pais de procedencia:");

    let mut pais : String = String::new();
    std::io::stdin().read_line(&mut pais).unwrap();
    pais = pais.trim().to_string();


    //obtener el dni

    println!("Por favor, ingrese su DNI:");
    let mut DNI : String = String::new();
    std::io::stdin().read_line(&mut DNI).unwrap();


    // Impresion en consola del formulario

    println! ("Hola, bienvenido o bienvenida {} de {} años, procedente de {} con DNI {}",nombre, edad_int, pais,DNI);

}
