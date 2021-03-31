module Cont: {
    type t<'r, 'a>

    let run: t<'r, 'a> => ('a => 'r) => 'r

    let return: 'a => t<'r, 'a>

    let bind: t<'r, 'a> => ('a => t<'r, 'b>) => t<'r, 'b>

    let callCC: (('a => t<'r, 'b>) => t<'r, 'a>) => t<'r, 'a>
} = {
    type t<'r, 'a> = Cont(('a => 'r) => 'r)

    let run = (Cont(runCont)) => runCont

    let return = a => Cont(k => k(a))

    let bind = (m, f) => Cont(k => m->run(a => f(a)->run(k)))

    let callCC = f => Cont(outer => {
        let cc = x => Cont(_ => outer(x))
        f(cc)->run(outer)
    })
}

let id = x => x

Cont.return(3)
->Cont.bind(x => Cont.return(x + 4))
->Cont.run(id)
->Js.log  // 7

let addCont = (x: int, y: int) => Cont.return(x + y)
addCont(1, 2)
->Cont.bind(x =>
    (Cont.callCC (k =>
        k(10)
        ->Cont.bind(_ => addCont(3, 4))))
    ->Cont.bind(y => addCont(x, y)))
->Cont.run(id)
->Js.log  // not 10, but 13

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
