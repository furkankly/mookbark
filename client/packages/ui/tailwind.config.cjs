/** @type {import('tailwindcss').Config} */
module.exports = {
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
          from: { opacity: 0, transform: translateY(2) },
          to: { opacity: 1, transform: translateY(0) },
        },
        "slide-right-and-fade": {
          from: { opacity: 0, transform: translateX(-2) },
          to: { opacity: 1, transform: translateX(0) },
        },
        "slide-down-and-fade": {
          from: { opacity: 0, transform: translateY(-2) },
          to: { opacity: 1, transform: translateY(0) },
        },
        "slide-left-and-fade": {
          from: { opacity: 0, transform: translateX(2) },
          to: { opacity: 1, transform: translateX(0) },
        },
      },
      animation: {
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
        "slide-up-and-fade": "slide-up-and-fade 0.2 ease-out",
        "slide-right-and-fade": "slide-right-and-fade 0.2 ease-out",
        "slide-down-and-fade": "slide-down-and-fade 0.2 ease-out",
        "slide-left-and-fade": "slide-left-and-fade 0.2 ease-out",
      },
      backgroundImage: {
        "mookbark-pattern": "url('/favicon-32x32.png')",
      },
      fontFamily: {
        sans: ["KoHo"],
      },
    },
  },
  plugins: [],
};
