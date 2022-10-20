use std::borrow::BorrowMut;

use actix_session::Session;
use actix_web::{get, web, Responder};
use serde::Serialize;

use crate::{
    data::{events::Event, session::use_session},
    ServerState,
};

use diesel::BelongingToDsl;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use super::EndpointError;

/// A struct used for a `get_events` response
#[derive(Serialize)]
struct GetEventsResponse {
    events: Vec<Event>,
}
/// An API endpoint used to register a user
#[get("/api/get_events")]
pub async fn get_events(
    session: Session,
    data: actix_web::web::Data<ServerState>,
) -> Result<impl Responder, EndpointError> {
    use_session!(session, user);

    // Get the connection from the mutex
    let mut connection = data
        .connection
        .lock()
        .expect("Could not get the connection from ServerState");

    let events = Event::get_events_with_user(&mut connection, &user);

    match events {
        Ok(events) => Ok(web::Json(GetEventsResponse { events })),
        Err(err) => {
            // Log the error
            log::error!("events.get_events.database: {}", err);
            Result::Err(EndpointError::InternalError)
        }
    }
}
