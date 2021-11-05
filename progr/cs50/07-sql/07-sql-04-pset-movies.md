Database - movies.db

sqlite3 movies.db 
SQLite version 3.34.0 2020-12-01 16:14:00
Enter ".help" for usage hints.
sqlite> .schema
CREATE TABLE movies (
                    id INTEGER,
                    title TEXT NOT NULL,
                    year NUMERIC,
                    PRIMARY KEY(id)
                );
CREATE TABLE stars (
                movie_id INTEGER NOT NULL,
                person_id INTEGER NOT NULL,
                FOREIGN KEY(movie_id) REFERENCES movies(id),
                FOREIGN KEY(person_id) REFERENCES people(id)
            );
CREATE TABLE directors (
                movie_id INTEGER NOT NULL,
                person_id INTEGER NOT NULL,
                FOREIGN KEY(movie_id) REFERENCES movies(id),
                FOREIGN KEY(person_id) REFERENCES people(id)
            );
CREATE TABLE ratings (
                movie_id INTEGER NOT NULL,
                rating REAL NOT NULL,
                votes INTEGER NOT NULL,
                FOREIGN KEY(movie_id) REFERENCES movies(id)
            );
CREATE TABLE people (
                id INTEGER,
                name TEXT NOT NULL,
                birth NUMERIC,
                PRIMARY KEY(id)
            );

            
In 8.sql, write a SQL query to list the names of all people who starred in Toy Story.

    Your query should output a table with a single column for the name of each person.
    You may assume that there is only one movie in the database with the title Toy Story.


```
--SELECT name
--FROM people
--WHERE id IN
--  (SELECT person_id
--  FROM stars
--  WHERE movie_id =
--    (SELECT id
--    FROM movies
--    WHERE title = 'Toy Story'))

SELECT people.name
FROM movies
JOIN stars ON movies.id = stars.movie_id
JOIN people ON people.id = stars.person_id
WHERE movies.title = 'Toy Story'
```

In 9.sql, write a SQL query to list the names of all people who starred in a movie released in 2004, ordered by birth year.

    Your query should output a table with a single column for the name of each person.
    People with the same birth year may be listed in any order.
    No need to worry about people who have no birth year listed, so long as those who do have a birth year are listed in order.
    If a person appeared in more than one movie in 2004, they should only appear in your results once.

```
SELECT DISTINCT people.name, people.birth
FROM movies
JOIN stars ON movies.id = stars.movie_id
JOIN people ON people.id = stars.person_id
WHERE movies.year = 2004
ORDER BY people.birth
```

Top 5 movies of an actor.

```
SELECT movies.title--, ratings.rating
FROM movies
JOIN stars ON movies.id = stars.movie_id
JOIN people ON people.id = stars.person_id
JOIN ratings ON movies.id = ratings.movie_id
WHERE people.name = 'Chadwick Boseman'
ORDER BY ratings.rating DESC LIMIT 5
```

In 13.sql, write a SQL query to list the names of all people who starred in a movie in which Kevin Bacon also starred.

    Your query should output a table with a single column for the name of each person.
    There may be multiple people named Kevin Bacon in the database. Be sure to only select the Kevin Bacon born in 1958.
    Kevin Bacon himself should not be included in the resulting list.


```
SELECT DISTINCT people.name
FROM movies
JOIN stars ON movies.id = stars.movie_id
JOIN people ON people.id = stars.person_id
WHERE movies.id IN
  (SELECT movies.id
  FROM movies
  JOIN stars on movies.id = stars.movie_id
  JOIN people on people.id = stars.person_id
  WHERE people.id =
    (SELECT id
    FROM people
    WHERE name = 'Kevin Bacon' AND birth = 1958))
  AND name != 'Kevin Bacon'
```
            
