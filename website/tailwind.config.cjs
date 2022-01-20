const config = {
    mode: "jit",
    content: ["./src/**/*.{html,js,svelte,ts}"],
    theme: {
        extend: {
			colors: {
				green: {
					600: '#009444'
				},
				"mawoka": {
					100: "#8dc63f",
					200: "#39b54a",
					300: "#009444"
				}
			}
		},
    },
    // Only add this if you installed the TailwindCSS-plugins:
    plugins: [require('daisyui')],
}

module.exports = config