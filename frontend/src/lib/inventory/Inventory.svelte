<script lang="ts">
	import { onMount } from 'svelte';
	import DataTable, { Head, Body, Row, Cell } from '@smui/data-table';

	import HttpClient from '../httpclient/HttpClient.svelte';
	import type { Product } from './inventory.model';
	import { dev } from '$app/environment';
	import { MOCKDATA_INVENTORY } from '../../mockdata/inventory.mockdata';

	let currentProductPage: Product[] = [];
	let httpClient;
	onMount(() => {
		getAllProducts();
	});

	const getAllProducts = async () => {
		if (dev) {
			console.warn('Using mock data - you should not see this in production or there is an issue');
			currentProductPage = MOCKDATA_INVENTORY;
		} else {
			httpClient.request<Product[]>(`/ui-api/inventory`).then((result) => {
				if (result === null) currentProductPage = [];
				else currentProductPage = result;
			});
		}
	};
</script>

<HttpClient bind:this={httpClient}></HttpClient>

<h1>Inventory</h1>
<DataTable table$aria-label="Inventory" style="width: 100%;">
	<Head>
		<Row>
			<Cell>Id</Cell>
			<Cell>Name</Cell>
			<Cell numeric>Price</Cell>
		</Row>
	</Head>
	<Body>
		{#each currentProductPage as product (product.id)}
			<Row>
				<Cell>{product.id}</Cell>
				<Cell>{product.name}</Cell>
				<Cell numeric>{product.price}</Cell>
			</Row>
		{/each}
	</Body>
</DataTable>
