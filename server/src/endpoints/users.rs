use crate::{
    data::{
        group::{ParticipationType, UnsavedGroup, UnsavedGroupParticipant},
        models::UnsavedModel,
        session::{get_session, set_session},
        users::User,
    },
    endpoints::EndpointError,
    ServerState,
};
use actix_session::Session;
use actix_web::{get, post, web::Json, Responder};
use diesel::{result::Error, Connection};
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
    server_state: actix_web::web::Data<ServerState>,
) -> Result<&'static str, EndpointError> {
    // Create a user with those details
    let unsaved_user = UnsavedUser::try_new(req_body.username.clone(), req_body.password.clone())
        .expect("Failed to create a user");

    // Get the connection from the mutex
    let mut connection = server_state
        .connection
        .lock()
        .expect("Could not get the connection from ServerState");

    // Try to save the user
    let user = connection.transaction::<User, _, _>(|connection| {
        // save the actual user
        let user = unsaved_user.save(connection)?;
        // Create a group just for the user
        let unsaved_group = UnsavedGroup {
            name: String::from("Myself"),
            owner_id: user.id,
            is_special: true,
        };

        // Save the group and the participant to the database
        let group = unsaved_group.save(connection)?;
        let participation = UnsavedGroupParticipant {
            participation_type: ParticipationType::Accepted,
            group_id: group.id,
            participant_id: user.id,
        };

        participation.save(connection)?;

        Ok(user)
    });

    // See if it worked
    match user {
        Result::Err(err) => {
            // Check if the error is due a username uniqueness constraint violation
            if let Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) = err
            {
                log::warn!("users.signup.save.taken: {}", err);
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
        // Set the user session
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
    server_state: actix_web::web::Data<ServerState>,
) -> Result<&'static str, EndpointError> {
    // Get the connection from the mutex
    let mut connection = server_state
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
            // Set the user session
            set_session(session, &user).expect("Could not serialise user");
            Result::Ok("Success!")
        }
    }
}

/// An API endpoint used to check if the user is logged in
#[get("/api/is_logged_in")]
pub async fn is_logged_in(session: Session) -> impl Responder {
    let user = get_session(session);
    Json(user.is_some())
}
