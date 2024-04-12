// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;

    swift_user (id) {
        id -> Uuid,
        email -> Text,
        password -> Nullable<Text>,
        first_name -> Text,
        last_name -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    organisation_user_role (id) {
        id -> BigSerial,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    organisation (id) {
        id -> Uuid,
        owner -> Uuid,
        name -> Text,
        is_archived -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    swift_user_accessible_organisation (organisation_id, swift_user_id) {
        organisation_id -> Uuid,
        swift_user_id -> Uuid,
        role_id -> BigInt
    }
}

diesel::table! {
    use diesel::sql_types::*;

    application (id) {
        id -> Uuid,
        organisation_id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    organisation,
    swift_user_accessible_organisation,
    swift_user
);