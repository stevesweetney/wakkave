table! {
    posts (id) {
        id -> Int4,
        content -> Text,
        valid -> Bool,
        created_at -> Timestamp,
        user_id -> Int4,
    }
}

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

table! {
    votes (user_id, post_id) {
        user_id -> Int4,
        post_id -> Int4,
        up_or_down -> Int2,
    }
}

joinable!(posts -> users (user_id));
joinable!(votes -> posts (post_id));
joinable!(votes -> users (user_id));

allow_tables_to_appear_in_same_query!(posts, sessions, users, votes,);
