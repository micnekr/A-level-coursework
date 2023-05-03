use actix_session::Session;
use actix_web::{get, post, web::Json, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    data::{
        events::{Event, RecurrenceType, UnsavedEvent, VisibilityType},
        group::Group,
        models::UnsavedModel,
        session::use_session,
    },
    ServerState,
};

use super::EndpointError;

/// A struct used for a `get_events` response
#[derive(Serialize)]
struct GetEventsResponse {
    events: Vec<Event>,
}
/// An API endpoint used to get events a user needs to attend
#[get("/api/get_events")]
pub async fn get_events(
    session: Session,
    server_state: actix_web::web::Data<ServerState>,
) -> Result<impl Responder, EndpointError> {
    use_session!(session, user);

    // Get the connection from the mutex
    let mut connection = server_state
        .connection
        .lock()
        .expect("Could not get the connection from ServerState");

    let events = Event::get_accepted_events_with_user(&mut connection, &user);

    // If the events are found, send them to the frontend
    match events {
        Ok(events) => Ok(Json(GetEventsResponse { events })),
        Err(err) => {
            // Log the error
            log::error!("events.get_events.get: {}", err);
            Result::Err(EndpointError::InternalError)
        }
    }
}

/// A struct for create_event requests
#[derive(Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub visibility: VisibilityType,
    pub start_time: i32,
    pub duration: i32,
    pub recurrence: RecurrenceType,
    pub group_id: i32,
}

/// An API endpoint used to create an event
#[post("/api/create_event")]
pub async fn create_event(
    session: Session,
    req_body: Json<CreateEventRequest>,
    server_state: actix_web::web::Data<ServerState>,
) -> Result<&'static str, EndpointError> {
    use_session!(session, user);

    // Get the connection from the mutex
    let mut connection = server_state
        .connection
        .lock()
        .expect("Could not get the connection from ServerState");

    let CreateEventRequest {
        title,
        visibility,
        recurrence,
        start_time,
        duration,
        group_id,
    } = req_body.0;

    // check that the user has admin rights over the group
    let group = Group::get_group_by_id(&mut connection, group_id);
    match group {
        Err(err) => {
            // log the error
            log::error!("events.create_event.find_group: {}", err);
            Err(EndpointError::InternalError)
        }
        // If the group was not found
        Ok(None) => Err(EndpointError::BadClientData("This group does not exist")),
        Ok(Some(group)) => {
            // If the user is not the owner
            if group.owner_id != user.id {
                return Err(EndpointError::BadClientData(
            "You are not the group owner and so do not have the permission to create events.",
        ));
            }

            let event = UnsavedEvent {
                title,
                visibility,
                recurrence,
                start_time,
                duration,
                group_id,
            };

            match event.save(&mut connection) {
                Ok(_) => Ok("Success!"),
                Err(err) => {
                    // Generic error

                    // Log the error
                    log::error!("events.create_event.save: {}", err);
                    Result::Err(EndpointError::InternalError)
                }
            }
        }
    }
}
