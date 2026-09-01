use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;
use unicode_normalization::UnicodeNormalization;

// ---------------------------------------------------------------------------
// Patrones estáticos (compilados una única vez al primer uso)
// ---------------------------------------------------------------------------

/// Caracteres de control y de formato que deben eliminarse del texto:
static PATRON_CARACTERES_CONTROL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"[\x00-\x08\x0B-\x0C\x0E-\x1F\x7F-\x9F\u202a-\u202e\u2066-\u2069]|\p{Cc}|\p{Cf}",
    )
    .expect("PATRON_CARACTERES_CONTROL: regex inválida")
});

/// Separadores de espacio y línea no estándar que se normalizan a un espacio:
static PATRON_SEPARADORES: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"[\n\r\t\x0C\x0B\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000]+",
    )
    .expect("PATRON_SEPARADORES: regex inválida")
});

/// Caracteres de ancho cero y pegables:
static PATRON_PEGABLES: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\u00AD\u034F\u061C\u180E\u200B\u200C\u200D\u2060\uFEFF]+")
        .expect("PATRON_PEGABLES: regex inválida")
});

/// Uno o más espacios en blanco consecutivos (`\s+`).
static PATRON_MULTIPLES_ESPACIOS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\s+").expect("PATRON_MULTIPLES_ESPACIOS: regex inválida")
});

// ---------------------------------------------------------------------------
// Struct principal
// ---------------------------------------------------------------------------

/// Limpiador de texto Unicode.
pub struct SpaceCleaner;

impl SpaceCleaner {

    /// Limpia y normaliza un texto.
    /// - `texto`: cadena de entrada
    /// - `longitud_maxima`: límite para evitar procesar textos demasiado largos
    /// - `limpiar_todo`: si es true elimina todos los espacios
    pub fn limpiar<T: AsRef<str>>(
        texto: T,
        longitud_maxima: usize,
        limpiar_todo: bool,
    ) -> String {

        // Obtiene una referencia &str del valor recibido.
        let t: &str = texto.as_ref();

        // Si está vacío no hay nada que limpiar.
        if t.is_empty() {
            return String::new();
        }

        // Si excede el límite, se devuelve sin procesar.
        if t.len() > longitud_maxima {
            return t.to_string();
        }

        // --- Normalización Unicode (NFKC) ---
        let mut buffer: String = t.nfkc().collect();

        // --- Sustituciones con regex ---
        Self::aplicar_regex_inplace(&mut buffer, &PATRON_CARACTERES_CONTROL, " ");
        Self::aplicar_regex_inplace(&mut buffer, &PATRON_PEGABLES, " ");
        Self::aplicar_regex_inplace(&mut buffer, &PATRON_SEPARADORES, " ");

        // Manejo de espacios múltiples según el modo.
        if limpiar_todo {
            Self::aplicar_regex_inplace(&mut buffer, &PATRON_MULTIPLES_ESPACIOS, "");
        } else {
            Self::aplicar_regex_inplace(&mut buffer, &PATRON_MULTIPLES_ESPACIOS, " ");
        }

        // --- Resultado final ---
        if limpiar_todo {
            // No quedan espacios en este modo.
            buffer
        } else {
            // Elimina espacios al inicio y final.
            buffer.trim().to_string()
        }
    }

    /// Aplica un regex sobre el buffer sin copiar si no hay cambios.
    fn aplicar_regex_inplace(buffer: &mut String, re: &Regex, reemplazo: &str) {
        let resultado = re.replace_all(buffer, reemplazo);

        // Solo reemplaza si el regex produjo un nuevo String.
        if let Cow::Owned(nuevo_texto) = resultado {
            *buffer = nuevo_texto;
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

// dependencias:
// [dev-dependencies]
// serde = { version = "1", features = ["derive"] }
// serde_json = "1"

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::path::Path;

    #[derive(Debug, Deserialize)]
    struct CasoPrueba {
        id: String,
        texto: String,
        longitud_maxima: usize,
        limpiar_todo: bool,
        esperado: String,
    }

    #[derive(Debug, Serialize)]
    struct ResultadoPrueba {
        id: String,
        entrada: String,
        salida: String,
        esperado: String,
        correcto: bool,
        longitud_maxima: usize,
        limpiar_todo: bool,
    }

    #[test]
    fn ejecutar_pruebas_desde_json() {
        let archivo_entrada = Path::new("tests/entrada/test2.json");
        let archivo_salida = Path::new("tests/salida/test2_salida.json");

        // Aseguramos que el directorio de salida exista
        if let Some(parent) = archivo_salida.parent() {
            fs::create_dir_all(parent).expect("No se pudo crear el directorio de salida");
        }

        let contenido = fs::read_to_string(archivo_entrada)
            .expect("No se pudo leer el archivo JSON de pruebas");

        let casos: Vec<CasoPrueba> = serde_json::from_str(&contenido)
            .expect("El archivo JSON de pruebas no es válido");

        let mut resultados = Vec::with_capacity(casos.len());
        let mut fallos = Vec::new();

        // 1. Procesar todos los casos sin detener la ejecución
        for caso in casos {
            let salida = SpaceCleaner::limpiar(
                &caso.texto,
                caso.longitud_maxima,
                caso.limpiar_todo,
            );

            let correcto = salida == caso.esperado;

            if !correcto {
                fallos.push(format!(
                    "Caso '{}' falló.\n  Esperado: {:?}\n  Obtenido:  {:?}",
                    caso.id, caso.esperado, salida
                ));
            }

            resultados.push(ResultadoPrueba {
                id: caso.id,
                entrada: caso.texto,
                salida,
                esperado: caso.esperado,
                correcto,
                longitud_maxima: caso.longitud_maxima,
                limpiar_todo: caso.limpiar_todo,
            });
        }

        // 2. Escribir el informe JSON completo siempre (incluso si hubo fallos)
        let json = serde_json::to_string_pretty(&resultados)
            .expect("No se pudieron serializar los resultados");

        fs::write(archivo_salida, json)
            .expect("No se pudo escribir el JSON de resultados");

        println!(
            "Resultados procesados ({}/{} correctos). Escritos en {}",
            resultados.len() - fallos.len(),
            resultados.len(),
            archivo_salida.display()
        );

        // 3. Hacer fallar el test al final si hubo algún error acumulado
        assert!(
            fallos.is_empty(),
            "\nSe encontraron {} fallos durante la ejecución:\n\n{}\n",
            fallos.len(),
            fallos.join("\n\n")
        );
    }
}