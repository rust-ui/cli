/** @type {import('tailwindcss').Config} */
export default {
	darkMode: "class",
	content: {
		files: ["./src/**/*.rs"],
	},
	theme: {
		container: {
			center: true,
			padding: "2rem",
			screens: {
				"2xl": "1400px",
			},
		},
		extend: {
			keyframes: {},
			animation: {},
		},
	}
};