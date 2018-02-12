table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Text,
        admin -> Nullable<Bool>,
        welcome -> Nullable<Bool>,
        avatar_url -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}
