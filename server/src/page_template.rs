use std::fs;

use actix_session::Session;
use actix_web::{http::header::LOCATION, web, HttpResponse, Resource};
use askama::Template;

use crate::data::session::get_session;

/// A struct used to compile a page
#[derive(Template)]
#[template(path = "_app.html", escape = "none")]
#[derive(Clone)]
pub struct PageTemplate<'a> {
    /// What to display in the <title> tag
    title: &'a str,
    /// The name of the component that is used as an entry point
    page_component_name: &'a str,
    /// The pieces of code to add to the page
    components: Vec<String>,
}

/// Generate the text to be sent as a response to a page request
/// `title` is the name of the page to be displayed using the <title> tag
/// `path` is the URL path under which this resource would be located
/// `elements` are the react elements to be included in the page
/// # Panics
/// This function panics if the react elements do not include exactly one page element or if the files for the elements could not be read.
fn create_template(title: &'static str, elements: &'static [ReactElement]) -> &'static mut String {
    // Find all the page elements
    let page_elements: Vec<_> = elements.iter().filter(|e| e.is_page()).collect();
    // Check that exactly one page element was given
    assert_eq!(page_elements.len(), 1, "Expected exactly 1 page element");
    // Only use the first one
    let page_component_name = page_elements.get(0).unwrap().name();

    let template = PageTemplate {
        title,
        page_component_name,
        // Read all the code for the elements
        components: elements.into_iter().map(|e| e.read_code()).collect(),
    };
    let template = template.render().expect("Could not render a template");

    // leak the template so that it can be referenced later.
    // This is OK to do because it is only created once and it does need to be referenced for the rest of the program
    Box::leak(Box::new(template))
}

/// Create the code for a page based on the components that need to be included
/// The page will only be accessible to users who are logged in
/// `title` is the name of the page to be displayed using the <title> tag
/// `path` is the URL path under which this resource would be located
/// `elements` are the react elements to be included in the page
/// # Panics
/// This function panics if the react elements do not include exactly one page element or if the files for the elements could not be read.
pub fn create_session_protected_page(
    title: &'static str,
    path: &str,
    elements: &'static [ReactElement],
) -> Resource {
    let template = create_template(title, elements);

    // create an actix resource handler
    web::resource(path).to(|session: Session| async {
        // Check if the session is valid
        let user = get_session(session);
        // If it is not, redirect to login
        if user.is_none() {
            return HttpResponse::TemporaryRedirect()
                .insert_header((LOCATION, "/login"))
                .finish();
        }
        // Otherwise, build a normal response
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(template.clone())
    })
}

/// Create the code for a page based on the components that need to be included
/// `title` is the name of the page to be displayed using the <title> tag
/// `path` is the URL path under which this resource would be located
/// `elements` are the react elements to be included in the page
/// # Panics
/// This function panics if the react elements do not include exactly one page element or if the files for the elements could not be read.
pub fn create_page(title: &'static str, path: &str, elements: &'static [ReactElement]) -> Resource {
    let template = create_template(title, elements);
    // create an actix resource handler
    web::resource(path).to(|| async {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(template.clone())
    })
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
    /// # Panics
    /// It panics if it could not read the file specified by the element instance
    fn read_code(&self) -> String {
        let path = match self {
            ReactElement::COMPONENT(name) => format!("public/jsx/components/{}.jsx", name),
            ReactElement::PAGE(name) => format!("public/jsx/pages/{}.jsx", name),
        };

        fs::read_to_string(&path).expect(&format!("Could not read jsx for {}", path))
    }
}
