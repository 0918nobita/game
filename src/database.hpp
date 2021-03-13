// Copyright 2021 Kodai Matsumoto

#pragma once

#include <sqlite3.h>

#include <memory>

struct SQLite3Deleter {
    void operator()(sqlite3 *db);
};

struct SQLite3StmtDeleter {
    void operator()(sqlite3_stmt *stmt);
};

struct Database {
    virtual void showRecords() = 0;
    virtual ~Database() {}
};

class SQLite3DB : public Database {
    std::unique_ptr<sqlite3, SQLite3Deleter> db;
    static sqlite3 *open_db();

   public:
    SQLite3DB();
    void showRecords();
};
