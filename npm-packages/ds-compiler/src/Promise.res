let resolve = v => Js.Promise.resolve(v)

let then_ = (p, f) => Js.Promise.then_(f, p)

let catch = (p, f) => Js.Promise.catch(f, p)
