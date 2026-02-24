import init, * as wasm from "./dist/client.js";
import highlight from "./vendor/highlight/highlight.js";
import html from './vendor/highlight/languages/xml.js';
import './js/layouts/code-example.js';

await init();

let root_html = wasm.render_root();
document.getElementById('root').innerHTML = root_html;

highlight.registerLanguage('html', html);
highlight.highlightAll();
