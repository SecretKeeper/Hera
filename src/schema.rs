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
