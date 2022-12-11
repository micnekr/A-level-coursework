use actix_session::Session;
use actix_web::{get, web::Json, Responder};

use crate::{
    data::{
        notifications::{NotificaitonVec, Notification},
        session::use_session,
    },
    endpoints::EndpointError,
    ServerState,
};

/// An API endpoint used to check the number of notifications
#[get("/api/get_notifications")]
pub async fn get_notifications(
    session: Session,
    server_state: actix_web::web::Data<ServerState>,
) -> Result<impl Responder, EndpointError> {
    use_session!(session, user);

    // Get the connection from the mutex
    let mut connection = server_state
        .connection
        .lock()
        .expect("Could not get the connection from ServerState");

    let notificatons = Notification::get_user_notifications(&mut connection, &user);

    // See if that worked
    match notificatons {
        Err(err) => {
            log::error!("notifications.get_notifications.get: {}", err);
            Result::Err(EndpointError::InternalError)
        }
        Ok(notifications) => Ok(Json(NotificaitonVec(notifications))),
    }
}
