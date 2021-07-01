const colors = require('tailwindcss/colors')

const config = {
	mode: "jit",
	purge: [
		"./src/**/*.{html,js,svelte,ts}",
	],
	theme: {
		extend: {
			black: colors.black,
			gray: colors.trueGray,
		},
	},
	plugins: [],
};

module.exports = config;
