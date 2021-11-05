**Database** is a file or program that stores data.

Flat-file database - all data in a single file. Best example: CSV. The first
line in a CSV is it's "schema", the names of each attribute of an entry.

Data massaging = cleaning up the data - canonicalize it.

 - all uppercase/lowercase
 - trim spaces
 - etc
 - note that real world data can take a lot of effort to clean up

Python

 - use DictReader to get dictionaries for the entries (based on the heading)
 - then you can easily iterate over the entries and access an attribute
 - use a set to add unique titles, for example; or a dict to count repititions
 - use a lambda function to adjust the way `sorted` work on the dictionary
   (you can also define a function separately and pass that)
 - use `with open()` to not have to close file manually
 - use `input` to get input from user

```
# Prints popularity of titles in CSV, sorted by popularity, using a lambda 
# function

import csv

# For accumulating (and later sorting) titles
titles = {}

# Open CSV file
with open("Favorite TV Shows - Form Responses 1.csv", "r") as file:

    # Create DictReader
    reader = csv.DictReader(file)

    # Iterate over CSV file
    for row in reader:

        # Canoncalize title
        title = row["title"].strip().upper()

        # Update counter
        if title in titles:
            titles[title] += 1
        else:
            titles[title] = 1

# Print titles in sorted order
for title in sorted(titles, key=lambda title: titles[title], reverse=True):
    print(title, titles[title])
```

syntax for lambda

    lambda argument(s): return-value

Get the popularity for a given item

```
# Searches CSV for popularity of a title

import csv

# Prompt user for title
title = input("Title: ").strip().upper()

# Open CSV file
with open("Favorite TV Shows - Form Responses 1.csv", "r") as file:

    # Create DictReader
    reader = csv.DictReader(file)

    # Iterate over CSV file, counting favorites
    counter = 0
    for row in reader:
        if row["title"].strip().upper() == title:
            counter += 1

# Print popularity
print(counter)
```
Relational databases - programs that give use more features to interact with
data (even if they persist the data in files)

Rows and columns -> in tables, instead of spreadsheets.
 
SQLite - a more lightweight implementation of SQL (structured query language)
used in mobile applications for example, iot etc

Tool: sqlite3

 - `.mode csv` - put it in cvs mode
 - `.import <csv file> <name of table>`
 - `.schema` - shows the tables in the database (the SQL for creating them)
 - `.save name.db`
 - `.timer ON` - see how long queries take

SQL - 4 fundamental operations
 
C reate - CREATE, INSERT
R ead   - INSERT
U pdate - UPDATE
D elete - DELETE

 - `CREATE TABLE table (column type, ...);`
 - `SELECT columns FROM table;`
   - `SELECT DISTINCT(UPPER(title)) FROM shows ORDER BY UPPER(title)`;
   - `SELECT title FROM shows WHERE title LIKE "%Office%";`

Count the shows. show the 10 most popula

```
SELECT UPPER(TRIM(title)), COUNT(title) FROM shows 
  GROUP BY UPPER(TRIM(title))
  ORDER BY COUNT(title) DESC LIMIT 10;
```

We can add, modify and delete items

```
INSERT INTO shows (Timestamp, title, genres) 
  VALUES("now", "x", "comedy, drama, musical");
  
UPDATE shows SET genres = "comedy, drama" WHERE title = "x";

DELETE FROM shows WHERE title LIKE "%friends%";
```

Designing and normalizing your database

 - eliminate redundancy
 - relationships between data
 

```
Table: Shows
  id
  title

Table: Genres
  show_id
  genre
  
id ----< show id
  one to many relantionship
```

Data types

 - INTEGER
 - REAL (~float)
 - TEXT
 - NUMERIC (number like, but not a simple integer)
 - BLOCB (binary large objects)

Column attributes

 - NOT NULL (can't be blank)
 - UNIQUE (doesn't accept duplicates)

Keys

 - PRIMARY KEY - column that uniquely identifies each row (auto-incrementing)
 - FOREIGN KEY - reference to another table's PRIMARY KEY

Python

 - library to bind python to sql database
 - create an empty file: open(file, w).close()
 - embed SQL queries and statements into Python
 - use split() to iterate through a comma separated text of items
 
```
# Imports titles and genres from CSV into a SQLite database

import cs50
import csv

# Create database
open("shows.db", "w").close()
db = cs50.SQL("sqlite:///shows.db")

# Create tables
db.execute("CREATE TABLE shows (id INTEGER, title TEXT, PRIMARY KEY(id))")
db.execute("CREATE TABLE genres (show_id INTEGER, genre TEXT, FOREIGN KEY(show_id) REFERENCES shows(id))")

# Open CSV file
with open("Favorite TV Shows - Form Responses 1.csv", "r") as file:

    # Create DictReader
    reader = csv.DictReader(file)

    # Iterate over CSV file
    for row in reader:

        # Canoncalize title
        title = row["title"].strip().upper()

        # Insert title
        id = db.execute("INSERT INTO shows (title) VALUES(?)", title)

        # Insert genres
        for genre in row["genres"].split(", "):
            db.execute("INSERT INTO genres (show_id, genre) VALUES(?, ?)", id, genre)
```

Using the information from two tables which have a relation between them

SUBQUERIES

```
SELECT title FROM show WHERE id in
  (SELECT show_id FROM genres WHERE genre = "Musical");
  
SELECT DISTINC(genre) FROM genres WHERE show_id in
  (SELECT id FROM shows WHERE title LIKE "%The Office%") 
  ORDER BY genre;
```

Further normalizing of the database, to remove duplicates in genres table

```
Table: Shows
  id
  title

Table shows_genres
  show_id
  genre_ide
  
Table: Genres
  id
  name
  
shows.id ----< show id
genre_id >---- genres.id

  join table that implements a relantionship
  many to many - one show can have any genre, on genre can have any show
```

Other data types (oracel, postgress etc)

 - smallint, integer, bigint
 - double precision
 - boolean, date, datetime, timestamp ...
 - char(n), varchar(n)


**IMDB database**

People, shows, stars and writers
Genres (a bit of duplication)
Ratings

Performance - indexing

 - Without an index, looking for 'the office' - 0.012s - O(n) complexity
 - but can create a helper structure - B-tree
 - `CREATE INDEX title_index ON shows (title);`
 - with index - 0.001s

JOIN

```
SELECT title FROM shows where id IN 
  (SELECT show_id FROM stars 
   WHERE person_id = (SELECT id FROM people WHERE name = "Steve Carrel"));

SELECT title FROM people 
  JOIN stars ON people.id = stars.person_id
  JOIN shows ON stars.show_id = shows.id
  WHERE name = "Steve Carell";
```

 - creating index on the tables used lowers the time for a join
 
**Problems**

1) User input -> SQL injection attach

 - using the SQL string specifier is good 
   `db.execute(".. WHERE user = ?", username)`
 - using the python string formatting is bad 
   `db.execute(f".. WHERE user= '{username}'")`
 => use placeholders and libraries

2) Race conditions

Example: increment number of likes

 - go to db, get the current number
 - increase it
 - update the entry in the database
 
In multiserver or multithreaded system, lines can be commingled. Solution is
to use TRANSACTIONS - lock a table or a row in a table

```
 BEGIN TRANSACTION
 ...
 COMMIT
```

#### Notes/Resources:

 - SQL keyword references - https://www.w3schools.com/sql/sql_ref_keywords.asp
 - SQL tutorial (and certificate0 - https://www.w3schools.com/sql/default.asp
 
other

 - functional programming -> "lambda" - Harvard CS51
 
