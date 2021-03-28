@bs.module("fs")
@bs.scope("promises")
external unsafeReadFile: string => string => Js.Promise.t<string> = "readFile"

let readFile = (~encoding, ~filename) =>
    unsafeReadFile(filename, encoding)
    |> Js.Promise.then_(content => content->Some->Js.Promise.resolve)
    |> Js.Promise.catch(_ => Js.Promise.resolve(None))
