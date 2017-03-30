use rocket_contrib::Template;
use tera_template_contexts::BasicTemplateContext;

#[get("/portfolio")]
fn get_portfolio() -> Template {
    let context = BasicTemplateContext {
        username: "freinn".to_string(),
    };

    Template::render("portfolio/portfolio", &context)
}

#[get("/")]
fn get_index() -> Template {
    let context = BasicTemplateContext {
        username: "Freinn".to_string(),
    };

    Template::render("index/index", &context)
}
