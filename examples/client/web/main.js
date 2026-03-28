import init, * as wasm from "./dist/client.js";
import highlight from "./vendor/highlight/highlight.js";
import html from './vendor/highlight/languages/xml.js';
import './js/layouts/code-example.js';
import './js/layouts/page.js';

await init();

let root_html = wasm.render_root();
let html_fragment = document.createRange().createContextualFragment(root_html);
let root = document.getElementById('root');
root.insertBefore(html_fragment, root.firstChild);

highlight.registerLanguage('html', html);
highlight.highlightAll();
