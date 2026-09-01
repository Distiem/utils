#[cfg(test)]
mod tests_adicionales {
    use crate::string_validador::StringValidatorConfig;
    use crate::string_validador::StringValidador;

    #[test]
    fn test_longitud_exacta_maxima_exitoso() {
        let config = StringValidatorConfig::new()
            .min_longitud(2)
            .max_longitud(8)
            .min_caracteres_unicos(2)
            .ascii_imprimible();

        let entrada = "12345678";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "codigo", &config);
        println!("Salida: {:?}", resultado);
        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap(), "12345678");
    }

    #[test]
    fn test_longitud_excede_maxima_falla() {
        let config = StringValidatorConfig::new()
            .min_longitud(2)
            .max_longitud(8)
            .min_caracteres_unicos(2)
            .ascii_imprimible();

        let entrada = "123456789";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "codigo", &config);
        println!("Salida de error: {:?}", resultado);
        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert!(error.message.contains("excede el límite"));
    }

    #[test]
    fn test_espacios_trim_no_eliminados_sin_opcion() {
        let config = StringValidatorConfig::new()
            .min_longitud(3)
            .max_longitud(10)
            .min_caracteres_unicos(2)
            .ascii_imprimible();

        let entrada = "  hola  ";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "saludo", &config);
        println!("Salida: {:?}", resultado);
        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap(), "hola");
    }

    #[test]
    fn test_espacios_internos_no_permitidos_por_regla() {
        let config = StringValidatorConfig::new()
            .min_longitud(2)
            .max_longitud(10)
            .min_caracteres_unicos(1)
            .solo_numeros();

        let entrada = "12 34";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "numero", &config);
        println!("Salida de error: {:?}", resultado);
        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert!(error.message.contains("caracteres no permitidos"));
    }

    #[test]
    fn test_eliminar_todo_espacio_con_longitud_original_excedida_falla() {
        let config = StringValidatorConfig::new()
            .min_longitud(5)
            .max_longitud(6)
            .min_caracteres_unicos(3)
            .ascii_imprimible()
            .eliminar_todo_espacio();

        let entrada = " a b c d e f ";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "texto", &config);
        println!("Salida de error: {:?}", resultado);
        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert!(error.message.contains("excede el límite"));
    }

    #[test]
    fn test_unicode_no_permitido_en_ascii_imprimible() {
        let config = StringValidatorConfig::new()
            .min_longitud(2)
            .max_longitud(10)
            .min_caracteres_unicos(2)
            .ascii_imprimible();

        let entrada = "hola ñoño";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "texto", &config);
        println!("Salida de error: {:?}", resultado);
        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert!(error.message.contains("caracteres no permitidos"));
    }

    #[test]
    fn test_simbolos_permitidos_y_no_permitidos() {
        let config = StringValidatorConfig::new()
            .min_longitud(2)
            .max_longitud(20)
            .min_caracteres_unicos(3)
            .con_simbolos(['_', '-']);

        let entrada_valida = "user_name-123";
        println!("Entrada válida: {:?}", entrada_valida);
        let resultado = StringValidador::validar(entrada_valida, "usuario", &config);
        println!("Salida: {:?}", resultado);
        assert!(resultado.is_ok());

        let entrada_invalida = "user.name@123";
        println!("Entrada inválida: {:?}", entrada_invalida);
        let resultado = StringValidador::validar(entrada_invalida, "usuario", &config);
        println!("Salida de error: {:?}", resultado);
        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert!(error.message.contains("caracteres no permitidos"));
    }

    #[test]
    fn test_regla_default_permite_alfanumericos_y_guion_bajo() {
        let config = StringValidatorConfig::new()
            .min_longitud(3)
            .max_longitud(20)
            .min_caracteres_unicos(3);

        let entrada_valida = "usuario_2026";
        println!("Entrada válida: {:?}", entrada_valida);
        let resultado = StringValidador::validar(entrada_valida, "nombre", &config);
        println!("Salida: {:?}", resultado);
        assert!(resultado.is_ok());

        let entrada_invalida = "usuario@2026";
        println!("Entrada inválida: {:?}", entrada_invalida);
        let resultado = StringValidador::validar(entrada_invalida, "nombre", &config);
        println!("Salida de error: {:?}", resultado);
        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert!(error.message.contains("caracteres no permitidos"));
    }

    #[test]
    fn test_caracteres_unicos_justo_por_debajo_falla() {
        let config = StringValidatorConfig::new()
            .min_longitud(4)
            .max_longitud(10)
            .min_caracteres_unicos(4)
            .ascii_imprimible();

        let entrada = "abca";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "codigo", &config);
        println!("Salida de error: {:?}", resultado);
        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert!(error.message.contains("caracteres distintos"));
    }

    #[test]
    fn test_caracteres_unicos_exactos_pasa() {
        let config = StringValidatorConfig::new()
            .min_longitud(4)
            .max_longitud(10)
            .min_caracteres_unicos(4)
            .ascii_imprimible();

        let entrada = "abcd";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "codigo", &config);
        println!("Salida: {:?}", resultado);
        assert!(resultado.is_ok());
    }

    #[test]
    fn test_cadena_vacia_siempre_falla() {
        let config = StringValidatorConfig::new()
            .min_longitud(1)
            .max_longitud(10)
            .min_caracteres_unicos(1)
            .permitir_todo();

        let entrada = "";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "campo", &config);
        println!("Salida de error: {:?}", resultado);
        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert!(error.message.contains("no puede estar vacío"));
    }
}