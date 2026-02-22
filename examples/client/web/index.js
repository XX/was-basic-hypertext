import init, * as wasm from "./dist/client.js";

await init();

let root_html = wasm.render_root();
document.getElementById('root').innerHTML = root_html;
