-- order
CREATE TABLE 'order' (
    id TEXT PRIMARY KEY,
    order_datetime Timestamptz,
    product_id TEXT NOT NULL
);
