/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{vue,ts,js}"],
  theme: {
    extend: {},
  },
  plugins: [
    require('flowbite/plugin')
  ]
}

