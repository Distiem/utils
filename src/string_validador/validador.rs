use std::collections::HashSet;

use crate::string_validador::enums_reglas::ReglaCaracteres;
use crate::enums::Severity;
use crate::space_cleaner::SpaceCleaner;

// ---------------------------------------------------------------------------
// Configuración de Validación
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct StringValidatorConfig {
    pub min_longitud: usize,
    pub max_longitud: usize,
    pub min_caracteres_unicos: usize,
    pub caracter_permitido: ReglaCaracteres,
    pub eliminar_todo_espacio: bool,
}

impl Default for StringValidatorConfig {
    fn default() -> Self {
        Self {
            min_longitud: 2,
            max_longitud: 255,
            min_caracteres_unicos: 2,
            caracter_permitido: ReglaCaracteres::default(),
            eliminar_todo_espacio: false,
        }
    }
}

impl StringValidatorConfig {
    /// Crea una nueva configuración con los valores por defecto.
    pub fn new() -> Self {
        Self::default()
    }

    /// Establece la longitud mínima permitida.
    pub fn min_longitud(mut self, min: usize) -> Self {
        self.min_longitud = min;
        self
    }

    /// Establece la longitud máxima permitida.
    pub fn max_longitud(mut self, max: usize) -> Self {
        self.max_longitud = max;
        self
    }

    /// Define el número mínimo de caracteres únicos requeridos.
    pub fn min_caracteres_unicos(mut self, min: usize) -> Self {
        self.min_caracteres_unicos = min;
        self
    }

    /// Asigna una regla de caracteres permitidos personalizada.
    pub fn caracter_permitido(mut self, regla: ReglaCaracteres) -> Self {
        self.caracter_permitido = regla;
        self
    }

    /// Configura la regla para aceptar únicamente números.
    pub fn solo_numeros(self) -> Self {
        self.caracter_permitido(ReglaCaracteres::SoloNumeros)
    }

    pub fn solo_letras_y_espacios(self) -> Self {
        self.caracter_permitido(ReglaCaracteres::SoloLetrasYEspacios)
    }

    /// Configura la regla para aceptar caracteres alfanuméricos y los símbolos indicados.
    pub fn con_simbolos(self, simbolos: impl IntoIterator<Item = char>) -> Self {
        self.caracter_permitido(ReglaCaracteres::con_simbolos(simbolos))
    }

    /// Configura la regla para aceptar solo caracteres ASCII imprimibles.
    pub fn ascii_imprimible(self) -> Self {
        self.caracter_permitido(ReglaCaracteres::AsciiImprimible)
    }

    /// Configura la regla para permitir cualquier carácter.
    pub fn permitir_todo(self) -> Self {
        self.caracter_permitido(ReglaCaracteres::PermitirTodo)
    }

    /// Activa la eliminación total de espacios (equivale a pasar `true` a `SpaceCleaner::limpiar`).
    pub fn eliminar_todo_espacio(mut self) -> Self {
        self.eliminar_todo_espacio = true;
        self
    }

    /// Permite establecer el valor directamente (opcional). Útil cuando se desea
    /// activar o desactivar la eliminación de espacios mediante una variable.
    pub fn set_eliminar_todo_espacio(mut self, value: bool) -> Self {
        self.eliminar_todo_espacio = value;
        self
    }
}

// ---------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct StringValidationError {
    pub message: String,
    pub campo: &'static str,
    pub severity: Severity,
}

impl std::fmt::Display for StringValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}][{}]: {}",
            self.severity, self.campo, self.message
        )
    }
}

impl std::error::Error for StringValidationError {}

// ---------------------------------------------------------------------------
// Validador Genérico
// ---------------------------------------------------------------------------

pub struct StringValidador;

impl StringValidador {
    /// Valida un String genérico basándose en la configuración enviada.
    pub fn validar(
        valor: impl AsRef<str>,
        campo: &'static str,
        config: &StringValidatorConfig,
    ) -> Result<String, StringValidationError> {
        let valor_raw = valor.as_ref();

        // 1. Limpieza inicial
        let valor = SpaceCleaner::limpiar(valor_raw, config.max_longitud, config.eliminar_todo_espacio);

        // 2. Verificación de contenido no vacío
        if valor.is_empty() {
            return Err(Self::error(
                campo,
                "El valor no puede estar vacío o contener únicamente espacios.",
            ));
        }

        // 3. Verificación de rangos de longitud
        let longitud = valor.len();

        if longitud < config.min_longitud {
            return Err(Self::error(
                campo,
                &format!(
                    "El valor es demasiado corto (mínimo {} caracteres).",
                    config.min_longitud
                ),
            ));
        }

        if longitud > config.max_longitud {
            return Err(Self::error(
                campo,
                &format!(
                    "El valor excede el límite permitido de {} caracteres.",
                    config.max_longitud
                ),
            ));
        }

        // 4. Caracteres permitidos según la regla configurada
        for caracter in valor.chars() {
            if !config.caracter_permitido.es_permitido(caracter) {
                return Err(Self::error(
                    campo,
                    "El valor contiene caracteres no permitidos.",
                ));
            }
        }

        // 5. Entropía / diversidad de caracteres
        let caracteres_unicos: HashSet<char> = valor.chars().collect();

        if caracteres_unicos.len() < config.min_caracteres_unicos {
            return Err(Self::error(
                campo,
                &format!(
                    "El valor debe ser más descriptivo (al menos {} caracteres distintos).",
                    config.min_caracteres_unicos
                ),
            ));
        }

        // 6. Resultado positivo
        Ok(valor)
    }

    fn error(campo: &'static str, message: &str) -> StringValidationError {
        StringValidationError {
            message: message.to_string(),
            campo,
            severity: Severity::Error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_eliminar_todo_espacio_dinamico() {
        // Variable que decide si eliminar todos los espacios.
        let debe_eliminar = true;

        // Configuración usando el método set_eliminar_todo_espacio.
        let config = StringValidatorConfig::new()
            .min_longitud(3)
            .max_longitud(10)
            .min_caracteres_unicos(2)
            .ascii_imprimible()
            .set_eliminar_todo_espacio(debe_eliminar);

        let entrada = " a b c ";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "texto", &config);
        println!("Salida: {:?}", resultado);

        // Esperamos que se eliminen todos los espacios: "abc"
        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap(), "abc");
    }

    #[test]
    fn test_set_eliminar_todo_espacio_desactivado() {
        // Aquí se desactiva la eliminación total; solo se recortan extremos.
        let debe_eliminar = false;

        let config = StringValidatorConfig::new()
            .min_longitud(3)
            .max_longitud(10)
            .min_caracteres_unicos(2)
            .permitir_todo()
            .set_eliminar_todo_espacio(debe_eliminar);

        let entrada = "  a b c  ";
        println!("Entrada: {:?}", entrada);
        let resultado = StringValidador::validar(entrada, "texto", &config);
        println!("Salida: {:?}", resultado);

        // Los espacios extremos se eliminan, pero los internos se mantienen.
        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap(), "a b c");
    }
}