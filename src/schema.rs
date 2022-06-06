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

allow_tables_to_appear_in_same_query!(
    categories,
    items,
);
