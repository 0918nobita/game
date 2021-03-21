open System.IO
open System.Text

let compileToBinaryFormat () =
    let bytes = Encoding.ASCII.GetBytes "DRAMA"
    File.WriteAllBytes("main.drama", bytes)

let loadBinary () =
    let bytes = File.ReadAllBytes("main.drama")
    if bytes.Length < 5 || bytes.[0..5] <> [|68uy; 82uy; 65uy; 77uy; 65uy|]
    then eprintfn "Failed to load binary: Invalid format"

[<EntryPoint>]
let main _ =
    printfn "DramaScript Engine"
    compileToBinaryFormat ()
    loadBinary ()
    0
