#[cfg(test)]
mod tests_validos {
    use crate::string_validador::StringValidatorConfig;
    use crate::string_validador::StringValidador;

    #[test]
    fn test_1_solo_numeros_exitoso() {
        let config = StringValidatorConfig::new()
            .min_longitud(4)
            .max_longitud(8)
            .min_caracteres_unicos(3)
            .solo_numeros();

        let entrada = "123456";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "telefono", &config);
        println!("Salida: {:?}", resultado);

        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap(), "123456");
    }

    #[test]
    fn test_2_con_simbolos_exitoso() {
        let config = StringValidatorConfig::new()
            .min_longitud(5)
            .max_longitud(20)
            .min_caracteres_unicos(4)
            .con_simbolos(['-', '_', '@']);

        let entrada = "user-123@rust";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "correo_o_usuario", &config);
        println!("Salida: {:?}", resultado);

        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap(), "user-123@rust");
    }

    #[test]
    fn test_3_ascii_imprimible_exitoso() {
        let config = StringValidatorConfig::new()
            .min_longitud(5)
            .max_longitud(30)
            .min_caracteres_unicos(5)
            .ascii_imprimible();

        let entrada = "Hello, World! #2026";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "comentario_ascii", &config);
        println!("Salida: {:?}", resultado);

        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap(), "Hello, World! #2026");
    }

    #[test]
    fn test_4_permitir_todo_exitoso() {
        let config = StringValidatorConfig::new()
            .min_longitud(2)
            .max_longitud(20)
            .min_caracteres_unicos(2)
            .permitir_todo();

        let entrada = "¡Hola 🚀 Rust!";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "texto_libre", &config);
        println!("Salida: {:?}", resultado);

        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap(), "¡Hola 🚀 Rust!");
    }

    #[test]
    fn test_5_default_regla_caracteres_exitoso() {
        let config = StringValidatorConfig::new()
            .min_longitud(3)
            .max_longitud(15)
            .min_caracteres_unicos(3);

        let entrada = "mi_cuenta.2026";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "nombre_usuario", &config);
        println!("Salida: {:?}", resultado);

        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap(), "mi_cuenta.2026");
    }
}
