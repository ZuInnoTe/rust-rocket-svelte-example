import type { Order } from '../lib/order/order.model';

import { MOCKDATA_INVENTORY } from './inventory.mockdata';

export const MOCKDATA_ORDER: Order[] = [
	{
		id: 'mock_inventory_1',
		order_datetime: new Date('2023-01-01T23:59:00'),
		product: MOCKDATA_INVENTORY[0]
	},
	{
		id: 'mock_inventory_2',
		order_datetime: new Date('2023-01-01T23:59:00'),
		product: MOCKDATA_INVENTORY[1]
	}
];
