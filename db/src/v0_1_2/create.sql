-- GeekORM Database Migrations

-- Users Table
CREATE TABLE IF NOT EXISTS Users (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL UNIQUE, email TEXT, user_type BLOB NOT NULL, password TEXT NOT NULL, created_at TEXT NOT NULL);

-- Sessions Table
CREATE TABLE IF NOT EXISTS Sessions (id INTEGER PRIMARY KEY AUTOINCREMENT, token TEXT NOT NULL);

-- Posts Table
CREATE TABLE IF NOT EXISTS Posts (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, user INTEGER NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, FOREIGN KEY (user) REFERENCES Users(id));

