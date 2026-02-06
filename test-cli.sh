#!/bin/bash

# Цвета для вывода
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Функции для вывода
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Проверка наличия CLI
check_cli() {
    if ! command -v target/debug/blog-cli &> /dev/null; then
        log_error "blog-cli не найден. Убедитесь, что CLI установлен и доступен в PATH."
        exit 1
    fi

    log_info "Проверка версии CLI..."
    blog-cli --version || blog-cli version || log_warning "Не удалось получить версию CLI"
    echo ""
}

# Генерация случайных данных
generate_random_data() {
    local timestamp=$(date +%s)
    local random_id=$((RANDOM % 1000))
    echo "${timestamp}_${random_id}"
}

# Очистка тестовых данных
cleanup() {
    log_info "Очистка тестовых данных..."
    # Удаляем тестовые посты, если они существуют
    if [ -n "$POST_ID" ]; then
        log_info "Удаление тестового поста ID: $POST_ID"
        blog-cli delete --id "$POST_ID" 2>/dev/null || true
        blog-cli delete --id "$POST_ID" --grpc 2>/dev/null || true
    fi
    log_success "Очистка завершена"
}

# Ловим сигналы для корректной очистки
trap cleanup EXIT INT TERM

# Настройки теста
TEST_USERNAME="testuser_$(generate_random_data)"
TEST_EMAIL="test_$(generate_random_data)@example.com"
TEST_PASSWORD="testpass123"
TEST_TITLE="Тестовый пост $(generate_random_data)"
TEST_CONTENT="Это тестовое содержимое поста, созданное автоматически."

# Переменные для хранения ID постов
POST_ID=""
POST_ID_GRPC=""

# Функции тестирования
test_http() {
    log_info "=== ТЕСТИРОВАНИЕ HTTP API ==="

    # 1. Регистрация
    log_info "1. Тестирование регистрации"
    if blog-cli register --username "$TEST_USERNAME" --email "$TEST_EMAIL" --password "$TEST_PASSWORD" | grep  -q"$TEST_USERNAME"; then
        log_success "Регистрация успешна"
    else
        log_error "Ошибка регистрации"
        return 1
    fi
    echo ""

    # 2. Вход
    log_info "2. Тестирование входа"
    if blog-cli login --username "$TEST_USERNAME" --password "$TEST_PASSWORD" | grep -q "$TEST_USERNAME"; then
        log_success "Вход успешен"
    else
        log_error "Ошибка входа"
        return 1
    fi
    echo ""

    # 3. Создание поста
    log_info "3. Тестирование создания поста (HTTP)"
    local http_output
    if http_output=$(blog-cli create --title "$TEST_TITLE" --content "$TEST_CONTENT" 2>&1); then
        log_success "Создание поста успешно"
        # Пытаемся извлечь ID поста из вывода
        POST_ID=$(echo "$http_output" | grep -oE 'ID: [0-9]+' | head -1 | awk '{print $2}' || echo "1")
        log_info "Создан пост с ID: $POST_ID"
    else
        log_error "Ошибка создания поста: $http_output"
        return 1
    fi
    echo ""

    # 4. Получение поста
    log_info "4. Тестирование получения поста (HTTP)"
    if blog-cli get --id "$POST_ID"; then
        log_success "Получение поста успешно"
    else
        log_error "Ошибка получения поста"
        return 1
    fi
    echo ""

    # 5. Обновление поста
    log_info "5. Тестирование обновления поста (HTTP)"
    local new_title="Обновлённый заголовок $(generate_random_data)"
    if blog-cli update --id "$POST_ID" --title "$new_title"; then
        log_success "Обновление поста успешно"
    else
        log_error "Ошибка обновления поста"
        return 1
    fi
    echo ""

    # 6. Проверка обновления
    log_info "6. Проверка обновления поста (HTTP)"
    if blog-cli get --id "$POST_ID" | grep -q "$new_title"; then
        log_success "Пост успешно обновлён"
    else
        log_error "Пост не был обновлён"
        return 1
    fi
    echo ""

    # 7. Список постов
    log_info "7. Тестирование списка постов (HTTP)"
    if blog-cli list --limit 10 --offset 0; then
        log_success "Получение списка успешно"
    else
        log_error "Ошибка получения списка"
        return 1
    fi
    echo ""

    # 8. Удаление поста
    log_info "8. Тестирование удаления поста (HTTP)"
    if blog-cli delete --id "$POST_ID"; then
        log_success "Удаление поста успешно"
        POST_ID=""  # Очищаем ID, так как пост удалён
    else
        log_error "Ошибка удаления поста"
        return 1
    fi
    echo ""

    log_success "=== HTTP ТЕСТЫ ПРОЙДЕНЫ УСПЕШНО ==="
    return 0
}

test_grpc() {
    log_info "=== ТЕСТИРОВАНИЕ gRPC API ==="

    # Предполагаем, что аутентификация уже выполнена через HTTP

    # 1. Создание поста через gRPC
    log_info "1. Тестирование создания поста (gRPC)"
    local grpc_output
    local grpc_title="gRPC Тестовый пост $(generate_random_data)"

    if grpc_output=$(blog-cli create --title "$grpc_title" --content "$TEST_CONTENT" --grpc 2>&1); then
        log_success "Создание поста через gRPC успешно"
        # Пытаемся извлечь ID поста из вывода
        POST_ID_GRPC=$(echo "$grpc_output" | grep -oE 'ID: [0-9]+' | head -1 | awk '{print $2}' || echo "2")
        log_info "Создан пост через gRPC с ID: $POST_ID_GRPC"
    else
        log_error "Ошибка создания поста через gRPC: $grpc_output"
        return 1
    fi
    echo ""

    # 2. Получение поста через gRPC
    log_info "2. Тестирование получения поста (gRPC)"
    if blog-cli get --id "$POST_ID_GRPC" --grpc; then
        log_success "Получение поста через gRPC успешно"
    else
        log_error "Ошибка получения поста через gRPC"
        return 1
    fi
    echo ""

    # 3. Обновление поста через gRPC
    log_info "3. Тестирование обновления поста (gRPC)"
    local grpc_new_title="Обновлённый gRPC заголовок $(generate_random_data)"
    if blog-cli update --id "$POST_ID_GRPC" --title "$grpc_new_title" --grpc; then
        log_success "Обновление поста через gRPC успешно"
    else
        log_error "Ошибка обновления поста через gRPC"
        return 1
    fi
    echo ""

    # 4. Проверка обновления через gRPC
    log_info "4. Проверка обновления поста (gRPC)"
    if blog-cli get --id "$POST_ID_GRPC" --grpc | grep -q "$grpc_new_title"; then
        log_success "Пост через gRPC успешно обновлён"
    else
        log_error "Пост через gRPC не был обновлён"
        return 1
    fi
    echo ""

    # 5. Список постов через gRPC
    log_info "5. Тестирование списка постов (gRPC)"
    if blog-cli list --limit 5 --offset 0 --grpc; then
        log_success "Получение списка через gRPC успешно"
    else
        log_error "Ошибка получения списка через gRPC"
        return 1
    fi
    echo ""

    # 6. Удаление поста через gRPC
    log_info "6. Тестирование удаления поста (gRPC)"
    if blog-cli delete --id "$POST_ID_GRPC" --grpc; then
        log_success "Удаление поста через gRPC успешно"
        POST_ID_GRPC=""  # Очищаем ID
    else
        log_error "Ошибка удаления поста через gRPC"
        return 1
    fi
    echo ""

    log_success "=== gRPC ТЕСТЫ ПРОЙДЕНЫ УСПЕШНО ==="
    return 0
}

test_error_cases() {
    log_info "=== ТЕСТИРОВАНИЕ ОШИБОЧНЫХ СЦЕНАРИЕВ ==="

    # 1. Регистрация с существующим пользователем
    log_info "1. Регистрация с существующим пользователем (должна вернуть ошибку)"
    if blog-cli register --username "$TEST_USERNAME" --email "$TEST_EMAIL" --password "$TEST_PASSWORD" 2>/dev/null; then
        log_error "Ожидалась ошибка при регистрации существующего пользователя"
        return 1
    else
        log_success "Правильно возвращена ошибка при дублировании пользователя"
    fi
    echo ""

    # 2. Вход с неверным паролем
    log_info "2. Вход с неверным паролем (должен вернуть ошибку)"
    if blog-cli login --username "$TEST_USERNAME" --password "wrongpassword" 2>/dev/null; then
        log_error "Ожидалась ошибка при входе с неверным паролем"
        return 1
    else
        log_success "Правильно возвращена ошибка при неверном пароле"
    fi
    echo ""

    # 3. Получение несуществующего поста
    log_info "3. Получение несуществующего поста (должен вернуть ошибку)"
    if blog-cli get --id 999999 2>/dev/null; then
        log_error "Ожидалась ошибка при получении несуществующего поста"
        return 1
    else
        log_success "Правильно возвращена ошибка при получении несуществующего поста"
    fi
    echo ""

    # 4. Создание поста без заголовка
    log_info "4. Создание поста без заголовка (должен вернуть ошибку)"
    if blog-cli create --content "Только контент" 2>/dev/null; then
        log_error "Ожидалась ошибка при создании поста без заголовка"
        return 1
    else
        log_success "Правильно возвращена ошибка при создании поста без заголовка"
    fi
    echo ""

    log_success "=== ОШИБОЧНЫЕ СЦЕНАРИИ ОБРАБОТАНЫ КОРРЕКТНО ==="
    return 0
}

run_performance_test() {
    log_info "=== ТЕСТ ПРОИЗВОДИТЕЛЬНОСТИ ==="

    local iterations=3
    local http_times=()
    local grpc_times=()

    # Тест HTTP
    log_info "Тестирование HTTP (итераций: $iterations)"
    for ((i=1; i<=iterations; i++)); do
        local start_time=$(date +%s%N)
        blog-cli list --limit 5 --offset 0 2>/dev/null
        local end_time=$(date +%s%N)
        local duration=$(( (end_time - start_time) / 1000000 ))
        http_times+=($duration)
        log_info "HTTP итерация $i: ${duration}мс"
    done

    # Тест gRPC
    log_info "Тестирование gRPC (итераций: $iterations)"
    for ((i=1; i<=iterations; i++)); do
        local start_time=$(date +%s%N)
        blog-cli list --limit 5 --offset 0 --grpc 2>/dev/null
        local end_time=$(date +%s%N)
        local duration=$(( (end_time - start_time) / 1000000 ))
        grpc_times+=($duration)
        log_info "gRPC итерация $i: ${duration}мс"
    done

    # Расчет средних значений
    local http_sum=0
    local grpc_sum=0

    for time in "${http_times[@]}"; do
        http_sum=$((http_sum + time))
    done

    for time in "${grpc_times[@]}"; do
        grpc_sum=$((grpc_sum + time))
    done

    local http_avg=$((http_sum / iterations))
    local grpc_avg=$((grpc_sum / iterations))

    log_info "Среднее время HTTP: ${http_avg}мс"
    log_info "Среднее время gRPC: ${grpc_avg}мс"

    if [ "$grpc_avg" -lt "$http_avg" ]; then
        log_success "gRPC быстрее HTTP на $((http_avg - grpc_avg))мс"
    else
        log_warning "HTTP быстрее gRPC на $((grpc_avg - http_avg))мс"
    fi
    echo ""
}

# Главная функция
main() {
    log_info "Начало тестирования CLI..."
    log_info "Тестовый пользователь: $TEST_USERNAME"
    log_info "Тестовый email: $TEST_EMAIL"
    log_info "Тестовый заголовок: $TEST_TITLE"
    echo ""

    # Проверка CLI
    check_cli

    # Запуск тестов
    local passed_tests=0
    local total_tests=4

    # Тест HTTP
    if test_http; then
        log_success "✓ HTTP тест пройден"
        passed_tests=$((passed_tests + 1))
    else
        log_error "✗ HTTP тест провален"
    fi
    echo ""

    # Тест gRPC
    if test_grpc; then
        log_success "✓ gRPC тест пройден"
        passed_tests=$((passed_tests + 1))
    else
        log_error "✗ gRPC тест провален"
    fi
    echo ""

    # Тест ошибочных сценариев
    if test_error_cases; then
        log_success "✓ Тест ошибок пройден"
        passed_tests=$((passed_tests + 1))
    else
        log_error "✗ Тест ошибок провален"
    fi
    echo ""

    # Опционально: тест производительности
    log_info "Запустить тест производительности? (y/n)"
    read -r answer
    if [[ "$answer" =~ ^[Yy]$ ]]; then
        if run_performance_test; then
            log_success "✓ Тест производительности завершен"
            passed_tests=$((passed_tests + 1))
        fi
    else
        log_info "Тест производительности пропущен"
        total_tests=$((total_tests - 1))
    fi
    echo ""

    # Итоги
    log_info "=== ИТОГИ ТЕСТИРОВАНИЯ ==="
    log_info "Пройдено тестов: $passed_tests из $total_tests"

    if [ "$passed_tests" -eq "$total_tests" ]; then
        log_success "✓ ВСЕ ТЕСТЫ ПРОЙДЕНЫ УСПЕШНО!"
        echo -e "${GREEN}"
        cat << "EOF"
        ╔══════════════════════════════════════╗
        ║     ВСЕ ТЕСТЫ ПРОЙДЕНЫ УСПЕШНО!     ║
        ╚══════════════════════════════════════╝
EOF
        echo -e "${NC}"
        return 0
    else
        log_error "✗ НЕ ВСЕ ТЕСТЫ ПРОЙДЕНЫ"
        echo -e "${RED}"
        cat << "EOF"
        ╔══════════════════════════════════════╗
        ║        ТЕСТЫ ПРОВАЛЕНЫ!              ║
        ╚══════════════════════════════════════╝
EOF
        echo -e "${NC}"
        return 1
    fi
}

# Запуск
main