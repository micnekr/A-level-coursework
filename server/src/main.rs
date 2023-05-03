use crate::{
    data::users::UnsavedUser,
    db::establish_connection,
    page_template::{create_page, create_session_protected_page},
    settings::{DOMAIN, PASSWORD_HASH_LENGTH, PORT},
};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware, App, HttpServer};
use diesel::PgConnection;
use dotenvy::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use page_template::ReactElement;
use std::{env, fs::File};
use std::{io::Read, sync::Mutex};

pub mod data;
pub mod db;
pub mod endpoints;
pub mod page_template;
pub mod schema;
pub mod settings;

/// A struct that contains the data about the state of the servserside code; it is passed onto the endpoints
/// For example, most endpoints need access the database, and so need the connection to be stored here
pub struct ServerState {
    pub connection: Mutex<PgConnection>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Startup
    // Create a logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

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
    let allowed_origin = format!("{DOMAIN}:{PORT}");
    let server = HttpServer::new(|| {
        // Connect to the database using the URL in the .env file
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = establish_connection(database_url);

        // Set up sessions

        // Read the key file
        let mut f = File::open("session_key.txt").expect("Could not open the session key file");
        let mut raw_key = vec![];
        f.read_to_end(&mut raw_key)
            .expect("Could not read the session key");

        let session_secret_key = Key::from(raw_key.as_slice());

        // Create server state
        let server_data = actix_web::web::Data::new(ServerState {
            connection: Mutex::new(connection),
        });

        App::new()
            // Activate logger middleware
            .wrap(middleware::Logger::default())
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
            .service(endpoints::events::get_events)
            .service(endpoints::events::create_event)
            // Serving files
            // Serve the static css and js files
            .service(actix_files::Files::new("/css", "public/css").show_files_listing())
            .service(actix_files::Files::new("/js", "public/js").show_files_listing())
            // Serve pages by constructing them out of their components
            // Create a login page
            .service(create_page(
                "Log in",
                "/login",
                &[
                    ReactElement::PAGE("Login"),
                    ReactElement::COMPONENT("PageContainerBox"),
                    ReactElement::COMPONENT("ErrorMessage"),
                ],
            ))
            // Create a signup page
            .service(create_page(
                "Sign up",
                "/signup",
                &[
                    ReactElement::PAGE("Signup"),
                    ReactElement::COMPONENT("PageContainerBox"),
                    ReactElement::COMPONENT("PasswordStrength"),
                    ReactElement::COMPONENT("ErrorMessage"),
                ],
            ))
            .service(create_session_protected_page(
                "Calendar",
                "/",
                &[
                    ReactElement::PAGE("Calendar"),
                    ReactElement::COMPONENT("Timetable"),
                    ReactElement::COMPONENT("TimetableEvent"),
                    ReactElement::COMPONENT("ErrorMessage"),
                    ReactElement::COMPONENT("PageContainerBoxLarge"),
                ],
            ))
            .service(create_session_protected_page(
                "Create an Event",
                "/create_event",
                &[
                    ReactElement::PAGE("CreateEvent"),
                    ReactElement::COMPONENT("ErrorMessage"),
                    ReactElement::COMPONENT("PageContainerBox"),
                ],
            ))
    })
    // set up openssl for use
    .bind_openssl(allowed_origin.clone(), ssl_builder)?;
    println!("Listening on {allowed_origin}");

    server.run().await
}
