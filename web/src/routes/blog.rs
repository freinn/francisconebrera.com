use rocket_contrib::Template;
use rocket::request::Form;

use chrono::NaiveDate;
use file;

use tera_template_contexts::*;
use super::common_utils;
use security::*;

use std::str;

// html escapes
use htmlescape::{encode_minimal};
// fin html escapes

fn obtener_entradas_blog() -> Vec<EntradaBlog> {
    let file_content = 
        match file::get("../entradas/backtracking-1/backtracking-1.html") {
            Ok(res) => res,
            Err(e) => panic!("{:?}", e),
        };

    let file_content_as_slice = str::from_utf8(file_content.as_slice());
    let escaped_content = encode_minimal(str::from_utf8(file_content_as_slice.unwrap().as_bytes()).unwrap());

    vec![EntradaBlog {
             id_entrada_blog: 1,
             titulo: "Backtracking: guía para principantes".to_string(),
             titulo_url: "backtracking-1".to_string(),
             fecha_publicacion: NaiveDate::from_ymd(2015, 8, 14),
             fecha_ultima_edicion: NaiveDate::from_ymd(2017, 3, 29),
             contenido: escaped_content,
             tiempo_de_lectura: 15,
             publicada: true,
         }]
}


#[get("/<title_url>")]
fn vista_entrada_blog(title_url: &str) -> Template {
    let entradas_blog = obtener_entradas_blog();

    Template::render("blog/mostrar_entrada_blog", &entradas_blog[0])
}

#[get("/")]
fn get_entradas_blog() -> Template {
    let results = obtener_entradas_blog();

    //println!("results = {:?}", results);

    let context = TemplateContextEntradaBlog {
        page_title: "Posts".to_string(),
        entradas_blog: results,
    };

    // let serialized = serde_json::to_string(&context).unwrap();

    Template::render("blog/blog", &context)
}
