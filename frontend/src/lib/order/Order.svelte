<script lang="ts">
	import DataTable, { Head, Body, Row, Cell } from '@smui/data-table';
</script>

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
		<Row>
			<Cell>tbd</Cell>
			<Cell>tbd</Cell>
			<Cell>tbd</Cell>
			<Cell>tbd</Cell>
			<Cell numeric>0.99</Cell>
		</Row>
	</Body>
</DataTable>
