table! {
    jwt_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        access_token -> Varchar,
        access_token_expires_at -> Timestamp,
        refresh_token -> Varchar,
        refresh_token_expires_at -> Timestamp,
    }
}

table! {
    profiles (id) {
        id -> Int4,
        user_id -> Int4,
        status -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        solana_pubkey -> Nullable<Varchar>,
        ethereum_pubkey -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        avatar -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    jwt_tokens,
    profiles,
    users,
);
