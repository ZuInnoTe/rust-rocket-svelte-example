import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: [vitePreprocess(),  {
		name: "strip-announcer",
		markup: ({ content: code }) => {
			code = code.replace(
				/<div id="svelte-announcer" [\s\S]*?<\/div>/,
				'<svelte:component this={null} />'
			);

			return { code }
		}
	}],
	kit: {
		adapter: adapter(),
		paths: {
			base: '',
			relative: false
		}
	}
};

export default config;
