import { print } from './base.js'

async function hello() {
  return new Promise((resolve) => {
    print("Hello world");
    resolve("Hello rust");
  });
}

await hello();
