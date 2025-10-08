# JenCRM 🚀

**Быстрая и модульная CRM система для турагентств**  
*Open-source решение на Rust + Postgres*

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-15-blue.svg)

## 📖 О проекте

JenCRM - это современная CRM система, разработанная специально для турагентств. Проект создается с акцентом на:
- **Производительность** - Rust + Postgres для максимальной скорости
- **Модульность** - гибкая архитектура для легкого расширения
- **Простота** - чистый код и минимальные зависимости
- **Open Source** - свободное использование и развитие сообществом

## 🏗️ Текущий статус

**✅ Реализовано:**
- Бэкенд на Rust с фреймворком Axum
- PostgreSQL в Docker контейнере
- Модуль клиентов (полный CRUD API)
- Миграции базы данных
- Асинхронная архитектура

**🔧 Базовое API:**
- GET    /api/clients       - Получить всех клиентов
- POST   /api/clients       - Создать клиента
- GET    /api/clients/{id}  - Получить клиента по ID  
- PUT    /api/clients/{id}  - Обновить клиента
- DELETE /api/clients/{id}  - Удалить клиента

## 🚀 Быстрый старт

### Предварительные требования
- Rust 1.70+
- Docker & Docker Compose
- PostgreSQL 15
Отличная идея! Вот готовый патчноут для версии 0.1.1:

## 🚀 Release Notes v0.1.1

### 📦 Что нового:

**✅ Добавлен модуль сделок (Deals)**
- Полный CRUD API для управления сделками
- Автоматический расчет комиссии на стороне Rust
- Статусы сделок: Draft, Active, Paid, Cancelled, Completed  
- Связь с клиентами через foreign keys

**🔧 Исправления:**
- Исправлены типы данных для денежных значений (DECIMAL → FLOAT8)
- Улучшена обработка ошибок
- Убраны зависимости от триггеров БД

### 📋 API Endpoints:

#### Клиенты
```bash
# Получить всех клиентов
curl http://localhost:3000/api/clients

# Создать клиента
curl -X POST http://localhost:3000/api/clients \
  -H "Content-Type: application/json" \
  -d '{"first_name":"Иван","last_name":"Петров","email":"test@example.com"}'

# Получить клиента по ID
curl http://localhost:3000/api/clients/1

# Обновить клиента
curl -X PUT http://localhost:3000/api/clients/1 \
  -H "Content-Type: application/json" \
  -d '{"first_name":"НовоеИмя","last_name":"НоваяФамилия"}'

# Удалить клиента  
curl -X DELETE http://localhost:3000/api/clients/1
```

#### Сделки
```bash
# Получить все сделки
curl http://localhost:3000/api/deals

# Создать сделку (комиссия рассчитывается автоматически)
curl -X POST http://localhost:3000/api/deals \
  -H "Content-Type: application/json" \
  -d '{
    "client_id": 1,
    "deal_amount": 50000.00,
    "commission_percent": 15.5,
    "tour_operator": "TUI", 
    "deal_date": "2024-01-15",
    "payment_due_date": "2024-02-01",
    "description": "Тур в Турцию на 10 дней"
  }'

# Получить сделку по ID
curl http://localhost:3000/api/deals/1
```

### 🛠️ Быстрый старт:

```bash
# 1. Запуск БД
docker start jen-crm-postgres

# 2. Запуск сервера  
cd jencrm
cargo run

# 3. Проверка здоровья
curl http://localhost:3000/health
```

### 📊 Пример расчета комиссии:
При создании сделки с `deal_amount: 50000.00` и `commission_percent: 15.5`  
**Автоматически рассчитывается:** `commission_amount: 7750.00`

### 🐛 Известные issues:
- Пока нет валидации данных на стороне API
- Нет фронтенд интерфейса
- Нет аутентификации

---

**Следующие цели:** Модуль туров, базовый фронтенд, аутентификация

⭐ **Если проект полезен - поставьте звезду!**

---

Этот патчноут показывает:
- Что сделано
- Как использовать  
- Что работает
- Что планируется

## 🗺️ Дорожная карта

### 🎯 Ближайшие цели (v0.2.0)
- [ ] Модуль туров (tours) - каталог туров и направлений
- [ ] Модуль бронирований (bookings) - связь клиентов и туров
- [ ] Базовая аутентификация
- [ ] Простой веб-интерфейс

### 🚀 Планы на v0.5.0  
- [ ] Модуль задач (tasks) - напоминания для менеджеров
- [ ] Модуль отчетов (reports) - аналитика и статистика
- [ ] Расширенный веб-интерфейс
- [ ] Docker-сборка для продакшена

### 🌟 Долгосрочное видение (v1.0.0)
- [ ] Полнофункциональная CRM система
- [ ] PWA приложение
- [ ] REST API документация
- [ ] Плагинная архитектура

