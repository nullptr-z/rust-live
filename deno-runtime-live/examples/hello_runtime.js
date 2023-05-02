(
  async function hello() {
    try {
      const res = await fetch("https://dummyjson.com/products/1")
      console.log("【 res 】==>", res);
    } catch {
      throw Error("error")
    }
  }
)()
