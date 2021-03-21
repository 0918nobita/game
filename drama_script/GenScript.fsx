open System

[<RequireQualifiedAccess>]
type JsonValue =
    | String of string
    | Array of JsonValue list
    | Object of Map<string, JsonValue>

module JsonValue =
    let rec toString (jsonValue : JsonValue) : string =
        match jsonValue with
        | JsonValue.String(str) ->
            sprintf "\"%s\"" str
        | JsonValue.Array(list) ->
            list
            |> List.map toString
            |> String.concat ","
            |> sprintf "[%s]"
        | JsonValue.Object(map) ->
            map
            |> Map.map
                (fun key value ->
                    toString value
                    |> sprintf "\"%s\":%s" key)
            |> Map.toList
            |> List.map snd
            |> String.concat ","
            |> sprintf "{%s}"

type IJsonSerializer =
    abstract ToJson: unit -> JsonValue

type Character(name : string) =
    member val Guid = string(Guid.NewGuid())
    member val Name = name with get, set

    interface IEquatable<Character> with
        member this.Equals(other) = this.Guid = other.Guid

type CharsDecl =
    | CharsDecl of Character list

    interface IJsonSerializer with
        member this.ToJson() =
            match this with
            | CharsDecl(chars) ->
                chars
                |> List.map (fun c ->
                    Map.empty
                        .Add("id", JsonValue.String c.Guid)
                        .Add("name", JsonValue.String c.Name)
                    |> JsonValue.Object)
                |> JsonValue.Array

type Speak =
    { speaker : Character
      content : string }

    interface IJsonSerializer with
        member this.ToJson() =
            Map.empty
                .Add("speaker", JsonValue.String this.speaker.Guid)
                .Add("content", JsonValue.String this.content)
            |> JsonValue.Object

type Scene =
    { formatVersion : string
      chars : CharsDecl
      script : Speak list }

    interface IJsonSerializer with
        member this.ToJson() =
            Map.empty
                .Add("formatVersion", JsonValue.String this.formatVersion)
                .Add("chars", (this.chars :> IJsonSerializer).ToJson())
                .Add("script",
                    this.script
                    |> List.map (fun cmd -> (cmd :> IJsonSerializer).ToJson())
                    |> JsonValue.Array)
            |> JsonValue.Object

let () =
    let madoka = Character("鹿目まどか")
    let homura = Character("暁美ほむら")

    let scene = {
        formatVersion = "1.0"
        chars = CharsDecl [madoka; homura]
        script = [
            {
                speaker = homura
                content = "鹿目まどか。あなたは、この世界が尊いと思う？欲望よりも秩序を大切にしてる？"
            }
            {
                speaker = madoka
                content = "それは…えっと、その…私は、尊いと思うよ。やっぱり、自分勝手にルールを破るのって、悪いことじゃないかな…"
            }
        ]
    }

    (scene :> IJsonSerializer).ToJson()
    |> JsonValue.toString
    |> printfn "%s"
