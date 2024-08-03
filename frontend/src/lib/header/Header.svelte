<script lang="ts">
	import { goto } from '$app/navigation';
	import TopAppBar, { Row, Section, Title, AutoAdjust } from '@smui/top-app-bar';
	import Fab, { Icon } from '@smui/fab';
	import IconButton from '@smui/icon-button';

	let topAppBar: TopAppBar;

	function navigate(route: string, replaceState: boolean) {
		goto(`/${route}`, { replaceState });
	}
</script>

<TopAppBar bind:this={topAppBar} variant="standard">
	<Row>
		<Section>
			<IconButton
				class="material-icons"
				aria-label="Inventory"
				on:click={() => navigate('inventory', false)}>inventory</IconButton
			>
			<IconButton
				class="material-icons"
				aria-label="Shopping Cart"
				on:click={() => navigate('order', false)}>shopping_cart</IconButton
			>
			<Title>MyApp</Title>
		</Section>
		<Section align="end" toolbar>
			<IconButton class="material-icons" aria-label="Settings">settings</IconButton>
		</Section>
	</Row>
</TopAppBar>
<AutoAdjust {topAppBar}></AutoAdjust>
