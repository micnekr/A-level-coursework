use crate::{
    data::{models::UnsavedModel, session::set_session, users::User},
    endpoints::EndpointError,
    ServerState,
};
use actix_session::Session;
use actix_web::{post, web::Json};
use diesel::result::Error;
use serde::Deserialize;

use crate::data::users::UnsavedUser;

/// A struct for signup requests
#[derive(Deserialize)]
pub struct SignupRequest {
    username: String,
    password: String,
}

/// A struct for login requests
#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

/// An API endpoint used to register a user
#[post("/api/signup")]
pub async fn signup(
    req_body: Json<SignupRequest>,
    session: Session,
    data: actix_web::web::Data<ServerState>,
) -> Result<&'static str, EndpointError> {
    // Create a user with those details
    let unsaved_user = UnsavedUser::try_new(req_body.username.clone(), req_body.password.clone())
        .expect("Failed to create a user");

    // Get the connection from the mutex
    let mut connection = data
        .connection
        .lock()
        .expect("Could not get the connection from ServerState");
    // Try to save the user
    let user = unsaved_user.save(&mut connection);

    // See if it worked
    match user {
        Result::Err(err) => {
            // Check if the error is due a username uniqueness constraint violation
            if let Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) = err
            {
                // Tell the client to use a unique username
                Result::Err(EndpointError::BadClientData(
                    "This username is taken. Try using a different username.",
                ))
            } else {
                // Generic error

                // Log the error
                log::error!("users.signup.save: {}", err);
                Result::Err(EndpointError::InternalError)
            }
        }
        Result::Ok(user) => {
            set_session(session, &user).expect("Could not serialise user");
            Result::Ok("Success!")
        }
    }
}

/// An API endpoint used to log in a user
#[post("/api/login")]
pub async fn login(
    req_body: Json<LoginRequest>,
    session: Session,
    data: actix_web::web::Data<ServerState>,
) -> Result<&'static str, EndpointError> {
    // Get the connection from the mutex
    let mut connection = data
        .connection
        .lock()
        .expect("Could not get the connection from ServerState");

    // Try to get the user from the database with this username and password
    let user = User::fetch_check(
        &mut connection,
        req_body.username.clone(),
        req_body.password.clone(),
    );

    // See if it worked
    match user {
        Option::None => {
            // Tell the client that this user does not exist
            Result::Err(EndpointError::BadClientData(
                "Incorrect username or password. Please double-check the username and password.",
            ))
        }
        Option::Some(user) => {
            set_session(session, &user).expect("Could not serialise user");
            Result::Ok("Success!")
        }
    }
}
