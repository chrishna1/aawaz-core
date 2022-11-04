/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.{vue,js,ts,jsx,tsx,svelte}",
        "./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
    ],
    theme: {
        extend: {},
    },
    plugins: [require("flowbite/plugin")],
    darkMode: "class",
    variants: {
        extend: {
            outline: ["dark"],
        },
    },
};
