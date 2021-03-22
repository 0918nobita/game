module Program

open Microsoft.Data.Sqlite
open System
open System.Runtime.CompilerServices
open System.Runtime.InteropServices

module VulkanApp =
    [<DllImport("NovelGameCpp")>]
    extern int run()

type OSType = Windows64 | MacOS64 | Linux64 | Other

[<Struct; IsReadOnly>]
type Row = Row of id : int * title : string

[<EntryPoint>]
let main _ =
    let inline selectRecords (conn : SqliteConnection) =
        use cmd = conn.CreateCommand()
        cmd.CommandText <- "SELECT * FROM scenes"
        use dataReader = cmd.ExecuteReader()
        seq {
            while dataReader.Read() do
                yield Row(dataReader.GetInt32(0), dataReader.GetString(1))
        }
        |> List.ofSeq

    let task =
        async {
            use conn = new SqliteConnection("Data Source=save_data.sqlite3")
            conn.Open()
            selectRecords conn
            |> List.iter (printfn "%O")
        }

    Async.Start(task)

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

    match getOSType () with
    | Windows64 | MacOS64 | Linux64 -> VulkanApp.run()
    | Other -> failwith "This platform is not supported"
