use lazy_static::lazy_static;
use tera::{Context, Tera};
use actix_web::{error::ErrorInternalServerError, Error};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("assets/templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

pub fn render_temp_controller (t: &Tera, tn: &str, c: &Context) -> Result<String, Error> {
    let r = match t.render(tn, c) {
        Ok(x) => x,
        Err(err) => {
            log::error!("{err:?}");
            return Err(ErrorInternalServerError(err));
        },
    };
    return Ok(r);
}
