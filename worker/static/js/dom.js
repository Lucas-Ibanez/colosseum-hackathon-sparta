// DOM helpers. Every text goes through textContent (C10-6); no HTML string is ever parsed.

const SVG_NS = "http://www.w3.org/2000/svg";

function addChildren(node, children) {
  for (const child of children.flat(Infinity)) {
    if (child === null || child === undefined || child === false) continue;
    node.append(child instanceof Node ? child : document.createTextNode(String(child)));
  }
}

// el("p", {class: "x", text: "...", attrs: {...}, on: {click}, data: {...}}, ...children)
export function el(tag, props = {}, ...children) {
  const node = document.createElement(tag);
  applyProps(node, props);
  addChildren(node, children);
  return node;
}

export function svg(tag, attrs = {}, ...children) {
  const node = document.createElementNS(SVG_NS, tag);
  for (const [name, value] of Object.entries(attrs)) node.setAttribute(name, String(value));
  addChildren(node, children);
  return node;
}

function applyProps(node, props) {
  for (const [key, value] of Object.entries(props || {})) {
    if (value === null || value === undefined || value === false) continue;
    if (key === "class") {
      const names = Array.isArray(value) ? value.filter(Boolean) : String(value).split(/\s+/).filter(Boolean);
      node.classList.add(...names);
    } else if (key === "text") {
      node.textContent = String(value);
    } else if (key === "attrs") {
      for (const [name, attr] of Object.entries(value)) {
        if (attr === null || attr === undefined || attr === false) continue;
        if (name === "style" || name.startsWith("on")) throw new Error(`attribute ${name} is not allowed`);
        node.setAttribute(name, attr === true ? "" : String(attr));
      }
    } else if (key === "on") {
      for (const [event, handler] of Object.entries(value)) node.addEventListener(event, handler);
    } else if (key === "data") {
      for (const [name, data] of Object.entries(value)) node.dataset[name] = String(data);
    } else {
      throw new Error(`unknown prop ${key}`);
    }
  }
}

export function clear(node) {
  while (node.firstChild) node.firstChild.remove();
  return node;
}

export function replace(node, ...children) {
  clear(node);
  addChildren(node, children);
  return node;
}

// Text with `code` spans: the dictionary marks identifiers with backticks.
export function rich(text, tag = "span", props = {}) {
  const node = el(tag, props);
  String(text)
    .split("`")
    .forEach((part, index) => {
      if (!part) return;
      node.append(index % 2 ? el("code", { class: "code-inline", text: part }) : document.createTextNode(part));
    });
  return node;
}

// Only links the worker produced for devnet Explorer pages are rendered.
export function explorerHref(url) {
  if (typeof url !== "string") return null;
  return /^https:\/\/explorer\.solana\.com\/(tx|address)\/[1-9A-HJ-NP-Za-km-z]{32,88}\?cluster=devnet$/.test(url) ? url : null;
}

export function visuallyHidden(text) {
  return el("span", { class: "visually-hidden", text });
}
