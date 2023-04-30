
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
  return await Deno.core.opAsync("op_fetch", args)
}
