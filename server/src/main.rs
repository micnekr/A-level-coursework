use crate::{
    data::users::UnsavedUser,
    db::establish_connection,
    page_template::create_page,
    settings::{ALLOWED_ORIGIN, PASSWORD_HASH_LENGTH},
};
use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{http::header, middleware, App, HttpServer, Responder};
use diesel::PgConnection;
use dotenvy::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use page_template::ReactElement;
use std::env;
use std::sync::Mutex;

pub mod data;
pub mod db;
pub mod endpoints;
pub mod page_template;
pub mod schema;
pub mod settings;

pub struct ServerState {
    pub connection: Mutex<PgConnection>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Startup
    // Create a logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    // Find out the length of the hash in this representation
    let password_length = UnsavedUser::hash("")
        .expect("Failed to hash a test string")
        .len();
    assert_eq!(password_length, PASSWORD_HASH_LENGTH);

    // Load the .env file
    dotenv().expect("Failed to load the .env file");

    // load TLS keys
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("Could not create an SSL builder");
    ssl_builder
        .set_private_key_file("ssl/key.pem", SslFiletype::PEM)
        .expect("Could not locate the key.pem file");
    ssl_builder
        .set_certificate_chain_file("ssl/cert.pem")
        .expect("Could not locate the cert.pem file");

    // create the server
    let server = HttpServer::new(|| {
        // Connect to the database using the URL in the .env file
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = establish_connection(database_url);

        // Set up sessions
        let session_secret_key = actix_web::cookie::Key::generate();

        // Create server state
        let server_data = actix_web::web::Data::new(ServerState {
            connection: Mutex::new(connection),
        });

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
            // Set up sessions
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                session_secret_key.clone(),
            ))
            // Server state
            .app_data(server_data)
            // endpoints
            .service(endpoints::users::signup)
            .service(endpoints::users::login)
            // Serving files
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

    server.run().await
}
