type engine

type lintMessage = {
    line: int,
    column: int,
    message: string
}

type lintResult = {
    messages: array<lintMessage>
}

let makeEngine: (~rules: array<string>) => engine

let executeOnText: (engine, string) => Js.Promise.t<array<lintResult>>
