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

(*
type Character(name : string) =
    member val Guid = string(Guid.NewGuid())
    member val Name = name with get, set

    interface IEquatable<Character> with
        member this.Equals(other) = this.Guid = other.Guid

type CharsList =
    | CharsList of Character list

    interface IJsonSerializer with
        member this.ToJson() =
            match this with
            | CharsList(chars) ->
                chars
                |> List.map (fun c ->
                    Map.empty
                        .Add("id", JsonValue.String c.Guid)
                        .Add("name", JsonValue.String c.Name)
                    |> JsonValue.Object)
                |> JsonValue.Array

type Speak =
    { Speaker : Character
      Content : string }

    interface IJsonSerializer with
        member this.ToJson() =
            Map.empty
                .Add("speaker", JsonValue.String this.Speaker.Guid)
                .Add("content", JsonValue.String this.Content)
            |> JsonValue.Object

type Scene =
    { FormatVersion : string
      Chars : CharsList
      Script : Speak list }

    interface IJsonSerializer with
        member this.ToJson() =
            Map.empty
                .Add("formatVersion", JsonValue.String this.FormatVersion)
                .Add("chars", (this.Chars :> IJsonSerializer).ToJson())
                .Add("script",
                    this.Script
                    |> List.map (fun cmd -> (cmd :> IJsonSerializer).ToJson())
                    |> JsonValue.Array)
            |> JsonValue.Object

type SceneBuilder() =
    member _.Yield(()) = {
        FormatVersion = "1.0"
        Chars = CharsList []
        Script = []
    }

    [<CustomOperation("addChar")>]
    member _.AddChar(scene : Scene, character : Character) =
        let chars =
            match scene.Chars with
            | CharsList(chars) -> chars
        let newChars = CharsList (chars @ [ character ])
        { scene with Chars = newChars }

    [<CustomOperation("speak")>]
    member _.Speak(scene : Scene, character : Character, content : string) =
        { scene with Script = scene.Script @ [{ Speaker = character; Content = content }] }

let () =
    let script = SceneBuilder()

    let scene =
        let homura = Character("暁美ほむら")
        let madoka = Character("鹿目まどか")
        script {
            addChar homura
            addChar madoka
            speak homura "鹿目まどか。あなたは、この世界が尊いと思う？欲望よりも秩序を大切にしてる？"
            speak madoka "それは…えっと、その…私は、尊いと思うよ。やっぱり、自分勝手にルールを破るのって、悪いことじゃないかな…"
        }

    (scene :> IJsonSerializer).ToJson()
    |> JsonValue.toString
    |> printfn "%s"
*)

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
    let addNewChar (name : string) (CharList(map)) : CharList =
        map
        |> Map.add (Guid.NewGuid() |> string |> CharId) (CharName name)
        |> CharList

    let merge (CharList(latter)) (CharList(former)) : CharList =
        latter
        |> Map.fold
            (fun acc charId charName -> Map.add charId charName acc)
            former
        |> CharList

let () =
    let charListA = CharList.Empty |> CharList.addNewChar "鹿目まどか"
    let charListB = CharList.Empty |> CharList.addNewChar "暁美ほむら"
    let charList =
        charListA
        |> CharList.merge charListB
    (charList :> IJsonSerializable).ToJson()
    |> string
    |> printfn "%A"
