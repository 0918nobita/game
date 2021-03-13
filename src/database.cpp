// Copyright 2021 Kodai Matsumoto

#include "database.hpp"

#include <sqlite3.h>

#include <iostream>

void SQLite3Deleter::operator()(sqlite3 *db) {
    int err = sqlite3_close(db);
    if (err != SQLITE_OK) {
        std::cerr << "Failed to close database (errno: " << err << ")" << std::endl;
        std::exit(EXIT_FAILURE);
    }
}

void SQLite3StmtDeleter::operator()(sqlite3_stmt *stmt) { sqlite3_finalize(stmt); }

sqlite3 *SQLite3DB::open_db() {
    sqlite3 *db_raw_ptr = nullptr;
    int err = sqlite3_open("save_data.sqlite3", &db_raw_ptr);
    if (err != SQLITE_OK) {
        std::cerr << "Failed to open database (errno: " << err << ")" << std::endl;
        std::exit(EXIT_FAILURE);
    }
    return db_raw_ptr;
}

SQLite3DB::SQLite3DB() : db(open_db()) {}

void SQLite3DB::showRecords() {
    sqlite3_stmt *stmt_raw_ptr = nullptr;
    int err = sqlite3_prepare(db.get(), "SELECT * FROM scenes", -1, &stmt_raw_ptr, nullptr);
    if (err != SQLITE_OK) {
        std::cerr << "Failed to create prepared statement (errno: " << err << ")" << std::endl;
        std::exit(EXIT_FAILURE);
    }

    std::unique_ptr<sqlite3_stmt, SQLite3StmtDeleter> stmt(stmt_raw_ptr);

    while (true) {
        int res = sqlite3_step(stmt_raw_ptr);
        if (res == SQLITE_ROW) {
            int id = sqlite3_column_int(stmt_raw_ptr, 0);
            auto title = sqlite3_column_text(stmt_raw_ptr, 1);
            std::cout << "id: " << id << ", title: " << title << std::endl;
        } else if (res == SQLITE_DONE) {
            break;
        } else {
            std::cout << "Failed to get column (errno: " << res << ")" << std::endl;
            std::exit(EXIT_FAILURE);
        }
    }
}
