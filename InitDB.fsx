#r "nuget: SQLitePCLRaw.bundle_e_sqlite3"
#r "nuget: Microsoft.Data.Sqlite.Core"

open Microsoft.Data.Sqlite
open System.Runtime.CompilerServices

[<Struct; IsReadOnly>]
type Scene = Scene of id : int * title : string

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

let inline selectRecords (conn : SqliteConnection) =
    use cmd = conn.CreateCommand()
    cmd.CommandText <- "SELECT * FROM scenes"
    use dataReader = cmd.ExecuteReader()
    seq {
        while dataReader.Read() do
            yield Scene(dataReader.GetInt32(0), dataReader.GetString(1))
    }
    |> List.ofSeq

let () =
    use conn = new SqliteConnection("Data Source=save_data.sqlite3")
    conn.Open()
    if not (tableExists conn)
    then
        createTable conn
        insertRecords conn
    selectRecords conn
    |> List.iter (printfn "%O")
