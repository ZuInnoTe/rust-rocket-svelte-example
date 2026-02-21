import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: [
		vitePreprocess(),
		{
			name: 'strip-announcer', // only needed to avoid CSP with svelte-announcer: https://github.com/sveltejs/kit/issues/11993 . Once svelte-announcer does not use anymore inline-styles it can be removed
			markup: ({ content: code }) => {
				code = code.replace(
					/<div id="svelte-announcer" [\s\S]*?<\/div>/,
					'{@const Component = null}<Component />'
				);

				return { code };
			}
		}
	],
	kit: {
		adapter: adapter(),
		paths: {
			base: '',
			relative: false
		}
	}
};

export default config;
