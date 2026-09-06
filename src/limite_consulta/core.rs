// -------------------------------------------------------------------------
// VALOR DE CONFIGURACIÓN / TYPE-SAFETY PARA LÍMITES DE CONSULTA
// -------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimiteConsulta(u64);

impl LimiteConsulta {
    pub const MINIMO: u64 = 5;
    pub const MAXIMO: u64 = 100;
    pub const DEFAULT: Self = Self(10);

    pub fn new(val: u64) -> Self {
        Self(val.clamp(Self::MINIMO, Self::MAXIMO))
    }

    pub fn valor(&self) -> u64 {
        self.0
    }
}

impl From<u64> for LimiteConsulta {
    fn from(val: u64) -> Self {
        Self::new(val)
    }
}

// Query parameters para endpoints de listado. | Request-deserealize
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListarQuery {
    pub limite: Option<u64>,
}