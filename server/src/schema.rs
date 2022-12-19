// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "participation_type"))]
    pub struct ParticipationType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "recurrence_type"))]
    pub struct RecurrenceType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "visibility_type"))]
    pub struct VisibilityType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::VisibilityType;
    use super::sql_types::RecurrenceType;

    events (id) {
        id -> Int4,
        owner_id -> Int4,
        title -> Varchar,
        visibility -> VisibilityType,
        start_time -> Int4,
        duration -> Int4,
        recurrence -> RecurrenceType,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ParticipationType;

    events_participants (id) {
        id -> Int4,
        event_id -> Int4,
        participant_id -> Int4,
        participation_type -> ParticipationType,
    }
}

diesel::table! {
    friendships (id) {
        id -> Int4,
        owner_id -> Int4,
        friend_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Text,
    }
}

diesel::joinable!(events -> users (owner_id));
diesel::joinable!(events_participants -> events (event_id));
diesel::joinable!(events_participants -> users (participant_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    events_participants,
    friendships,
    users,
);
