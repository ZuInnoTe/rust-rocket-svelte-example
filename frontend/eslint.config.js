import eslintPluginSvelte from 'eslint-plugin-svelte';
import * as svelteParser from 'svelte-eslint-parser';
import * as typescriptParser from '@typescript-eslint/parser';
import svelteConfig from './svelte.config.js';

export default [
	// add more generic rule sets here, such as:
	//js.configs.recommended,
	...eslintPluginSvelte.configs['flat/recommended'],
	{
		ignores: ['.svelte-kit/*', 'build/*', 'eslint.config.js', 'svelte.config.js']
	},
	{
		languageOptions: {
			parser: typescriptParser,
			parserOptions: {
				project: 'tsconfig.json',
				extraFileExtensions: ['.svelte'] // This is a required setting in `@typescript-eslint/parser` v4.24.0.
			}
		}
	},
	{
		files: ['**/*.svelte', '*.svelte'],
		languageOptions: {
			parser: svelteParser,
			parserOptions: {
				parser: typescriptParser,
				// Specify the `svelte.config.js`.
				svelteConfig
			}
		}
	}
];
