use std::{fs, path::Path};

use actix_web::{web, Resource};
use askama::Template;

#[derive(Template)]
#[template(path = "_app.html", escape = "none")]
#[derive(Clone)]
pub struct AppTemplate<'a> {
    title: &'a str,
    page_component_name: &'a str,
    components: Vec<String>,
}

/// Create the code for a page based on the components that need to be included
pub fn create_page(
    title: &'static str,
    path: &str,
    components: &'static [ReactElement],
) -> Resource {
    let template = AppTemplate {
        title,
        page_component_name: components
            .iter()
            .find(|e| e.is_page())
            .expect("No page element found")
            .name(),
        components: components.into_iter().map(|e| e.read_code()).collect(),
    };
    let template = Box::new(template);
    let template = Box::leak(template);

    web::resource(path).to(|| async { template.clone() })
}

/// A React element (component or a page) that needs to be imported to a page
pub enum ReactElement<'a> {
    COMPONENT(&'a str),
    PAGE(&'a str),
}

impl<'a> ReactElement<'a> {
    /// Returns the name of the element
    fn name(&self) -> &str {
        match self {
            ReactElement::COMPONENT(e) => e,
            ReactElement::PAGE(e) => e,
        }
    }

    /// Determines whether a component is a page or not
    fn is_page(&self) -> bool {
        match self {
            ReactElement::COMPONENT(_) => false,
            ReactElement::PAGE(_) => true,
        }
    }

    /// loads the code associated with this element
    fn read_code(&self) -> String {
        let path = match self {
            ReactElement::COMPONENT(name) => format!("public/jsx/components/{}.jsx", name),
            ReactElement::PAGE(name) => format!("public/jsx/pages/{}.jsx", name),
        };

        fs::read_to_string(&path).expect(&format!("Could not read jsx for {}", path))
    }
}
