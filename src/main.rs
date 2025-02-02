use delete_files_temporal::Carpeta;
use std::io;

fn main() {
    loop {
        println!("Hola bienvenido :D!");
        println!("------------------------------");
        println!("Por favor selecciona una opción:");
        println!("1. Eliminar C:\\Windows\\Temp");
        println!("2. Eliminar C:\\Windows\\Prefetch");
        println!("3. Eliminar %temp%");
        println!("4. Eliminar todos");
        println!("------------------------------");

        let usuario = Carpeta::obtener_usuario().unwrap();
        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).unwrap();
        let opcion: u32 = match opcion.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor ingresa un número");
                continue;
            }
        };


        match opcion {
            1 => {
                let carpeta = Carpeta::new("C:\\Windows\\Temp".to_string());
                match carpeta.eliminar_data_all() {
                    Ok(_) => {
                        println!("Se eliminó el contenido de la carpeta C:\\Windows\\Temp");
                    }
                    Err(e) => {
                        eprintln!("No se pudo eliminar el contenido de la carpeta: {}", e);
                    }
                }
            }
            2 => {
                let carpeta = Carpeta::new("C:\\Windows\\Prefetch".to_string());
                match carpeta.eliminar_data_all() {
                    Ok(_) => {
                        println!("Se eliminó el contenido de la carpeta C:\\Windows\\Prefetch");
                    }
                    Err(e) => {
                        eprintln!("No se pudo eliminar el contenido de la carpeta: {}", e);
                    }
                }
            }
            3 => {
                let carpeta = Carpeta::new(format!("C:\\Users\\{}\\AppData\\Local\\Temp", usuario));
                match carpeta.eliminar_data_all() {
                    Ok(_) => {
                        println!("Se eliminó el contenido de la carpeta C:\\Users\\%username%\\AppData\\Local\\Temp");
                    }
                    Err(e) => {
                        eprintln!("No se pudo eliminar el contenido de la carpeta: {}", e);
                    }
                }
            }
            4 => {
                let mut urls = vec![
                    "C:\\Windows\\Temp",
                    "C:\\Windows\\Prefetch",
                ];

                let ruta_temp = format!("C:\\Users\\{}\\AppData\\Local\\Temp", usuario);
                urls.push(&ruta_temp);

                for url in urls {
                    let carpeta = Carpeta::new(url.to_string());
                    match carpeta.eliminar_data_all() {
                        Ok(_) => {
                            println!("Se eliminó el contenido de la carpeta {}", url);
                        }
                        Err(e) => {
                            eprintln!("No se pudo eliminar el contenido de la carpeta {}: {}", url, e);
                        }
                    }
                }
            }
            _ => {
                println!("Opción no válida");
            }
        }
    }
}