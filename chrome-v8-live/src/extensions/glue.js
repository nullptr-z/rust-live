"use strict";

({ print }) => {
  globalThis.print = (...args) => {
    console.log("【 args 】==>", args);
    print(Object(args))
  }
}
