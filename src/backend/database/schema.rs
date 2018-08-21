table! {
    sessions (id) {
        id -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        karma -> Int4,
        streak -> Int2,
    }
}

allow_tables_to_appear_in_same_query!(sessions, users,);
