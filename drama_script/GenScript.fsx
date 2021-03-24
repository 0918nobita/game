open System

[<RequireQualifiedAccess>]
type JsonValue =
    | Object of Map<string, JsonValue>
    | String of string

module JsonValue =
    let rec toString (jsonValue : JsonValue) : string =
        match jsonValue with
        | JsonValue.String(str) ->
            sprintf "\"%s\"" str
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

type CharDecl = {
    id : string
    name : string
}

type Speak = {
    speaker : string
    content : string
}

type Scene = {
    formatVersion : string
    chars : CharDecl list
    script : Speak list
}

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
    printfn "%A" scene
    Map.empty
        .Add("foo", JsonValue.String "bar")
        .Add("hoge", JsonValue.String "fuga")
    |> JsonValue.Object
    |> JsonValue.toString
    |> printfn "%s"
