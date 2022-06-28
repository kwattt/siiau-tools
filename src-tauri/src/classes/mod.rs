use serde::Serialize; 

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct Ciclo {
    pub id: String,
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct Centro {
    pub id: String,
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct Sesion {
    pub entrada: i32,
    pub salida: i32,
    pub dias: Vec<String>,
    pub edificio: String,
    pub aula: String,
    pub periodo: String
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct Materia {
    pub activo: bool,
    pub nrc: String,
    pub clave: String,
    pub nombre: String,
    pub seccion: String,
    pub creditos: i32,
    pub cupos: i32,
    pub disponibles: i32,
    pub profesores: Vec<String>,
    pub horas: Vec<Sesion>
}

impl Materia {
    pub fn new() -> Self {
        Self {
            activo: false,
            nrc: String::new(),
            clave: String::new(),
            nombre: String::new(),
            seccion: String::new(),
            creditos: 0,
            cupos: 0,
            disponibles: 0,
            profesores: Vec::new(),
            horas: Vec::new()
        }
    }
}