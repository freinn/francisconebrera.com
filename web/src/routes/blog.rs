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
    let mut content_string = get_file_content_as_string("../entradas/backtracking-1/backtracking-1.html");

    println!("content_string = {:?}", content_string);

    let mut current_path = "".to_string();
    let paths = fs::read_dir("../entradas/backtracking-1/code").unwrap();

    for path in paths {
        let filename = path.unwrap()
                     .file_name()
                     .to_str()
                     .unwrap()
                     .to_string();
        let re = Regex::new(&filename).unwrap();
        let realpath: String = ["../entradas/backtracking-1/code/", &filename].concat();
        let encoded: String = encode_minimal(&get_file_content_as_string(&realpath));
        println!("encoded = {}", &encoded);

        let content_string2 = content_string;
        content_string = content_string2.replace(&filename, &encoded);
    }

    // print!("{:?}", filenames);

    // let mut fixed_content_string = content_string;
    let mut filenames: Vec<String> = Vec::new();

    for filename in filenames {
        // let mut joined = Vec::with_capacity(40);
        // content_string: String
        let re = Regex::new(&filename).unwrap();
        let realpath: String = ["../entradas/backtracking-1/code/", &filename].concat();
        let encoded: String = encode_minimal(&get_file_content_as_string(&realpath));
        println!("encoded = {}", &encoded);

        let content_string2 = content_string;
        content_string = content_string2.replace(&filename, &encoded);
        // fixed_content_string = fixed_content_string2;

    }

    let mut content_string_ocean = get_file_content_as_string("../entradas/guia-the-ocean-hunter/guia-ocean-hunter.html");

    let mut content_string_monada_maybe = get_file_content_as_string("../entradas/monadas/entradaMonadaMaybe.html");

    vec![EntradaBlog {
             id_entrada_blog: 1,
             titulo: "Backtracking: guía para principantes".to_string(),
             titulo_url: "backtracking-1".to_string(),
             fecha_publicacion: NaiveDate::from_ymd(2015, 8, 14),
             fecha_ultima_edicion: NaiveDate::from_ymd(2017, 3, 29),
             contenido: content_string,
             tiempo_de_lectura: 15,
             publicada: true,
         },
         EntradaBlog {
             id_entrada_blog: 2,
             titulo: "La mónada Maybe para principantes".to_string(),
             titulo_url: "monada-maybe-1".to_string(),
             fecha_publicacion: NaiveDate::from_ymd(2015, 7, 10),
             fecha_ultima_edicion: NaiveDate::from_ymd(2017, 5, 3),
             contenido: content_string_monada_maybe,
             tiempo_de_lectura: 15,
             publicada: true,
         },
         EntradaBlog {
             id_entrada_blog: 3,
             titulo: "Guía The Ocean Hunter".to_string(),
             titulo_url: "guia-the-ocean-hunter".to_string(),
             fecha_publicacion: NaiveDate::from_ymd(2017, 4, 30),
             fecha_ultima_edicion: NaiveDate::from_ymd(2017, 4, 29),
             contenido: content_string_ocean,
             tiempo_de_lectura: 15,
             publicada: true,
         },]
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
