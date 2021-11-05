**SQL**

 - MySQL - open source platform
   - SQLite
 - a lot of installations of SQL come with a GUI - phpMyAdmin.

Database -> table -> column -> row

Column - data of a particular data type

 - INT - smallint, tinyint, mediumint, bigint
 - DECIMAL, FLOAT
 - DATE, TIME, DATETIME, TIMESTAMP
 - GEOMETRY, LINESTRING - mapping out of a drawing on a map
 - TEXT, CHAR, VARCHAR (CHAR(n) - exactly n characters; VARCHAR(n) - max n)
 - ENUM - limited set of values
 - BLOB

In SQLite all of the above are reduced to

 - NULL, INTEGER, REAL, TEXT, BLOB

To uniquely identify a row -> primary key

 - can be a joint primary key (2 columns)
 - if it is an integer it can be configured to autoincrement

------------------------------------------------------------------------------

SQL Operations

INSERT INTO table (column, ...) VALUES (value, ...)

SELECT columns FROM table WHERE predicate ORDER BY column
note: '*' is a shorthand for every column

SELECT columns FROM table JOIN table ON predicate
```
SELECT users.username, moms.mother 
FROM users
JOIN moms 
ON user.username = moms.username
```

UPDATE table SET column = value WHERE predicate

DELETE FROM table WHERE predicate


