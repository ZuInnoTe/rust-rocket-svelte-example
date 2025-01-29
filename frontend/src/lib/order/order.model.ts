import type { Product } from '$lib/inventory/inventory.model';

export interface Order {
	id: string;
	order_datetime: Date;
	product: Product;
}
