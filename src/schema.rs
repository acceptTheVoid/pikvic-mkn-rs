table! {
    categories (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    items (id) {
        id -> Integer,
        name -> Text,
        price -> Integer,
        cat_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    items,
    users,
);
