-- Create orders table
CREATE TABLE IF NOT EXISTS welcomes (
    order_uid VARCHAR PRIMARY KEY,
    track_number VARCHAR NOT NULL,
    entry VARCHAR NOT NULL,
    locale VARCHAR NOT NULL,
    internal_signature VARCHAR NOT NULL,
    customer_id VARCHAR NOT NULL,
    delivery_service VARCHAR NOT NULL,
    shardkey VARCHAR NOT NULL,
    sm_id BIGINT NOT NULL,
    date_created TIMESTAMP NOT NULL,
    oof_shard VARCHAR NOT NULL
);

-- Create deliveries table
CREATE TABLE IF NOT EXISTS deliveries (
    order_uid VARCHAR PRIMARY KEY REFERENCES welcomes(order_uid),
    name VARCHAR NOT NULL,
    phone VARCHAR NOT NULL,
    zip VARCHAR NOT NULL,
    city VARCHAR NOT NULL,
    address VARCHAR NOT NULL,
    region VARCHAR NOT NULL,
    email VARCHAR NOT NULL
);

-- Create payments table
CREATE TABLE IF NOT EXISTS payments (
    order_uid VARCHAR PRIMARY KEY REFERENCES welcomes(order_uid),
    transaction VARCHAR NOT NULL,
    request_id VARCHAR NOT NULL,
    currency VARCHAR NOT NULL,
    provider VARCHAR NOT NULL,
    amount BIGINT NOT NULL,
    payment_dt BIGINT NOT NULL,
    bank VARCHAR NOT NULL,
    delivery_cost BIGINT NOT NULL,
    goods_total BIGINT NOT NULL,
    custom_fee BIGINT NOT NULL
);

-- Create items table
CREATE TABLE IF NOT EXISTS items (
    id SERIAL PRIMARY KEY,
    order_uid VARCHAR REFERENCES welcomes(order_uid),
    chrt_id BIGINT NOT NULL,
    track_number VARCHAR NOT NULL,
    price BIGINT NOT NULL,
    rid VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    sale BIGINT NOT NULL,
    size VARCHAR NOT NULL,
    total_price BIGINT NOT NULL,
    nm_id BIGINT NOT NULL,
    brand VARCHAR NOT NULL,
    status BIGINT NOT NULL
);


-- Create indexes
-- CREATE INDEX idx_orders_customer_id ON welcomes(customer_id);
-- CREATE INDEX idx_items_order_uid ON items(order_uid);