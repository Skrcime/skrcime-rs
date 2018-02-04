table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Text,
        admin -> Nullable<Bool>,
        welcome -> Nullable<Bool>,
        avatar_url -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}
