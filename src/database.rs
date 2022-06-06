use diesel::prelude::*;
use crate::models::*;

pub fn get_categories(conn: &SqliteConnection) -> Vec<Category> {
    use crate::schema::categories;
    categories::table
        .load::<Category>(conn)
        .expect("Failed fetching categories")
}

pub fn get_category(cat_id: i32, conn: &SqliteConnection) -> Option<Category> {
    use crate::schema::categories;
    categories::table.find(cat_id).first(conn).ok()
}

pub fn get_items(cat_id: i32, conn: &SqliteConnection) -> Vec<Item> {
    use crate::schema::*;
    items::table
        .filter(items::dsl::cat_id.eq(cat_id))
        .load::<Item>(conn)
        .expect("Failed fetching items")
}
