
function print(data) {
    Deno.core.print(`${data}\n`);

    // 如果是data类型 string直接打印，如果是object类型，转换成string打印
    // if (typeof data === "string") {
    //     Deno.core.print(data);
    // } else {
    //     Deno.core.print(JSON.stringify(data));
    // }
}

print("starting to fetch...\n\n");

let res = await fetch({ url: "https://dummyjson.com/products/1" })
print(`【 status 】==> ${res.status}`);
print(`【 headers 】==>\n ${JSON.stringify(res.headers)}\n`);
print(`【 body 】==>\n ${JSON.stringify(res.json(), null, 2)}\n`);
