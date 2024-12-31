/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'media',
  theme: {
    extend: {
      colors: {
        theme: {
          primary: '#fff',
          secondary: '#e9e9e9',
          accent: '#dedede',
          'text-primary': '#0f0f0f',
        },
        dark: {
          primary: '#1b1b1b',
          secondary: '#242424',
          accent: '#3d3d3d',
          'text-primary': '#f6f6f6',
        },
      },
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}