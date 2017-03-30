use rocket_contrib::Template;
use rocket::request::Form;

use chrono::NaiveDate;
use file;

use tera_template_contexts::*;
use super::common_utils;
use security::*;

fn obtener_entradas_blog() -> Vec<EntradaBlog> {
    let file_content = 
        match file::get_text("../entradas/backtracking-1/backtracking-1.html") {
            Ok(res) => res,
            Err(e) => panic!("{:?}", e),
        };

    vec![EntradaBlog {
             id_entrada_blog: 1,
             titulo: "Backtracking: guía para principantes".to_string(),
             titulo_url: "backtracking-1".to_string(),
             fecha_publicacion: NaiveDate::from_ymd(2015, 8, 14),
             fecha_ultima_edicion: NaiveDate::from_ymd(2017, 3, 29),
             contenido: file_content,
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
