<script lang="ts">
	import DataTable, { Head, Body, Row, Cell } from '@smui/data-table';
	import type { Product } from './inventory.model';
	import { getAllProducts } from './inventory.service';

	let currentProductPage: Product[] = [];

	getAllProducts().then((items) => {
		currentProductPage = items;
	});
</script>

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
