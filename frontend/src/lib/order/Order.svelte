<script lang="ts">
	import { onMount } from 'svelte';
	import HttpClient from '../httpclient/HttpClient.svelte';
	import DataTable, { Head, Body, Row, Cell } from '@smui/data-table';
	import { dev } from '$app/environment';
	import { MOCKDATA_ORDER } from '../../mockdata/order.mockdata';

	let currentOrderPage: Order[] = [];
	let httpClient;

	onMount(() => {
		getAllProducts();
	});

	const getAllProducts = async () => {
		if (dev) {
			console.warn('Using mock data - you should not see this in production or there is an issue');
			currentOrderPage = MOCKDATA_ORDER;
		} else {
			httpClient.request<Product[]>(`/ui-api/order`).then((result) => {
				if (result === null) currentOrderPage = [];
				else currentOrderPage = result;
			});
		}
	};
</script>

<HttpClient bind:this={httpClient}></HttpClient>

<h1>Order</h1>
<DataTable table$aria-label="Order" style="width: 100%;">
	<Head>
		<Row>
			<Cell>Id</Cell>
			<Cell>Order Date Time</Cell>
			<Cell>Product Id</Cell>
			<Cell>Product Name</Cell>
			<Cell>Product Price</Cell>
		</Row>
	</Head>
	<Body>
		{#each currentOrderPage as order (order.id)}
			<Row>
				<Cell>{order.id}</Cell>
				<Cell>{order.order_datetime}</Cell>
				<Cell>{order.product.id}</Cell>
				<Cell>{order.product.name}</Cell>
				<Cell numeric>{order.product.price}</Cell>
			</Row>
		{/each}
	</Body>
</DataTable>
