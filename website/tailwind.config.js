/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './src/**/*.rs',
    './index.html',
    './src/**/*.html',
    './src/**/*.css'
  ],
  theme: {
    extend: {},
  },
  plugins: [require('@tailwindcss/aspect-ratio')],
}