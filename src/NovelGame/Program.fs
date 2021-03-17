open Microsoft.Data.Sqlite
open System
open System.Runtime.CompilerServices
open System.Runtime.InteropServices

[<Struct; IsReadOnly>]
type Scene = Scene of id : int * title : string

let inline selectRecords (conn : SqliteConnection) =
    use cmd = conn.CreateCommand()
    cmd.CommandText <- "SELECT * FROM scenes"
    use dataReader = cmd.ExecuteReader()
    seq {
        while dataReader.Read() do
            yield Scene(dataReader.GetInt32(0), dataReader.GetString(1))
    }
    |> List.ofSeq

module Cpp =
    [<DllImport("lib/libgraphics.so")>]
    extern int run()

[<EntryPoint>]
let main _ =
    printfn "isWindows: %b" <| OperatingSystem.IsWindows()
    printfn "isMacOS: %b" <| OperatingSystem.IsMacOS()
    printfn "isLinux: %b" <| OperatingSystem.IsLinux()

    use conn = new SqliteConnection("Data Source=save_data.sqlite3")
    conn.Open()

    selectRecords conn
    |> List.iter (printfn "%O")

    Cpp.run() |> ignore
    0
