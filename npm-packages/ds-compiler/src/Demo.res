module Cont: {
    type t<'r, 'a>

    let run: t<'r, 'a> => ('a => 'r) => 'r

    let return: 'a => t<'r, 'a>

    let bind: t<'r, 'a> => ('a => t<'r, 'b>) => t<'r, 'b>
} = {
    type t<'r, 'a> = Cont(('a => 'r) => 'r)

    let run = (Cont(runCont)) => runCont

    let return = a => Cont(k => k(a))

    let bind = (m, f) => Cont(k => m->run(a => f(a)->run(k)))
}

let c: Cont.t<int, int> =
    Cont.return(10)
        ->Cont.bind(x => Cont.return(x + 1))

Js.log(Cont.run(c, x => x))  // 11

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
