/*
Cosas que necesitaremos en varios sitios, y sus tests.
*/

use security::xss_prevention::*;

use regex::Regex;
use chrono;

use itertools::multizip;

// html escapes
/*
use escapade::Append;
use escapade::Escapable;
*/
// fin html escapes

pub fn from_spanish_title_to_url_compatible(titulo_en_espanol: &str) -> String {
    let titulo_lowercase = titulo_en_espanol.trim().to_string().to_lowercase();
    let titulo_lowercase_one_spaced = drop_multiple_spaces(&titulo_lowercase);
    let titulo_lowercase_one_spaced_sanitized = destroy_xss(&titulo_lowercase_one_spaced);
    let mut string_for_url = String::with_capacity(titulo_lowercase_one_spaced_sanitized.len());
    let not_allowed_in_url: &str = "!?¿¡;:,()\"\'$%\\\\/";

    for c in titulo_lowercase_one_spaced_sanitized.chars() {
        if not_allowed_in_url.find(c).is_some() {
            continue;
        }
        string_for_url.push(
            match c {
                ' ' => '-',
                'ñ' | 'Ñ' => 'n',
                'á' | 'Á' => 'a',
                'é' | 'É' => 'e',
                'í' | 'Í' => 'i',
                'ó' | 'Ó' => 'o',
                'ú' | 'Ú' => 'u',
                another_char => another_char,
            }
        );
    }

    // string_for_url = String::from(string_for_url.as_str()).escape().into_inner();
    string_for_url
}

// Cuidado con #[should_panic], invierte el resultado de los tests!

#[test]
fn it_converts_spanish_strings_into_urls() {
    let test_inputs: Vec<&str> =
        vec![
            "Mi nueva  web  ",
            "   El Último Cuadro",
            " Lo áureo    ",
            " Áureo como lo  más  áureo",
            "EspaÑa  va bien "
        ];

    let expected_outputs: Vec<String> =
        vec![
            "mi-nueva-web".to_string(),
            "el-ultimo-cuadro".to_string(),
            "lo-aureo".to_string(),
            "aureo-como-lo-mas-aureo".to_string(),
            "espana-va-bien".to_string(),
        ];

    // multizip accepts iterators and/or values with IntoIterator.
    // podemos hacer un zip con tantas cosas como queramos, itertools::Zip lo aguanta
    for (test_input, expected_output) in multizip((&test_inputs, &expected_outputs)) {
        assert_eq!(
            from_spanish_title_to_url_compatible(*test_input),
            *expected_output
        );
    }
}

pub fn fecha_y_hora_de_hoy() -> chrono::NaiveDateTime {
    chrono::offset::utc::UTC::now().naive_utc()
}

// Prevent hacking

fn one_time_compiling_drop_multiple_spaces(text: &mut String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s+").unwrap();
    }
    RE.replace_all(text, " ").into_owned()
}

pub fn drop_multiple_spaces(cadena: &str) -> String {
    one_time_compiling_drop_multiple_spaces(&mut cadena.to_string())
}
