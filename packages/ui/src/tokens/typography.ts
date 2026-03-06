export const fontFamily = {
  body: ["Inter", "ui-sans-serif", "system-ui", "sans-serif"],
  display: ["Plus Jakarta Sans", "Inter", "ui-sans-serif", "system-ui", "sans-serif"],
  mono: ["JetBrains Mono", "ui-monospace", "monospace"],
} as const;

export const fontSize = {
  display: ["32px", { lineHeight: "40px", fontWeight: "700" }],
  h1: ["24px", { lineHeight: "32px", fontWeight: "600" }],
  h2: ["20px", { lineHeight: "28px", fontWeight: "600" }],
  h3: ["16px", { lineHeight: "24px", fontWeight: "600" }],
  body: ["14px", { lineHeight: "22px", fontWeight: "400" }],
  "body-sm": ["13px", { lineHeight: "20px", fontWeight: "400" }],
  caption: ["12px", { lineHeight: "18px", fontWeight: "400" }],
  overline: ["11px", { lineHeight: "16px", fontWeight: "600", letterSpacing: "0.05em" }],
} as const;
