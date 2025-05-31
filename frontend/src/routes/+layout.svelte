<script lang="ts">
	// include fonts
	import '@fontsource/material-icons';
	// include app assets
	import { assets } from '$app/paths';
	import Header from '../lib/header/Header.svelte';
	interface Props {
		children?: import('svelte').Snippet;
	}

	let { children }: Props = $props();
</script>

<svelte:head>
	<!-- SMUI Styles -->
	<link rel="stylesheet" href="{assets}/css/smui.css" />

	<!-- Site Styles -->
	<link rel="stylesheet" href="{assets}/css/site.css" />

	<!-- Site Styles -->
	<link rel="stylesheet" href="{assets}/css/site.css" media="(prefers-color-scheme: light)" />
	<link
		rel="stylesheet"
		href="{assets}/css/site-dark.css"
		media="screen and (prefers-color-scheme: dark)"
	/>
</svelte:head>

<Header />

{@render children?.()}
