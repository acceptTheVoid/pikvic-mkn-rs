CREATE TABLE categories (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(60) NOT NULL
);

INSERT INTO categories (name) VALUES ("Компьютеры"), ("Мониторы"), ("Молоко");

CREATE TABLE items (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(60) NOT NULL,
    price INTEGER NOT NULL,
    cat_id INTEGER NOT NULL
);

INSERT INTO items (name, price, cat_id) VALUES 
("PC2000", 2000, 1), 
("Laptop100500", 1000, 1), 
("Acer 19", 888, 2);
