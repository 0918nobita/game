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

type CharDecl =
    { id : string
      name : string }
    interface IJsonSerializer with
        member this.ToJson() =
            Map.empty
                .Add("id", JsonValue.String this.id)
                .Add("name", JsonValue.String this.name)
            |> JsonValue.Object

type Speak =
    { speaker : string
      content : string }
    interface IJsonSerializer with
        member this.ToJson() =
            Map.empty
                .Add("speaker", JsonValue.String this.speaker)
                .Add("content", JsonValue.String this.content)
            |> JsonValue.Object

type Scene =
    { formatVersion : string
      chars : CharDecl list
      script : Speak list }
    interface IJsonSerializer with
        member this.ToJson() =
            Map.empty
                .Add("formatVersion", JsonValue.String this.formatVersion)
                .Add("chars",
                    this.chars
                    |> List.map (fun c -> (c :> IJsonSerializer).ToJson())
                    |> JsonValue.Array)
                .Add("script",
                    this.script
                    |> List.map (fun cmd -> (cmd :> IJsonSerializer).ToJson())
                    |> JsonValue.Array)
            |> JsonValue.Object

let () =
    let madoka = string (Guid.NewGuid())
    let homura = string (Guid.NewGuid())
    let scene = {
        formatVersion = "1.0"
        chars = [
            { id = madoka; name = "鹿目まどか" }
            { id = homura; name = "暁美ほむら" }
        ]
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
