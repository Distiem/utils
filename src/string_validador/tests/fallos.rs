
#[cfg(test)]
mod tests_invalidos {
    use crate::string_validador::StringValidatorConfig;
    use crate::string_validador::StringValidador;

    #[test]
    fn test_6_error_solo_numeros_con_letras() {
        let config = StringValidatorConfig::new()
            .min_longitud(3)
            .max_longitud(10)
            .min_caracteres_unicos(2)
            .solo_numeros();

        let entrada = "123a56";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "telefono", &config);
        println!("Salida de error: {:?}", resultado);

        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert_eq!(error.campo, "telefono");
        assert!(error.message.contains("caracteres no permitidos"));
    }

    #[test]
    fn test_7_error_longitud_minima() {
        let config = StringValidatorConfig::new()
            .min_longitud(8)
            .max_longitud(15)
            .min_caracteres_unicos(3);

        let entrada = "pass";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "password", &config);
        println!("Salida de error: {:?}", resultado);

        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert_eq!(error.campo, "password");
        assert!(error.message.contains("demasiado corto"));
    }

    #[test]
    fn test_8_error_caracteres_unicos_insuficientes() {
        let config = StringValidatorConfig::new()
            .min_longitud(5)
            .max_longitud(10)
            .min_caracteres_unicos(4)
            .ascii_imprimible();

        let entrada = "aaaaa";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "codigo_repetido", &config);
        println!("Salida de error: {:?}", resultado);

        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert_eq!(error.campo, "codigo_repetido");
        assert!(error.message.contains("caracteres distintos"));
    }

    #[test]
    fn test_9_error_simbolo_no_permitido() {
        let config = StringValidatorConfig::new()
            .min_longitud(3)
            .max_longitud(15)
            .min_caracteres_unicos(3)
            .con_simbolos(['_']);

        let entrada = "user#123";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "usuario", &config);
        println!("Salida de error: {:?}", resultado);

        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert_eq!(error.campo, "usuario");
        assert!(error.message.contains("caracteres no permitidos"));
    }

    #[test]
    fn test_10_error_vacio_o_espacios() {
        let config = StringValidatorConfig::new()
            .min_longitud(4)
            .max_longitud(10)
            .min_caracteres_unicos(2)
            .permitir_todo();

        let entrada = "    ";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "comentario", &config);
        println!("Salida de error: {:?}", resultado);

        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert_eq!(error.campo, "comentario");
        assert!(error.message.contains("no puede estar vacío"));
    }
}
