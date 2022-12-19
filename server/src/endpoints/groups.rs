use actix_session::Session;
use actix_web::{get, post, web::Json, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    data::{
        group::{Group, UnsavedGroup, UnsavedGroupParticipant, UserParticipationData},
        models::UnsavedModel,
        session::use_session,
    },
    ServerState,
};

use super::EndpointError;

#[derive(Deserialize)]
pub struct CreateGroupRequest {
    name: String,
}

#[derive(Deserialize)]
pub struct InviteToGroupRequest {
    user_id: i32,
    group_id: i32,
}

#[derive(Serialize)]
/// A struct that represents the response to get owned groups and participants
pub struct GetOwnedGroupsWithParticipantsResponse {
    groups: Vec<Group>,
    participants: Vec<UserParticipationData>,
}

/// An API endpoint to get all groups with their users that are owned by this user
#[get("/api/get_owned_groups_with_participants")]
pub async fn get_owned_groups_with_participants(
    session: Session,
    server_state: actix_web::web::Data<ServerState>,
) -> Result<impl Responder, EndpointError> {
    use_session!(session, user);

    // Get the connection from the mutex
    let mut connection = server_state
        .connection
        .lock()
        .expect("Could not get the connection from ServerState");

    // First get the groups the user owns
    let groups = Group::get_owned_groups(&mut connection, &user);
    // Then try to get the users in those groups
    let response = groups.and_then(|groups| {
        let participants = Group::get_owned_groups_participants(&mut connection, &user)?;
        Ok(GetOwnedGroupsWithParticipantsResponse {
            groups,
            participants,
        })
    });

    match response {
        Ok(data) => Ok(Json(data)),
        Err(err) => {
            // Log the error
            log::error!("groups.get_owned_groups_with_participants.get: {}", err);
            Err(EndpointError::InternalError)
        }
    }
}

/// An API endpoint used to create a group
#[post("/api/create_group")]
pub async fn create_group(
    session: Session,
    req_body: Json<CreateGroupRequest>,
    server_state: actix_web::web::Data<ServerState>,
) -> Result<&'static str, EndpointError> {
    use_session!(session, user);

    // Get the connection from the mutex
    let mut connection = server_state
        .connection
        .lock()
        .expect("Could not get the connection from ServerState");

    let CreateGroupRequest { name } = req_body.0;

    let group = UnsavedGroup {
        name,
        owner_id: user.id,
    };

    let group = group.save(&mut connection);

    match group {
        Err(err) => {
            // log the error
            log::error!("groups.create_group.save: {}", err);
            Err(EndpointError::InternalError)
        }
        Ok(_) => Ok("Success!"),
    }
}

/// An API endpoint used to add a user to a group
#[post("/api/invite_to_group")]
pub async fn invite_to_group(
    session: Session,
    req_body: Json<InviteToGroupRequest>,
    server_state: actix_web::web::Data<ServerState>,
) -> Result<&'static str, EndpointError> {
    use_session!(session, user);

    // Get the connection from the mutex
    let mut connection = server_state
        .connection
        .lock()
        .expect("Could not get the connection from ServerState");

    let InviteToGroupRequest { group_id, user_id } = req_body.0;
    //
    // check that the user has admin rights over the group
    let group = Group::get_group_by_id(&mut connection, group_id);
    match group {
        Err(err) => {
            // log the error
            log::error!("groups.invite_to_group.find_group: {}", err);
            Err(EndpointError::InternalError)
        }
        // If the group was not found
        Ok(None) => Err(EndpointError::BadClientData("This group does not exist")),
        Ok(Some(group)) => {
            if group.owner_id != user.id {
                return Err(EndpointError::BadClientData(
            "You are not the group owner and so do not have the permission to create events."));
            }
            // Create a relationship between the user and the group
            let group_participant = UnsavedGroupParticipant {
                group_id,
                participant_id: user_id,
                participation_type: crate::data::events::ParticipationType::NoResponse,
            };

            // Try to save it
            let groups_participant = group_participant.save(&mut connection);

            match groups_participant {
                Err(err) => {
                    // log the error
                    log::error!("groups.invite_to_group.save: {}", err);
                    Err(EndpointError::InternalError)
                }
                Ok(_) => Ok("Success!"),
            }
        }
    }
}
