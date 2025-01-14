import type { Product } from '$lib/inventory/inventory.model';

export interface Order {
	id: string;
	orderDateTime: Date;
	product: Product;
}
