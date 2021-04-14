module Program

let () =
    let element = Browser.Dom.document.createElement("p")
    element.innerText <- "Hello, world!"
    ignore <| Browser.Dom.document.body.appendChild(element)
