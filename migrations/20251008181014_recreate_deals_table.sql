-- Создаем тип enum для статусов сделок
CREATE TYPE deal_status AS ENUM ('draft', 'active', 'paid', 'cancelled', 'completed');

-- Создаем таблицу deals с FLOAT8
CREATE TABLE deals (
    id SERIAL PRIMARY KEY,
    client_id INTEGER NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    deal_amount FLOAT8 NOT NULL,
    commission_percent FLOAT8 NOT NULL,
    commission_amount FLOAT8 NOT NULL DEFAULT 0,
    tour_operator VARCHAR(200) NOT NULL,
    deal_date DATE NOT NULL DEFAULT CURRENT_DATE,
    payment_due_date DATE NOT NULL,
    status deal_status NOT NULL DEFAULT 'draft',
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Индексы
CREATE INDEX idx_deals_client_id ON deals(client_id);
CREATE INDEX idx_deals_status ON deals(status);
CREATE INDEX idx_deals_due_date ON deals(payment_due_date);