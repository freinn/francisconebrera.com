use regex::Regex;
use itertools::multizip;

fn one_time_compiling_xss_prevention(text: &mut String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<\s*/?script.*?>").unwrap();
    }
    RE.replace_all(text, "").into_owned()
}

pub fn destroy_xss(cadena: &str) -> String {
    one_time_compiling_xss_prevention(&mut cadena.to_string())
}

#[test]
fn test_destroy_xss() {
    let test_inputs: Vec<&str> =
        vec![
            "Contenido wapo, sano y verídico y luego <  script>un script malicioso </script>y luego más cosas!",
        ];

    let expected_outputs: Vec<String> =
        vec![
            "Contenido wapo, sano y verídico y luego un script malicioso y luego más cosas!".to_string(),
        ];

    // podemos hacer un zip con tantas cosas como queramos, itertools::Zip lo aguanta
    for (test_input, expected_output) in multizip((&test_inputs, &expected_outputs)) {
        assert_eq!(
            destroy_xss(*test_input),
            *expected_output
        );
    }
}
