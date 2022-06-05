extern crate diesel;

use diesel::prelude::*;
use models::*;
use rocket_test::*;

fn main() {
    use schema::items;

    let items = vec![
        NewItem {
            name: "ASDLK",
            price: 1233,
            description: "jsadklsaksalkjdasjjk dsasa",
        },
        NewItem {
            name: "AdsLK",
            price: 532,
            description: "jsadklsaksalkjdas32jjk dsasa",
        },
        NewItem {
            name: "asdasDLK",
            price: 234,
            description: "jsadklsaksalk23jdasjjk dsasa",
        },
        NewItem {
            name: "ASsdaK",
            price: 222,
            description: "jsadklsaksalkjasddasjjk dsasa",
        },
        NewItem {
            name: "ASDLasdK",
            price: 12,
            description: "jsadklsaksalkjasddasjjk dsasa",
        },
    ];

    let conn = connect();

    diesel::insert_into(items::table)
        .values(&items)
        .execute(&conn)
        .expect("Failed inserting into table");
}
