Fs.readFile(~encoding="utf-8", ~filename="example.tdrama")
|> Js.Promise.then_(res => {
    Js.log(res)
    Js.Promise.resolve(())
})
|> ignore
