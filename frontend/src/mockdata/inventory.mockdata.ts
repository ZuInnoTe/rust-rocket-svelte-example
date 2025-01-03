import type { Product } from '../lib/inventory/inventory.model';

export const MOCKDATA_INVENTORY: Product[] = [
	{
		id: 'mock_product_1',
		name: 'Mock Product 1',
		price: 11.99
	},
	{
		id: 'mock_product_2',
		name: 'Mock Product 2',
		price: 12.99
	}
];

/*

export const MOCKDATA_INVENTORY_PAGE: ProductPage = {
	content: MOCKDATA_INVENTORY,
	pageable: {
	  pageNumber: 0,
	  pageSize: 20,
	  sort: {
		sorted: false,
		unsorted: true,
		empty: true,
	  },
	  offset: 0,
	  paged: true,
	  unpaged: false,
	},
	totalPages: 1,
	totalElements: 2,
	last: true,
	size: 20,
	number: 0,
	sort: {
	  sorted: false,
	  unsorted: true,
	  empty: true,
	},
	numberOfElements: 2,
	first: true,
	empty: false,
  };
*/
