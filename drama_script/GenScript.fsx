open System

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
        | CharList(map) ->
            let charId = Guid.NewGuid() |> string |> CharId 
            let charList = map |> Map.add charId (CharName name) |> CharList
            (charId, charList)

    interface IJsonSerializable with
        member this.ToJson() =
            match this with
            | CharList(map) ->
                map
                |> Map.fold
                    (fun acc (CharId(charId)) (CharName(name)) ->
                        let charObj =
                            Map.empty
                                .Add("guid", JsonValue.String charId)
                                .Add("name", JsonValue.String name)
                            |> JsonValue.Object
                        acc @ [charObj])
                    []
                |> JsonValue.Array

(*
module CharList =
    let inline addNewChar (name : string) (charList : CharList) : CharId * CharList =
        charList.AddChar(name)

    let inline merge (CharList(latter)) (CharList(former)) : CharList =
        latter
        |> Map.fold
            (fun acc charId charName -> Map.add charId charName acc)
            former
        |> CharList
*)

type Command =
    | Speak of speaker : CharId * message : string

type Script =
    private
    | Script of Command list

    static member Empty = Script []

    member inline this.Speak(charId : CharId, message : string) : Script =
         match this with
         | Script(cmds) -> Script(cmds @ [Speak(charId, message)])

    interface IJsonSerializable with
        member this.ToJson() =
            match this with
            | Script(cmds) ->
                cmds
                |> List.map
                    (fun (Speak(CharId(charId), message)) ->
                        Map.empty
                            .Add("speaker", JsonValue.String charId)
                            .Add("message", JsonValue.String message)
                        |> JsonValue.Object)
                |> JsonValue.Array

(*
module Script =
    let inline speak (charId : CharId) (message : string) (script : Script) : Script =
        script.Speak(charId, message)

    let inline append (Script(latter)) (Script(former)) =
        Script(former @ latter)
*)

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

    (*
    /// 与えられた初期状態を遷移させ、結果を返す
    let inline eval< ^a, ^b> (initialState : ^a) (State(f) : State< ^a, ^b>) : ^b =
        fst <| f initialState

    /// 与えられた初期状態を遷移させ、最終状態を返す
    let inline exec< ^a, ^b> (initialState : ^a) (State(f) : State< ^a, ^b>) : ^a =
        snd <| f initialState

    /// 現在の状態を結果として受け取る State
    let get = State(fun s -> (s, s))

    /// 指定した値で状態を更新する
    let inline put< ^a> (s : ^a) : State< ^a, unit> =
        State(fun _ -> ((), s))

    /// 指定した関数を適用して状態を更新する
    let inline modify< ^a> (f : ^a -> ^a) : State< ^a, unit> =
        State(fun s -> ((), f s))
    *)

type StateBuilder() =
    member _.Return(v) = State(fun s -> (v, s))
    member _.Bind(state, binder) =
        State.bind binder state

type ScriptBuilder() =
    member _.Yield(()) = Script []

    [<CustomOperation("speak")>]
    member _.Speak(script : Script, speaker : CharId, message : string) : Script =
        script.Speak(speaker, message)

let () =
    let withChars = StateBuilder()
    let script = ScriptBuilder()

    let addChar name : State<CharList, CharId> =
        State(fun (charList : CharList) -> charList.AddChar(name))

    let speak (charId : CharId) (message : string) : State<Script, unit> =
        State(fun (script : Script) -> ((), script.Speak(charId, message)))

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

    charList
    |> JsonValue.from
    |> string
    |> printfn "charList: %s"

    script
    |> JsonValue.from
    |> string
    |> printfn "script: %s"
