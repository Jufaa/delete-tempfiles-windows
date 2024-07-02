use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Carpeta {
    pub url: String,
}

impl Carpeta {
    pub fn new(url: String) -> Carpeta {
        Carpeta { url }
    }

    pub fn listar_contenido(&self) -> io::Result<Vec<PathBuf>> {
        let path = Path::new(&self.url);
        if !path.is_dir() {
            return Err(io::Error::new(io::ErrorKind::Other, "La ruta no es un directorio"));
        }

        let mut contenido = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            contenido.push(entry.path());
        }
        Ok(contenido)
    }

    pub fn eliminar_data_all(&self) -> io::Result<()> {
        let path = Path::new(&self.url);
        if !path.is_dir() {
            return Err(io::Error::new(io::ErrorKind::Other, "La ruta no es un directorio"));
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                fs::remove_dir_all(entry_path)?;
            } else {
                fs::remove_file(entry_path)?;
            }
        }
        Ok(())
    }
}
