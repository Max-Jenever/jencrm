-- Add migration script here
-- Сначала создаем функцию, если её нет
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Создаем тип enum для статусов сделок
CREATE TYPE deal_status AS ENUM ('draft', 'active', 'paid', 'cancelled', 'completed');

-- Создаем таблицу deals
CREATE TABLE deals (
    id SERIAL PRIMARY KEY,
    client_id INTEGER NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    deal_amount DECIMAL(10, 2) NOT NULL,
    commission_percent DECIMAL(5, 2) NOT NULL,
    commission_amount DECIMAL(10, 2) NOT NULL,
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

-- Триггер для обновления updated_at
CREATE TRIGGER update_deals_updated_at
    BEFORE UPDATE ON deals
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Функция для расчета commission_amount
CREATE OR REPLACE FUNCTION calculate_commission_amount()
RETURNS TRIGGER AS $$
BEGIN
    NEW.commission_amount = (NEW.deal_amount * NEW.commission_percent) / 100;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Триггер для расчета commission_amount
CREATE TRIGGER calculate_deal_commission
    BEFORE INSERT OR UPDATE ON deals
    FOR EACH ROW
    EXECUTE FUNCTION calculate_commission_amount();
