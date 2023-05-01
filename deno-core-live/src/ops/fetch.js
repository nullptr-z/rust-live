
// ((window) => {
//   async function fetch(args) {
//     const argsType = typeof args
//     if (argsType === "string") {
//       args = { url: args, method: "GET", headers: [], body: null };
//     } else if (argsType === "object") {
//       if (args.url) {
//         args.method = args.method || "GET"
//         args.headers = args.headers || []
//         args.body = args.body || null
//       } else {
//         throw new Error("Invalid arguments")
//       }
//     } else {
//       throw new Error("Invalid fetch args, should be string or object")
//     }
//     return await Deno.core.opAsync("op_fetch", args)
//   }

//   window.fetch = fetch
// })(this)

async function fetch(args) {
  const argsType = typeof args
  if (argsType === "string") {
    args = { url: args, method: "GET", headers: [], body: null };
  } else if (argsType === "object") {
    if (args.url) {
      args.method = args.method || "GET"
      args.headers = args.headers || []
      args.body = args.body || null
    } else {
      throw new Error("Invalid arguments")
    }
  } else {
    throw new Error("Invalid fetch args, should be string or object")
  }
  let res = await Deno.core.opAsync("op_fetch", args)
  // 添加response text()函数
  res.text = () => {
    let body = res.body
    if (!body) return null;
    return Deno.core.opSync("op_decode_utf8", body)
  }

  res.json = () => {
    let text = res.text()
    if (!text) return null;
    return JSON.parse(text)
  }

  return res
}
