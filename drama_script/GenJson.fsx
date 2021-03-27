open System

/// 状態遷移を表す型
type State<'s, 't> = State of ('s -> 't * 's)

module State =
    /// 与えられた初期状態を遷移させ、結果と最終状態を返す
    let inline run (initialState : 'a) (State(f) : State<'a, 'b>) : 'b * 'a =
        f initialState

    let inline bind<'s, 'a, 'b> (binder : 'a -> State<'s, 'b>) (state : State<'s, 'a>) : State<'s, 'b> =
        State(fun s ->
            let result, state' = run s state
            binder result |> run state')

type StateBuilder() =
    member _.Return(v) = State(fun s -> (v, s))
    member _.Bind(state, binder) =
        State.bind binder state

[<RequireQualifiedAccess>]
type JsonValue =
    | String of string
    | Array of JsonValue list
    | Object of Map<string, JsonValue>

    static member private ToString(jsonValue : JsonValue) =
        match jsonValue with
        | String(str) ->
            sprintf "\"%s\"" str
        | Array(list) ->
            list
            |> List.map JsonValue.ToString
            |> String.concat ","
            |> sprintf "[%s]"
        | Object(map) ->
            map
            |> Map.map
                (fun key value ->
                    JsonValue.ToString value
                    |> sprintf "\"%s\":%s" key)
            |> Map.toList
            |> List.map snd
            |> String.concat ","
            |> sprintf "{%s}"

    override this.ToString() = JsonValue.ToString(this)

type IJsonSerializable =
    abstract ToJson: unit -> JsonValue

module JsonValue =
    let inline from< ^a when ^a :> IJsonSerializable> (a : ^a) : JsonValue =
        (a :> IJsonSerializable).ToJson()

type CharId = CharId of string

type CharName = CharName of string

type CharList =
    private
    | CharList of Map<CharId, CharName>

    static member Empty = CharList Map.empty

    member inline this.AddChar(name : string) : CharId * CharList =
        match this with
        | CharList(charList) ->
            let charId = Guid.NewGuid() |> string |> CharId 
            let charList = charList |> Map.add charId (CharName name) |> CharList
            (charId, charList)

    interface IJsonSerializable with
        member this.ToJson() =
            match this with
            | CharList(charList) ->
                let folder (jsonValues : JsonValue list) (CharId(charId)) (CharName(charName)) : JsonValue list =
                    let charObj =
                        [ "guid", JsonValue.String charId
                          "name", JsonValue.String charName ]
                        |> Map.ofList
                        |> JsonValue.Object
                    jsonValues @ [charObj]

                charList
                |> Map.fold folder []
                |> JsonValue.Array

type Command =
    | Speak of speaker : CharId * message : string

type Script =
    private
    | Script of Command list

    static member Empty = Script []

    member inline this.Speak(charId : CharId, message : string) : Script =
         match this with
         | Script(commands) -> Script(commands @ [Speak(charId, message)])

    interface IJsonSerializable with
        member this.ToJson() =
            match this with
            | Script(commands) ->
                let mapping (Speak(CharId(charId), message)) : JsonValue =
                    [ "speaker", JsonValue.String charId
                      "message", JsonValue.String message ]
                    |> Map.ofList
                    |> JsonValue.Object

                commands
                |> List.map mapping
                |> JsonValue.Array

type ScriptBuilder() =
    member _.Yield(()) = Script []

    [<CustomOperation("speak")>]
    member _.Speak(script : Script, speaker : CharId, message : string) : Script =
        script.Speak(speaker, message)

type Scene =
    { charList : CharList
      script : Script }

    interface IJsonSerializable with
        member this.ToJson() =
            [ "charList", JsonValue.from this.charList
              "script", JsonValue.from this.script ]
            |> Map.ofList
            |> JsonValue.Object

let () =
    let withChars = StateBuilder()
    let script = ScriptBuilder()

    let addChar name : State<CharList, CharId> =
        State(fun charList -> charList.AddChar(name))

    let speak (charId : CharId) (message : string) : State<Script, unit> =
        State(fun script -> ((), script.Speak(charId, message)))

    let (script, charList) =
        withChars {
            let! madoka = addChar "鹿目まどか"
            let! homura = addChar "暁美ほむら"
            return script {
                speak homura "鹿目まどか。あなたは、この世界が尊いと思う？欲望よりも秩序を大切にしてる？"
                speak madoka "それは…えっと、その…私は、尊いと思うよ。やっぱり、自分勝手にルールを破るのって、悪いことじゃないかな…"
            }
        }
        |> State.run CharList.Empty

    { charList = charList; script = script }
    |> JsonValue.from
    |> string
    |> printfn "%s"
