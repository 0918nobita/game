open Microsoft.Data.Sqlite
open System
open System.Runtime.CompilerServices
open System.Runtime.InteropServices

[<Struct; IsReadOnly>]
type Scene = Scene of id : int * title : string

module Cpp =
    [<DllImport("bin/libgraphics.so")>]
    extern int run()

type OSType = Windows64 | MacOS64 | Linux64 | Other

[<EntryPoint>]
let main _ =
    let inline getOSType () =
        let is64bitOS = Environment.Is64BitOperatingSystem
        let isWindows = OperatingSystem.IsWindows()
        let isMacOS = OperatingSystem.IsMacOS()
        let isLinux = OperatingSystem.IsLinux()
        match (is64bitOS, isWindows, isMacOS, isLinux) with
        | (true, true, false, false) -> Windows64
        | (true, false, true, false) -> MacOS64
        | (true, false, false, true) -> Linux64
        | _ -> Other

    printfn "OSType: %A" <| getOSType ()

    let task1 = async { Cpp.run() |> ignore }


    let inline selectRecords (conn : SqliteConnection) =
        use cmd = conn.CreateCommand()
        cmd.CommandText <- "SELECT * FROM scenes"
        use dataReader = cmd.ExecuteReader()
        seq {
            while dataReader.Read() do
                yield Scene(dataReader.GetInt32(0), dataReader.GetString(1))
        }
        |> List.ofSeq

    let task2 =
        async {
            use conn = new SqliteConnection("Data Source=save_data.sqlite3")
            conn.Open()
            selectRecords conn
            |> List.iter (printfn "%O")
        }

    [task1; task2]
    |> Async.Parallel
    |> Async.RunSynchronously
    |> ignore
    0
