/** @type {import('tailwindcss').Config} */
module.exports = {
    daisyui: {
        themes: [
            {
                mytheme: {

                    "primary": "#1c1917",

                    "secondary": "#57534e",

                    "accent": "#4b5563",

                    "neutral": "#1c1917",

                    "base-100": "#e7e5e4",

                    "info": "#0ea5e9",

                    "success": "#16a34a",

                    "warning": "#eab308",

                    "error": "#ef4444",
                },
            },
        ],
    },
    content: ["./src/**/*.{js,ts,jsx,tsx}"],
    theme: {
        extend: {
            colors: {},
            fontFamily: { inter: ['Inter'], mon: ['Montserrat'], pop: ["Poppins"] },
        },
    },
    plugins: [require("daisyui")],
};
