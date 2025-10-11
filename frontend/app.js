const API_BASE = 'http://localhost:3000/api';

// Утилиты
async function apiCall(endpoint, options = {}) {
    try {
        const response = await fetch(`${API_BASE}${endpoint}`, {
            headers: {
                'Content-Type': 'application/json',
                ...options.headers,
            },
            ...options
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        return await response.json();
    } catch (error) {
        console.error('API call failed:', error);
        alert('Ошибка: ' + error.message);
        throw error;
    }
}

// Управление вкладками
function showTab(tabName) {
    // Скрываем все вкладки
    document.querySelectorAll('.tab-content').forEach(tab => {
        tab.classList.remove('active');
    });
    document.querySelectorAll('.tab-button').forEach(button => {
        button.classList.remove('active');
    });

    // Показываем выбранную вкладку
    document.getElementById(tabName).classList.add('active');
    event.target.classList.add('active');

    // Загружаем данные для вкладки
    if (tabName === 'clients') {
        loadClients();
    } else if (tabName === 'deals') {
        loadDeals();
    }
}

// Клиенты
function showClientForm() {
    document.getElementById('clientForm').style.display = 'block';
}

function hideClientForm() {
    document.getElementById('clientForm').style.display = 'none';
    // Очищаем форму
    document.getElementById('clientForm').querySelectorAll('input').forEach(input => {
        input.value = '';
    });
}

async function createClient() {
    const clientData = {
        first_name: document.getElementById('clientFirstName').value,
        last_name: document.getElementById('clientLastName').value,
        email: document.getElementById('clientEmail').value || null,
        phone: document.getElementById('clientPhone').value || null
    };

    if (!clientData.first_name || !clientData.last_name) {
        alert('Имя и фамилия обязательны!');
        return;
    }

    try {
        await apiCall('/clients', {
            method: 'POST',
            body: JSON.stringify(clientData)
        });

        hideClientForm();
        loadClients();
        alert('Клиент создан!');
    } catch (error) {
        // Ошибка уже обработана в apiCall
    }
}

async function loadClients() {
    const container = document.getElementById('clientsList');
    container.innerHTML = '<div class="loading">Загрузка клиентов...</div>';

    try {
        const clients = await apiCall('/clients');

        if (clients.length === 0) {
            container.innerHTML = '<div class="loading">Клиентов пока нет</div>';
            return;
        }

        container.innerHTML = clients.map(client => `
            <div class="item">
                <div class="item-header">
                    <div class="item-title">${client.first_name} ${client.last_name}</div>
                </div>
                <div class="item-meta">
                    ${client.email ? `📧 ${client.email}` : ''}
                    ${client.phone ? `📞 ${client.phone}` : ''}
                    <br>
                    <small>Создан: ${new Date(client.created_at).toLocaleDateString('ru-RU')}</small>
                </div>
            </div>
        `).join('');
    } catch (error) {
        container.innerHTML = '<div class="loading">Ошибка загрузки клиентов</div>';
    }
}

// Сделки
function showDealForm() {
    loadClientsForDealForm();
    document.getElementById('dealForm').style.display = 'block';
}

function hideDealForm() {
    document.getElementById('dealForm').style.display = 'none';
    document.getElementById('dealForm').querySelectorAll('input, select, textarea').forEach(field => {
        field.value = '';
    });
}

async function loadClientsForDealForm() {
    const select = document.getElementById('dealClientId');
    select.innerHTML = '<option value="">Выберите клиента</option>';

    try {
        const clients = await apiCall('/clients');
        clients.forEach(client => {
            const option = document.createElement('option');
            option.value = client.id;
            option.textContent = `${client.first_name} ${client.last_name} (ID: ${client.id})`;
            select.appendChild(option);
        });
    } catch (error) {
        console.error('Failed to load clients for deal form');
    }
}

async function createDeal() {
    const dealData = {
        client_id: parseInt(document.getElementById('dealClientId').value),
        deal_amount: parseFloat(document.getElementById('dealAmount').value),
        commission_percent: parseFloat(document.getElementById('dealCommission').value),
        tour_operator: document.getElementById('dealOperator').value,
        deal_date: document.getElementById('dealDate').value,
        payment_due_date: document.getElementById('dealDueDate').value,
        description: document.getElementById('dealDescription').value || null
    };

    // Валидация
    if (!dealData.client_id || !dealData.deal_amount || !dealData.commission_percent ||
        !dealData.tour_operator || !dealData.deal_date || !dealData.payment_due_date) {
        alert('Заполните все обязательные поля!');
        return;
    }

    try {
        await apiCall('/deals', {
            method: 'POST',
            body: JSON.stringify(dealData)
        });

        hideDealForm();
        loadDeals();
        alert('Сделка создана!');
    } catch (error) {
        // Ошибка уже обработана в apiCall
    }
}

async function loadDeals() {
    const container = document.getElementById('dealsList');
    container.innerHTML = '<div class="loading">Загрузка сделок...</div>';

    try {
        const deals = await apiCall('/deals');

        if (deals.length === 0) {
            container.innerHTML = '<div class="loading">Сделок пока нет</div>';
            return;
        }

        container.innerHTML = deals.map(deal => `
            <div class="item">
                <div class="item-header">
                    <div class="item-title">Сделка #${deal.id}</div>
                    <div class="deal-amount">${deal.deal_amount.toLocaleString()} ₽</div>
                </div>
                <div class="item-meta">
                    <strong>Клиент ID:</strong> ${deal.client_id} |
                    <strong>Оператор:</strong> ${deal.tour_operator}<br>
                    <strong>Комиссия:</strong> <span class="deal-commission">${deal.commission_amount.toLocaleString()} ₽</span>
                    (${deal.commission_percent}%)<br>
                    <strong>Дата:</strong> ${deal.deal_date} |
                    <strong>Оплата до:</strong> ${deal.payment_due_date}<br>
                    <strong>Статус:</strong> ${getStatusText(deal.status)}<br>
                    ${deal.description ? `<strong>Описание:</strong> ${deal.description}<br>` : ''}
                    <small>Создана: ${new Date(deal.created_at).toLocaleDateString('ru-RU')}</small>
                </div>
            </div>
        `).join('');
    } catch (error) {
        container.innerHTML = '<div class="loading">Ошибка загрузки сделок</div>';
    }
}

function getStatusText(status) {
    const statusMap = {
        'draft': '📝 Черновик',
        'active': '🟢 Активна',
        'paid': '💰 Оплачена',
        'cancelled': '❌ Отменена',
        'completed': '✅ Завершена'
    };
    return statusMap[status] || status;
}

// Запускаем при загрузке страницы
document.addEventListener('DOMContentLoaded', function() {
    loadClients();
});
