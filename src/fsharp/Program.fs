module Program

open Microsoft.Data.Sqlite
open System
open System.Runtime.CompilerServices

open VulkanApp

type OSType = Windows64 | MacOS64 | Linux64 | Other

[<Struct; IsReadOnly>]
type Scene = Scene of id : int * title : string

[<EntryPoint>]
let main _ =
    let inline selectRecords (conn : SqliteConnection) =
        use cmd = conn.CreateCommand()
        cmd.CommandText <- "SELECT * FROM scenes"
        use dataReader = cmd.ExecuteReader()
        seq {
            while dataReader.Read() do
                yield Scene(dataReader.GetInt32(0), dataReader.GetString(1))
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

    let app =
        match getOSType () with
        | Windows64 -> failwith "Not implemented"
        | MacOS64 -> VulkanAppForMacOS() :> VulkanApp
        | Linux64 -> VulkanAppForLinux() :> VulkanApp
        | Other -> failwith "This platform is not supported"

    app.Run() |> ignore
    0
