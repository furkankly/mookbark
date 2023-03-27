/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
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
