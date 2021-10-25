const colors = require('tailwindcss/colors')

module.exports = {
    purge: [],
    darkMode: false, // or 'media' or 'class'
    theme: {
        colors: {
            ...colors,
            accent: '#77D970',
            accent0: '#F8485E',
            accent1: '#FF9300',
            accent2: '#F7FD04',
        },
        extend: {},
    },
    variants: {
        extend: {},
    },
    plugins: [
        require("tailwind-scrollbar"),
    ],
}
