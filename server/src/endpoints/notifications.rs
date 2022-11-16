use actix_session::Session;
use actix_web::{get, web::Json, Responder};

use crate::{data::session::use_session, endpoints::EndpointError};

/// An API endpoint used to check the number of notifications
#[get("/api/get_notifications")]
pub async fn get_notifications(session: Session) -> Result<impl Responder, EndpointError> {
    use_session!(session, user);

    let notificatons = vec![3, 4];
    Ok(Json(notificatons))
}
