fn main() {
   
    let name_nightclub:String = "Disco baby Disco".to_string();

    println!("");
    println!("{}",name_nightclub);
    println!("----------");
    println!("");



    let pill_color : String = ask_user_for ("Elige la pildora (roja/azul:");

    if pill_color == "roja" {
        println! ("Entras a la matrix!");
    }else if pill_color =="azul" {
        println!("Vuelves al mundo real");
    }
    else {
        println!("No es una opcion valida");
    }
}

fn ask_user_for (label: &str) -> String{

    let mut user_awnser: String =

String::new();
println!("{}", label);
std::io::stdin().read_line(&mut user_awnser).unwrap();

user_awnser.trim().to_lowercase().to_string()
}



