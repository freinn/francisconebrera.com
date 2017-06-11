use rocket_contrib::Template;
use rocket::request::Form;

use chrono::NaiveDate;
use file;

use tera_template_contexts::*;
use super::common_utils;
use security::*;

use std::str;
use std::fs; // ficheros en el SO
use regex::Regex;

// html escapes
use htmlescape::encode_minimal;
// fin html escapes

fn get_file_content_as_string(path: &str) -> String {
    let file_content = match file::get(path) {
        Ok(res) => res,
        Err(e) => panic!("{:?}", e),
    };

    let file_content_as_slice = str::from_utf8(file_content.as_slice());
    let content_string =
        str::from_utf8(file_content_as_slice.unwrap().as_bytes()).unwrap().to_string();

    content_string
}

fn obtener_entradas_blog() -> Vec<EntradaBlog> {
    let mut content_string_backtracking1 = get_file_content_as_string("../entradas/backtracking-1/backtracking-1.html");

    let mut content_string_ocean = get_file_content_as_string("../entradas/guia-the-ocean-hunter/guia-ocean-hunter.html");

    let mut content_string_monada_maybe = get_file_content_as_string("../entradas/monadas/entradaMonadaMaybe.html");

    let mut content_string_guia_meepo = get_file_content_as_string("../entradas/dota2/guia_meepo/guia_meepo.html");

    let mut content_string_consejos_para_programadores = get_file_content_as_string("../entradas/consejos-para-programadores/consejos-para-programadores.html");

    let mut content_string_rust = get_file_content_as_string("../entradas/rust/rust.html");

    vec![EntradaBlog {
             id_entrada_blog: 6,
             titulo: "El lenguaje de programación rust".to_string(),
             titulo_url: "el-lenguaje-de-programacion-rust".to_string(),
             fecha_publicacion: "11/6/2017".to_string(),
             fecha_ultima_edicion: "11/6/2017".to_string(),
             contenido: content_string_rust,
             tiempo_de_lectura: 5,
             publicada: true,
         },
         EntradaBlog {
             id_entrada_blog: 5,
             titulo: "Consejos para programadores".to_string(),
             titulo_url: "consejos-para-programadores".to_string(),
             fecha_publicacion: "25/5/2017".to_string(),
             fecha_ultima_edicion: "25/5/2017".to_string(),
             contenido: content_string_consejos_para_programadores,
             tiempo_de_lectura: 20,
             publicada: true,
         },
         EntradaBlog {
             id_entrada_blog: 4,
             titulo: "Meepo el geomante, guía de abuso".to_string(),
             titulo_url: "meepo-guia-de-abuso".to_string(),
             fecha_publicacion: "16/5/2017".to_string(),
             fecha_ultima_edicion: "16/5/2017".to_string(),
             contenido: content_string_guia_meepo,
             tiempo_de_lectura: 15,
             publicada: true,
         },
         EntradaBlog {
             id_entrada_blog: 3,
             titulo: "Guía The Ocean Hunter".to_string(),
             titulo_url: "guia-the-ocean-hunter".to_string(),
             fecha_publicacion: "16/4/2017".to_string(),
             fecha_ultima_edicion: "16/4/2017".to_string(),
             contenido: content_string_ocean,
             tiempo_de_lectura: 15,
             publicada: true,
         },
         EntradaBlog {
             id_entrada_blog: 2,
             titulo: "La mónada Maybe para principantes".to_string(),
             titulo_url: "monada-maybe-1".to_string(),
             fecha_publicacion: "10/7/2015".to_string(),
             fecha_ultima_edicion: "10/7/2015".to_string(),
             contenido: content_string_monada_maybe,
             tiempo_de_lectura: 15,
             publicada: true,
         },
         EntradaBlog {
             id_entrada_blog: 1,
             titulo: "Backtracking: guía para principantes".to_string(),
             titulo_url: "backtracking-1".to_string(),
             fecha_publicacion: "29/3/2015".to_string(),
             fecha_ultima_edicion: "29/3/2015".to_string(),
             contenido: content_string_backtracking1,
             tiempo_de_lectura: 15,
             publicada: true,
         }]
}

#[get("/<title_url>")]
fn vista_entrada_blog(title_url: &str) -> Template {
    let entradas_blog = obtener_entradas_blog();

    for entrada in entradas_blog {
        if entrada.titulo_url == title_url {
            return Template::render("blog/mostrar_entrada_blog", &entrada);
        }
    }

    Template::render("404", &obtener_entradas_blog()[0])
}

#[get("/")]
fn get_entradas_blog() -> Template {
    let results = obtener_entradas_blog();

    //println!("results = {:?}", results);

    let context = TemplateContextEntradaBlog {
        page_title: "Posts".to_string(),
        entradas_blog: results,
    };

    Template::render("blog/blog", &context)
}
