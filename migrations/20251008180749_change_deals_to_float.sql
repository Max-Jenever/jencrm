-- Add migration script here
-- Меняем DECIMAL на FLOAT для совместимости с Rust f64
ALTER TABLE deals 
ALTER COLUMN deal_amount TYPE FLOAT8,
ALTER COLUMN commission_percent TYPE FLOAT8,
ALTER COLUMN commission_amount TYPE FLOAT8;