/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.{vue,js,ts,jsx,tsx,svelte}"],
    theme: {
        extend: {},
    },
    plugins: [],
    darkMode: "class",
    variants: {
        extend: {
            outline: ["dark"],
        },
    },
};
