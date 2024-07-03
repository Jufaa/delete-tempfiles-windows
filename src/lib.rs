use sysinfo::{ProcessExt, System, SystemExt};
use std::fs;
use std::path::Path;
use std::io;
use std::ptr;
use winapi::um::winbase::GetUserNameW;
use winapi::um::winnt::LPWSTR;

pub struct Carpeta {
    pub url: String,
}

impl Carpeta {
    pub fn new(url: String) -> Carpeta {
        Carpeta { url }
    }

    pub fn obtener_usuario() -> Result<String, String> {
        let mut buffer: [u16; 256] = [0; 256];
        let mut size = buffer.len() as u32;

        unsafe {
            if GetUserNameW(buffer.as_mut_ptr(), &mut size) == 0 {
                return Err("No se pudo obtener el nombre de usuario".to_string());
            }
        }

        let username = String::from_utf16_lossy(&buffer[..(size as usize - 1)]);
        Ok(username)
    }
    pub fn eliminar_data_all(&self) -> io::Result<()> {
        let path = Path::new(&self.url);
        if !path.is_dir() {
            return Err(io::Error::new(io::ErrorKind::Other, "La ruta no es un directorio"));
        }

        // Inicializar el sistema para obtener información sobre los procesos
        let mut system = System::new_all();
        system.refresh_all();

        // Verificar cada archivo en el directorio
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                if let Err(e) = fs::remove_dir_all(&entry_path) {
                    eprintln!("Error al eliminar el directorio {:?}: {}", entry_path, e);
                }
            } else {
                // Intentar eliminar el archivo, si falla, intentar finalizar procesos que lo están usando
                if let Err(_) = fs::remove_file(&entry_path) {
                    for (pid, process) in system.processes() {
                        if process.exe().to_string_lossy().contains("C:\\Windows\\Temp") {
                            println!("Finalizando proceso: {} (PID: {})", process.name(), pid);
                            process.kill();
                        }
                    }

                    // Intentar eliminar el archivo nuevamente
                    if let Err(e) = fs::remove_file(&entry_path) {
                        eprintln!("No se pudo eliminar el archivo {:?}: {}", entry_path, e);
                    }
                }
            }
        }
        Ok(())
    }
}
