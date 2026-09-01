// ---------------------------------------------------------------------------
// Regla de Caracteres
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
pub enum ReglaCaracteres {
    SoloNumeros,
    /// Permite caracteres alfanuméricos, espacios en blanco y una lista de símbolos permitidos.
    AlfanumericoConEspaciosYSimbolos(Vec<char>),
    AsciiImprimible,
    /// Permite cualquier carácter, incluyendo emojis y otros símbolos Unicode.
    PermitirTodo,
}

impl ReglaCaracteres {
    pub fn con_simbolos(simbolos: impl IntoIterator<Item = char>) -> Self {
        Self::AlfanumericoConEspaciosYSimbolos(simbolos.into_iter().collect())
    }

    /// Evalúa si un carácter cumple con la regla configurada.
    pub fn es_permitido(&self, c: char) -> bool {
        match self {
            Self::SoloNumeros => c.is_ascii_digit(),
            Self::AlfanumericoConEspaciosYSimbolos(simbolos) => {
                c.is_alphanumeric() || c.is_whitespace() || simbolos.contains(&c)
            }
            Self::AsciiImprimible => c.is_ascii() && !c.is_ascii_control(),
            Self::PermitirTodo => true,
        }
    }
}

impl Default for ReglaCaracteres {
    fn default() -> Self {
        Self::con_simbolos(['-', '_', '.', ',', '@', '+'])
    }
}

