/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",                  // Rust source files in the `src` directory
    "./index.html",                   // Main HTML entry point
    "./style/tailwind.css",           // Explicitly include the Tailwind CSS input file
    "./templates/**/*.html",          // Any custom HTML templates
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
