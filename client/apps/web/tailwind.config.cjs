const path = require("path");

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./**/index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
    path.join(path.dirname(require.resolve("ui")), "./**/*.{js,ts,jsx,tsx}"),
  ],
  theme: {
    extend: {
      keyframes: {
        "accordion-down": {
          from: { height: 0 },
          to: { height: "var(--radix-accordion-content-height)" },
        },
        "accordion-up": {
          from: { height: "var(--radix-accordion-content-height)" },
          to: { height: 0 },
        },
        "slide-up-and-fade": {
          from: { opacity: 0, transform: "translateY(2px)" },
          to: { opacity: 1, transform: "translateY(0)" },
        },
        "slide-right-and-fade": {
          from: { opacity: 0, transform: "translateX(-2px)" },
          to: { opacity: 1, transform: "translateX(0)" },
        },
        "slide-down-and-fade": {
          from: { opacity: 0, transform: "translateY(-2px)" },
          to: { opacity: 1, transform: "translateY(0)" },
        },
        "slide-left-and-fade": {
          from: { opacity: 0, transform: "translateX(2px)" },
          to: { opacity: 1, transform: "translateX(0)" },
        },
        "dialog-overlay-show": {
          from: { opacity: 0 },
          to: { opacity: 1 },
        },
        "dialog-content-show": {
          from: {
            opacity: 0,
            transform: "translate(-50%, -48%) scale(0.96)",
          },
          to: {
            opacity: 1,
            transform: "translate(-50%, -50%) scale(1)",
          },
        },
      },
      animation: {
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
        "slide-up-and-fade": "slide-up-and-fade 0.2s ease-out",
        "slide-right-and-fade": "slide-right-and-fade 0.2s ease-out",
        "slide-down-and-fade": "slide-down-and-fade 0.2s ease-out",
        "slide-left-and-fade": "slide-left-and-fade 0.2s ease-out",
        "dialog-overlay-show":
          "dialog-overlay-show 0.15s cubic-bezier(0.16, 1, 0.3, 1)",
        "dialog-content-show":
          "dialog-content-show 0.15s cubic-bezier(0.16, 1, 0.3, 1)",
      },
      backgroundImage: {
        "mookbark-pattern": "url('/favicon-32x32.png')",
      },
      fontFamily: {
        sans: ["KoHo"],
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
