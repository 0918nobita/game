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
    let inline from< ^a when ^a :> IJsonSerializable > (a : ^a) : JsonValue =
        (a :> IJsonSerializable).ToJson()

type CharId = CharId of string

type CharName = CharName of string

type CharList =
    private
    | CharList of Map<CharId, CharName>

    static member Empty = CharList Map.empty

    member inline this.AddNewChar(name : string) : CharList * CharId =
        match this with
        | CharList(map) ->
            let charId = Guid.NewGuid() |> string |> CharId 
            let charList = map |> Map.add charId (CharName name) |> CharList
            (charList, charId)

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

module CharList =
    let inline addNewChar (name : string) (charList : CharList) : CharList * CharId =
        charList.AddNewChar(name)

    let inline merge (CharList(latter)) (CharList(former)) : CharList =
        latter
        |> Map.fold
            (fun acc charId charName -> Map.add charId charName acc)
            former
        |> CharList

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

module Script =
    let inline speak (charId : CharId) (message : string) (script : Script) : Script =
        script.Speak(charId, message)

    let inline append (Script(latter)) (Script(former)) =
        Script(former @ latter)

let () =
    let (charListA, madoka) = CharList.Empty.AddNewChar("鹿目まどか")
    let (charListB, homura) = charListA.AddNewChar("暁美ほむら")

    charListB
    |> JsonValue.from
    |> string
    |> printfn "charList: %s"

    let script =
        Script.Empty
            .Speak(homura, "鹿目まどか。あなたは、この世界が尊いと思う？欲望よりも秩序を大切にしてる？")
            .Speak(madoka, "それは…えっと、その…私は、尊いと思うよ。やっぱり、自分勝手にルールを破るのって、悪いことじゃないかな…")

    script
    |> JsonValue.from
    |> string
    |> printfn "script: %s"
