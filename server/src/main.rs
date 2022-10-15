use crate::{
    data::users::UnsavedUser,
    db::establish_connection,
    page_template::create_page,
    settings::{ALLOWED_ORIGIN, PASSWORD_HASH_LENGTH},
};
use actix_cors::Cors;
use actix_web::{http::header, middleware, App, HttpServer};
use dotenvy::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use page_template::ReactElement;
use std::env;

pub mod data;
pub mod db;
pub mod page_template;
pub mod schema;
pub mod settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Startup
    // Find out the length of the hash in this representation
    let password_length = UnsavedUser::hash("")
        .expect("Failed to hash a test string")
        .len();
    assert_eq!(password_length, PASSWORD_HASH_LENGTH);

    // Load the .env file
    dotenv().expect("Failed to load the .env file");

    // Connect to the database using the URL in the .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = &mut establish_connection(database_url);

    // load TLS keys
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("Could not create an SSL builder");
    ssl_builder
        .set_private_key_file("ssl/key.pem", SslFiletype::PEM)
        .expect("Could not locate the key.pem file");
    ssl_builder
        .set_certificate_chain_file("ssl/cert.pem")
        .expect("Could not locate the cert.pem file");

    // create the pages from the templates
    // create the server
    let server = HttpServer::new(|| {
        App::new()
            // Activate logger middleware
            .wrap(middleware::Logger::default())
            // Set suitable CORS
            .wrap(
                Cors::default()
                    .allowed_origin(ALLOWED_ORIGIN)
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .supports_credentials(), // Allow the cookie auth.
            )
            // Serve the static css and js files
            .service(actix_files::Files::new("/css", "public/css").show_files_listing())
            .service(actix_files::Files::new("/js", "public/js").show_files_listing())
            // Serve pages by constructing them out of their components
            .service(create_page(
                "Log in",
                "/login",
                &[
                    ReactElement::PAGE("Login"),
                    ReactElement::COMPONENT("PageContainerBox"),
                ],
            ))
            .service(create_page(
                "Sign up",
                "/signup",
                &[
                    ReactElement::PAGE("Signup"),
                    ReactElement::COMPONENT("PageContainerBox"),
                    ReactElement::COMPONENT("PasswordStrength"),
                ],
            ))
    })
    // set up openssl for use
    .bind_openssl(ALLOWED_ORIGIN, ssl_builder)?;

    // Main operation
    {
        // use self::schema::users::dsl::*;
        // let results = users.load::<User>(connection).expect("Error loading users");

        // let new_user =
        //     UnsavedUser::try_new(String::from("test user"), String::from("Test")).unwrap();
        // println!("{}", new_user.password_hash.len());

        // println!("Num results: {}", results.len());
        // for (i, result) in results.iter().enumerate() {
        //     println!("Result number {}: {:?}", i, result);
        // }
    }

    server.run().await
}
