
async function hello() {
  return new Promise((resolve, _reject) => {
    Deno.core.print("Hello Js");
    resolve("Hello Rust")
  });
}

hello();
