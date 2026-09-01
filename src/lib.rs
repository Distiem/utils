pub mod core;
pub mod enums;

pub use core::space_cleaner::SpaceCleaner;

// ejemplo uso:

// use crate::SpaceCleaner;
// fn mi_funcion() {
//    let resultado = SpaceCleaner::limpiar("Hola  mundo", 1000, false);
// }

#[cfg(test)]
mod obtener_path {
    
    use std::{env, fs};
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct CargoToml {
        package: Package,
    }

    #[derive(Debug, Deserialize)]
    struct Package {
        name: String,
    }

    #[test]
    fn print_cargo_add_command() {
        let package_path = env::current_dir()
            .expect("No se pudo obtener el directorio actual");

        let cargo_toml_path = package_path.join("Cargo.toml");

        let content = fs::read_to_string(&cargo_toml_path)
            .expect("No se pudo leer Cargo.toml");

        let cargo: CargoToml = toml::from_str(&content)
            .expect("Cargo.toml no es un TOML válido");

        println!(
            "\ncargo add {} --path {}\n",
            cargo.package.name,
            package_path.display()
        );
    }
}