import { dev } from '$app/environment';
import type { Product } from './inventory.model';
import { MOCKDATA_INVENTORY } from '../../mockdata/inventory.mockdata';

export const getAllProducts = async () => {
	let items: Product[] = [];
	if (dev) {
		console.warn('Using mock data - you should not see this in production or there is an issue');
		items = MOCKDATA_INVENTORY;
	} else {
		//const res = await fetch(`/api/items/${params.id}`);
		//const item = await res.json();
	}
	return items;
};
