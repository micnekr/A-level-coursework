use std::{fs, path::Path};

use actix_web::{web, Resource};
use askama::Template;

use crate::settings::{ALLOWED_ORIGIN, PROTOCOL}; // bring trait in scope

#[derive(Template)]
#[template(path = "_app.html", escape = "none")]
#[derive(Clone)]
pub struct AppTemplate<'a> {
    title: &'a str,
    component_name: &'a str,
    component: String,
}

pub fn create_page(title: &'static str, path: &str, component_name: &'static str) -> Resource {
    let code_path = format!("public/jsx/{}.jsx", component_name.to_lowercase());
    let code_path = Path::new(code_path.as_str());
    let template = AppTemplate {
        title,
        component_name,
        component: fs::read_to_string(code_path).expect(&format!(
            "Could not read jsx for component {}",
            component_name
        )),
    };
    let template = Box::new(template);
    let template = Box::leak(template);

    web::resource(path).to(|| async { template.clone() })
}
