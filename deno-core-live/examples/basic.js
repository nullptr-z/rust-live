
async function hello() {
  return new Promise((resolve, _reject) => {
    Deno.core.print("Hello Js\n");
    resolve("Hello Rust")
  });
}

// function hello() {
//   return "Hello Rust"
// }

hello();
