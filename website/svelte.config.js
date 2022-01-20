import adapter from '@sveltejs/adapter-auto';
import preprocess from "svelte-preprocess"

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: [ // <- Create this option
		preprocess({ // <- Add the module
			postcss: true, // <- Set this to enable PostCSS
		}),
	],
	kit: {
		
		adapter: adapter(),

		// hydrate the <div id="svelte"> element in src/app.html
		target: '#svelte',

		// Override http methods in the Todo forms

	}
};

export default config;
