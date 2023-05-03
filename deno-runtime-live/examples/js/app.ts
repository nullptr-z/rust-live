/// 运行： deno run --allow-net app.ts


import * as Drash from "https://deno.land/x/drash@v2.6.0/mod.ts";

class HomeResource extends Drash.Resource {
  public paths = ["/"];

  public GET(_request: Drash.Request, response: Drash.Response): void {
    return response.json({
      hello: "world",
      time: new Date(),
    });
  }
}

let todo: any[] = []
class TodoResource extends Drash.Resource {
  public paths = ["/todo"];

  public GET(_request: Drash.Request, response: Drash.Response): void {
    return response.json({ todo });
  }

  public POST(request: Drash.Request, response: Drash.Response): void {

    console.log("%c 🎉 request:", "font-size:22px;background-color:rgb(205, 209, 211);color:#fff;", request);

    const todoItem: any = request.bodyAll()
    todo.push({ id: todo.length, ...todoItem })
    return response.json({ status: 200, todoLen: todo.length })
  }
}

// Create and run your server

const server = new Drash.Server({
  hostname: "0.0.0.0",
  port: 1447,
  protocol: "http",
  resources: [HomeResource, TodoResource],
});

server.run();

console.log(`Server running at ${server.address}.`);
