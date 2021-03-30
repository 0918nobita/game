type config = {
    rules: array<string>
}

type engine

type lintMessage = {
    line: int,
    column: int,
    message: string
}

type lintResult = {
    messages: array<lintMessage>
}

@bs.module("textlint")
@bs.new
external makeEngineInner: config => engine = "TextLintEngine"

let makeEngine = (~rules: array<string>): engine => makeEngineInner({ rules: rules })

@bs.send
external executeOnText: (engine, string) => Js.Promise.t<array<lintResult>> = "executeOnText"
