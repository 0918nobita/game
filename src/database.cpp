// Copyright 2021 Kodai Matsumoto

#include <sqlite3.h>

#include <iostream>

int main() {
    sqlite3 *db = nullptr;
    auto err = sqlite3_open("save_data.sqlite3", &db);
    if (err != SQLITE_OK) {
        std::cerr << "Failed to open database (errno: " << err << ")" << std::endl;
        std::exit(EXIT_FAILURE);
    }

    sqlite3_stmt *stmt = nullptr;
    err = sqlite3_prepare(db, "SELECT * FROM scenes", -1, &stmt, nullptr);
    if (err != SQLITE_OK) {
        std::cerr << "Failed to create prepared statement (errno: " << err << ")" << std::endl;
        std::exit(EXIT_FAILURE);
    }

    while (true) {
        auto res = sqlite3_step(stmt);
        if (res == SQLITE_ROW) {
            int id = sqlite3_column_int(stmt, 0);
            auto title = sqlite3_column_text(stmt, 1);
            std::cout << "id: " << id << ", title: " << title << std::endl;
        } else if (res == SQLITE_DONE) {
            break;
        } else {
            std::cout << "Failed to get column (errno: " << res << ")" << std::endl;
            std::exit(EXIT_FAILURE);
        }
    }

    sqlite3_finalize(stmt);

    err = sqlite3_close(db);
    if (err != SQLITE_OK) {
        std::cerr << "Failed to close database (errno: " << err << ")" << std::endl;
        std::exit(EXIT_FAILURE);
    }
}
