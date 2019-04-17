table! {
    users (id) {
        id -> Int4,
        uuid -> Uuid,
        email -> Varchar,
        encrypted_password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
