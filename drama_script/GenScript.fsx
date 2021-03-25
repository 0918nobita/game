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

type CharId = CharId of string

type CharName = CharName of string

type CharList =
    private
    | CharList of Map<CharId, CharName>

    static member Empty = CharList Map.empty

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
    let inline addNewChar (name : string) (CharList(map)) : CharList * CharId =
        let charId = Guid.NewGuid() |> string |> CharId 
        let charList = map |> Map.add charId (CharName name) |> CharList
        (charList, charId)

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
    let inline speak (charId : CharId) (message : string) (Script(cmds)) =
        Script(cmds @ [Speak(charId, message)])

    let inline append (Script(latter)) (Script(former)) =
        Script(former @ latter)

let () =
    let (charListA, madoka) = CharList.Empty |> CharList.addNewChar "鹿目まどか"
    let (charListB, homura) = CharList.Empty |> CharList.addNewChar "暁美ほむら"

    let charList =
        charListA
        |> CharList.merge charListB

    (charList :> IJsonSerializable).ToJson()
    |> string
    |> printfn "charList: %s"

    let script =
        Script.Empty
        |> Script.speak homura "鹿目まどか。あなたは、この世界が尊いと思う？欲望よりも秩序を大切にしてる？"
        |> Script.speak madoka "それは…えっと、その…私は、尊いと思うよ。やっぱり、自分勝手にルールを破るのって、悪いことじゃないかな…"

    (script :> IJsonSerializable).ToJson()
    |> string
    |> printfn "script: %s"
