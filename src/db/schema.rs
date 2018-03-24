table! {
    urls (id) {
        id -> Int4,
        target -> Text,
        hash -> Varchar,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

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

table! {
    user_urls (user_id) {
        user_id -> Int4,
        url_id -> Int4,
    }
}

joinable!(user_urls -> urls (url_id));
joinable!(user_urls -> users (user_id));

allow_tables_to_appear_in_same_query!(
    urls,
    users,
    user_urls,
);
