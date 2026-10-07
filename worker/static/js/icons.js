// Lucide icons (ISC), copied byte for byte to /ui/icons/ and inlined as SVG so they take
// currentColor and the 1.5 px stroke of DESIGN.md (Shapes). Parsed as XML, never as HTML.

export const ICONS = [
  "blocks", "check", "chevron-down", "chevron-up", "circle-check", "circle-dashed", "circle-x", "clock", "copy",
  "cpu", "equal", "equal-not", "external-link", "file-check", "file-x", "hourglass", "info", "key-round", "list",
  "loader-circle", "lock", "minus", "package-check", "plus", "refresh-cw", "repeat", "server", "terminal",
  "triangle-alert", "undo-2", "unlink", "x",
];

const cache = new Map();

export async function preloadIcons() {
  const parser = new DOMParser();
  await Promise.all(
    ICONS.map(async (name) => {
      try {
        const response = await fetch(`/ui/icons/${name}.svg`, { cache: "no-store", credentials: "omit" });
        if (!response.ok) return;
        const doc = parser.parseFromString(await response.text(), "image/svg+xml");
        const root = doc.documentElement;
        if (root && root.localName === "svg" && !doc.querySelector("parsererror, script, foreignObject")) cache.set(name, root);
      } catch {
        // A missing icon leaves its label alone; state is never carried by the icon only.
      }
    }),
  );
}

export function icon(name, size = "sm", extra = "") {
  const source = cache.get(name);
  const node = source ? document.importNode(source, true) : document.createElementNS("http://www.w3.org/2000/svg", "svg");
  node.removeAttribute("class");
  node.removeAttribute("width");
  node.removeAttribute("height");
  node.setAttribute("aria-hidden", "true");
  node.setAttribute("focusable", "false");
  node.classList.add("icon", `icon--${size}`);
  if (extra) node.classList.add(...extra.split(" ").filter(Boolean));
  return node;
}
