@bs.module("fs")
@bs.scope("promises")
external unsafeReadFile: string => string => Js.Promise.t<string> = "readFile"

let readFile = (filename) =>
    unsafeReadFile(filename, "utf-8")
        ->Promise.then_(content => Promise.resolve(Some(content)))
        ->Promise.catch(_ => Promise.resolve(None))
