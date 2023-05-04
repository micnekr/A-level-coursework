use actix_session::Session;
use actix_web::{get, post, web::Json, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    data::{
        friends::{FriendAddResult, Friendship},
        session::use_session,
        users::{User, UserPublic},
    },
    ServerState,
};

use super::EndpointError;

/// A struct to represent responses to get_friends requests
#[derive(Serialize)]
struct GetFriendsResponse(Vec<UserPublic>);

/// A struct to represent add_friend requests
#[derive(Deserialize)]
pub struct AddFriendRequest {
    username: String,
}

#[get("/api/get_friends")]
pub async fn get_friends(
    session: Session,
    server_state: actix_web::web::Data<ServerState>,
) -> Result<impl Responder, EndpointError> {
    use_session!(session, user);
    // Get the connection from the mutex
    let mut connection = server_state
        .connection
        .lock()
        .expect("Could not get the connection from ServerState");

    let friends = Friendship::get_friends(&mut connection, &user);

    match friends {
        Ok(friends) => {
            // make it safe to send to the frontend
            let friends: Vec<_> = friends.into_iter().map(User::to_public).collect();
            Ok(Json(GetFriendsResponse(friends)))
        }
        Err(err) => {
            // log the error
            log::error!("friendships.get_friends.get: {}", err);
            Result::Err(EndpointError::InternalError)
        }
    }
}

#[post("/api/add_friend")]
pub async fn add_friend(
    session: Session,
    req_body: Json<AddFriendRequest>,
    server_state: actix_web::web::Data<ServerState>,
) -> Result<&'static str, EndpointError> {
    use_session!(session, user);
    // Get the connection from the mutex
    let mut connection = server_state
        .connection
        .lock()
        .expect("Could not get the connection from ServerState");

    let friend_username = req_body.username.clone();

    // Try to add the friend and record it to the database
    let result = Friendship::add_friend(&mut connection, &user, &friend_username);
    match result {
        Err(err) => {
            // Log the error
            log::error!("friendships.add_friend.save: {}", err);
            Result::Err(EndpointError::InternalError)
        }
        // Send different error messages for different causes of failure
        Ok(result) => match result {
            FriendAddResult::TriedFriendThemselves => Err(EndpointError::BadClientData(
                "You can not add yourself as a friend",
            )),
            FriendAddResult::AlreadyFriends => Err(EndpointError::BadClientData(
                "You are already friends with this user.",
            )),
            FriendAddResult::UsernameNotFound => {
                Err(EndpointError::BadClientData("This user does not exist."))
            }
            FriendAddResult::Success => Ok("Success!"),
        },
    }
}
