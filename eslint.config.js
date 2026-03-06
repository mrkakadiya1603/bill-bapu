import js from "@eslint/js";
import tsPlugin from "@typescript-eslint/eslint-plugin";
import tsParser from "@typescript-eslint/parser";
import reactPlugin from "eslint-plugin-react";
import reactHooksPlugin from "eslint-plugin-react-hooks";

export default [
  js.configs.recommended,
  {
    ignores: [
      "**/node_modules/**",
      "**/dist/**",
      "**/build/**",
      "**/.next/**",
      "**/src-tauri/**",
      "**/generated/**",
      "**/target/**",
      "**/.turbo/**",
    ],
  },
  {
    files: ["**/*.{ts,tsx}"],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module",
        ecmaFeatures: { jsx: true },
      },
      globals: {
        // Browser globals
        document: "readonly",
        window: "readonly",
        navigator: "readonly",
        localStorage: "readonly",
        sessionStorage: "readonly",
        fetch: "readonly",
        URL: "readonly",
        URLSearchParams: "readonly",
        console: "readonly",
        setTimeout: "readonly",
        setInterval: "readonly",
        clearTimeout: "readonly",
        clearInterval: "readonly",
        requestAnimationFrame: "readonly",
        cancelAnimationFrame: "readonly",
        // DOM types (used as values in React.forwardRef generics)
        HTMLDivElement: "readonly",
        HTMLButtonElement: "readonly",
        HTMLInputElement: "readonly",
        HTMLTextAreaElement: "readonly",
        HTMLSelectElement: "readonly",
        HTMLFormElement: "readonly",
        HTMLAnchorElement: "readonly",
        HTMLSpanElement: "readonly",
        HTMLHeadingElement: "readonly",
        HTMLParagraphElement: "readonly",
        HTMLElement: "readonly",
        HTMLTableElement: "readonly",
        HTMLTableRowElement: "readonly",
        HTMLTableCellElement: "readonly",
        HTMLLabelElement: "readonly",
        HTMLTableSectionElement: "readonly",
        HTMLTableCaptionElement: "readonly",
        HTMLFieldSetElement: "readonly",
        HTMLParagraphElement: "readonly",
        Element: "readonly",
        Event: "readonly",
        MouseEvent: "readonly",
        KeyboardEvent: "readonly",
        FocusEvent: "readonly",
        MediaQueryListEvent: "readonly",
        MediaQueryList: "readonly",
        // Web APIs
        AbortController: "readonly",
        FormData: "readonly",
        Headers: "readonly",
        Request: "readonly",
        Response: "readonly",
        WebSocket: "readonly",
        EventSource: "readonly",
      },
    },
    plugins: {
      "@typescript-eslint": tsPlugin,
      react: reactPlugin,
      "react-hooks": reactHooksPlugin,
    },
    rules: {
      ...tsPlugin.configs.recommended.rules,
      ...reactHooksPlugin.configs.recommended.rules,
      "@typescript-eslint/no-unused-vars": [
        "warn",
        { argsIgnorePattern: "^_" },
      ],
      "@typescript-eslint/no-explicit-any": "warn",
      // Allow the const + type same-name pattern used for enum-like objects
      // e.g., export const UserRole = {...}; export type UserRole = ...;
      "no-redeclare": "off",
      "@typescript-eslint/no-redeclare": "off",
      "react/react-in-jsx-scope": "off",
      "no-console": ["warn", { allow: ["warn", "error"] }],
    },
    settings: {
      react: { version: "detect" },
    },
  },
  // Node.js environment for config files
  {
    files: [
      "*.config.{js,ts,mjs}",
      "**/*.config.{js,ts,mjs}",
      "**/postcss.config.*",
    ],
    languageOptions: {
      globals: {
        __dirname: "readonly",
        __filename: "readonly",
        process: "readonly",
        module: "readonly",
        require: "readonly",
        exports: "readonly",
        console: "readonly",
      },
    },
  },
];
