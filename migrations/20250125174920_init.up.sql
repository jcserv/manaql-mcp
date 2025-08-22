CREATE TABLE Author (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE Book (
    id SERIAL PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL,
    author_id SERIAL NOT NULL REFERENCES Author(id)
);

INSERT INTO Author (name) VALUES
    ('Andy Weir'),
    ('David Quamen'),
    ('Lola Olufemi');

INSERT INTO Book (title, author_id) VALUES
    ('The Martian', 1),
    ('Natural Acts', 2),
    ('Feminism Interrupted', 3),
    ('Project Hail Mary', 1);