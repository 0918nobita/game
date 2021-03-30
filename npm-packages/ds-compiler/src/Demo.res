let engine = Textlint.makeEngine(~rules = ["no-hankaku-kana"])

engine
    ->Textlint.executeOnText(`こんにちは。\n半角ｶﾀｶﾅ`)
    ->Promise.then_(results => {
        open Belt

        results
        ->Array.forEach(result =>
            result.messages
            ->Array.forEach(msg => {
                let line = Int.toString(msg.line)
                let column = Int.toString(msg.column)
                Js.log(`(Ln ${line}, Col ${column}) ${msg.message}`)
            }))

        Fs.readFile("example.tdrama")
            ->Promise.then_(content => {
                Js.log(content)
                Promise.resolve(())
            })
    })
    ->ignore
