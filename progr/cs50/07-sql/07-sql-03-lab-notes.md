Database - songs.db - 2018 top 100 Spotify songs

~/lab7/ $ sqlite3 songs.db 
SQLite version 3.34.0 2020-12-01 16:14:00
Enter ".help" for usage hints.
sqlite> .schema
CREATE TABLE songs (
    id INTEGER,
    name TEXT,
    artist_id INTEGER,
    danceability REAL,
    energy REAL,
    key INTEGER,
    loudness REAL,
    speechiness REAL,
    valence REAL,
    tempo REAL,
    duration_ms INTEGER
);
CREATE TABLE artists (
    id INTEGER,
    name TEXT
);


In 6.sql, write a SQL query that lists the names of songs that are by Post Malone. 

```
SELECT name FROM songs
WHERE artist_id = (SELECT id FROM artists WHERE name = 'Post Malone')

--SELECT songs.name, songs.artist_id, artists.name
--FROM songs
--JOIN artists ON songs.artist_id = artists.id
--WHERE artists.name = 'Post Malone'
```

In 7.sql, write a SQL query that returns the average energy of songs that are by Drake. 


```
SELECT AVG(energy) FROM songs
WHERE artist_id = (SELECT id FROM artists WHERE name = 'Drake')
```

Resources

 - https://www.w3schools.com/sql/sql_ref_keywords.asp
 
