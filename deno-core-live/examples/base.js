
Deno.core.print("Resolving module base.js\n\n");

export function print(str) {
  Deno.core.print(`base print: ${str}\n`);
}
