import print from "./print.js";

(
  async function hello() {
    try {
      const res = await fetch("https://dummyjson.com/products/1")
      print(`【 res 】==> `, res);
    } catch {
      throw Error("error")
    }
  }
)()
