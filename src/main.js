import init, { main } from "./pkg/rewordle.js";

async function go() {
  await init();
  main();
}

go();
