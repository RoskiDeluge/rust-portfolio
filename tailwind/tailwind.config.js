// Manually disabled code block CSS for tailwind
// because we wish to style this using some custom css
// that works on markdown output without the tailwind
// typography plugin. Reference for disabling:
// https://building.hellonext.co/blog/disable-tailwind-prose-code
const disabledCss = {
  "code::before": false,
  "code::after": false,
  "blockquote p:first-of-type::before": false,
  "blockquote p:last-of-type::after": false,
  pre: false,
  code: false,
  "pre code": false,
  "code::before": false,
  "code::after": false,
};

/** @type {import('tailwindcss').Config} */

module.exports = {
  content: ["../templates/**/*.{html,js}"],
  theme: {
    extend: {},
  },
  plugins: [],
};
