#r "nuget: SQLitePCLRaw.bundle_e_sqlite3"
#r "nuget: Microsoft.Data.Sqlite.Core"

open Microsoft.Data.Sqlite

let inline tableExists (conn : SqliteConnection) =
    use cmd = conn.CreateCommand()
    cmd.CommandText <-
        "SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'scenes'"
    use dataReader = cmd.ExecuteReader()
    dataReader.Read()

let inline createTable (conn : SqliteConnection) =
    use cmd = conn.CreateCommand()
    cmd.CommandText <-
        "CREATE TABLE scenes (id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL)"
    cmd.ExecuteNonQuery() |> ignore

let inline insertRecords (conn : SqliteConnection) =
    use cmd = conn.CreateCommand()
    cmd.CommandText <- "INSERT INTO scenes VALUES(1, 'hoge')"
    cmd.ExecuteNonQuery() |> ignore

let () =
    use conn = new SqliteConnection("Data Source=save_data.sqlite3")
    conn.Open()
    if not (tableExists conn)
    then
        createTable conn
        insertRecords conn
