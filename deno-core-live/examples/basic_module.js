async function hello() {
  return new Promise((resolve, reject) => {
    Deno.core.print("Hello world");
    resolve("Hello rust");
  });
}

await hello();
