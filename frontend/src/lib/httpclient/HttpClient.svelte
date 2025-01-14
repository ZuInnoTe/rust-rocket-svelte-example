<script lang="ts">
	/***
	  HttpClient for a SvelteKit application with an UI based on Svelte Material. Shows errors as a Snackbar and logs them to the console.
	**/
	import Snackbar, { Label, Actions } from '@smui/snackbar';

	let snackbarError: Snackbar;
	let httpErrorText: string = '';

	export async function request<T>(url: string, method: string): T {
		if (method === '') method = 'GET';
		try {
			let result = await fetch(url, {
				method: method
			}).then((response) => {
				if (!response.ok) {
					httpErrorText = 'HTTP error code' + response.status + ' Message' + response.statusText;
					console.error(httpErrorText);
					snackbarError.open();
					return null;
				}
				return response.json();
			});
			return result;
		} catch (e) {
			console.error(e);
			httpErrorText = 'Network error: ' + e;
			snackbarError.open();
		}
		return null;
	}
</script>

<Snackbar bind:this={snackbarError} labelText={httpErrorText} class="httpclient-error">
	<Label />
</Snackbar>
