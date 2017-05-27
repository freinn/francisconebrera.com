use std;

// Para pasar parámetros a Tera

#[derive(Serialize, Deserialize)]
pub struct BasicTemplateContext {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateContextEntradaBlog {
    pub page_title: String,
    pub entradas_blog: Vec<EntradaBlog>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntradaBlog {
    pub id_entrada_blog: i32,
    pub titulo: String,
    pub titulo_url: String,
    pub fecha_publicacion: String,
    pub fecha_ultima_edicion: String,
    pub contenido: String,
    pub tiempo_de_lectura: i16, // SmallInt
    pub publicada: bool,
}
