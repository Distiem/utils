
#[cfg(test)]
mod tests_eliminar_espacios {
    use crate::string_validador::StringValidatorConfig;
    use crate::string_validador::StringValidador;

    #[test]
    fn test_eliminar_todo_espacio_exitoso() {
        // Caso 1: String con espacios internos y externos, pero se eliminan todos.
        let config = StringValidatorConfig::new()
            .min_longitud(3)
            .max_longitud(10)
            .min_caracteres_unicos(2)
            .ascii_imprimible()
            .eliminar_todo_espacio();

        let entrada = "  a b c  ";
        println!("Entrada original: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "texto", &config);
        println!("Salida: {:?}", resultado);

        assert!(resultado.is_ok());
        // Se espera que todos los espacios sean eliminados: "abc"
        assert_eq!(resultado.unwrap(), "abc");
    }

    #[test]
    fn test_eliminar_todo_espacio_con_solo_espacios_falla() {
        // Caso 2: Un string que solo contiene espacios se convierte en vacío y debe fallar.
        let config = StringValidatorConfig::new()
            .min_longitud(1)
            .max_longitud(10)
            .min_caracteres_unicos(1)
            .ascii_imprimible()
            .eliminar_todo_espacio();

        let entrada = "     ";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "texto", &config);
        println!("Salida de error: {:?}", resultado);

        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert_eq!(error.campo, "texto");
        assert!(error.message.contains("no puede estar vacío"));
    }

    #[test]
    fn test_eliminar_todo_espacio_permite_caracteres_especiales() {
        // Caso 3: Sin eliminar espacios, el espacio sería un carácter no permitido,
        // pero al activar la opción, los espacios se eliminan y la validación pasa.
        let config = StringValidatorConfig::new()
            .min_longitud(4)
            .max_longitud(20)
            .min_caracteres_unicos(3)
            .solo_numeros() // solo números, el espacio no está permitido
            .eliminar_todo_espacio();

        let entrada = "12 34 56";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "codigo", &config);
        println!("Salida: {:?}", resultado);

        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap(), "123456"); // espacios eliminados
    }

    #[test]
    fn test_sin_eliminar_todo_espacio_no_elimina_internos() {
        // Caso 4: Comprobamos que sin la opción, los espacios internos no se eliminan.
        // En este caso, los espacios no están permitidos, por lo que la validación falla.
        let config = StringValidatorConfig::new()
            .min_longitud(1)
            .max_longitud(10)
            .min_caracteres_unicos(1)
            .solo_numeros(); // no se llama a eliminar_todo_espacio

        let entrada = "12 34";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "codigo", &config);
        println!("Salida de error: {:?}", resultado);

        assert!(resultado.is_err());
        let error = resultado.unwrap_err();
        assert_eq!(error.campo, "codigo");
        assert!(error.message.contains("caracteres no permitidos"));
    }

    #[test]
    fn test_eliminar_todo_espacio_combinado_con_longitud() {
        // Caso 5: Al eliminar todos los espacios, la longitud cambia y debe cumplir los límites.
        let config = StringValidatorConfig::new()
            .min_longitud(5)
            .max_longitud(13)
            .min_caracteres_unicos(3)
            .ascii_imprimible()
            .eliminar_todo_espacio();

        let entrada = " a b c d e f ";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "texto", &config);
        println!("Salida: {:?}", resultado);

        // "abcdef" tiene longitud 6 y 6 caracteres únicos, cumple.
        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap(), "abcdef");
    }
}
