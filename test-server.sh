#!/bin/bash

# test_api.sh
# Скрипт для тестирования HTTP API сервера Blog

set -e  # Выход при ошибке

# Цвета для вывода
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Конфигурация
BASE_URL=${1:-"http://localhost:8080"}
API_URL="${BASE_URL}/api"
TIMEOUT=5
DEBUG=false

# Переменные для хранения данных
USER_TOKEN=""
USER_ID=""
USER_EMAIL="testuser@example.com"
USER_PASSWORD="TestPassword123!"
POST_ID=""

# Вспомогательные функции
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_debug() {
    if [ "$DEBUG" = true ]; then
        echo -e "[DEBUG] $1"
    fi
}

print_json() {
    if command -v jq &> /dev/null; then
        echo "$1" | jq .
    else
        echo "$1"
        echo "Note: Install 'jq' for pretty JSON output"
    fi
}

make_request() {
    local method=$1
    local endpoint=$2
    local data=$3
    local require_auth=$4

    local curl_cmd="curl -s -X $method"
    curl_cmd="$curl_cmd '$API_URL$endpoint'"
    curl_cmd="$curl_cmd -H 'Content-Type: application/json'"
    curl_cmd="$curl_cmd -H 'Accept: application/json'"

    if [ "$require_auth" = true ] && [ -n "$USER_TOKEN" ]; then
        curl_cmd="$curl_cmd -H 'Authorization: Bearer $USER_TOKEN'"
    fi

    if [ -n "$data" ]; then
        curl_cmd="$curl_cmd -d '$data'"
    fi

    curl_cmd="$curl_cmd --connect-timeout $TIMEOUT"

    log_debug "Выполняем запрос: $curl_cmd"

    local response
    response=$(eval "$curl_cmd")
    local exit_code=$?

    if [ $exit_code -ne 0 ]; then
        log_error "CURL error: $exit_code"
        return 1
    fi

    echo "$response"
    return 0
}

check_server() {
    log_info "Проверяем доступность сервера..."
    echo "$BASE_URL/api/health"
    if ! curl -s "$BASE_URL/api/health" | grep -q "ok"; then
        log_error "Сервер не отвечает на $BASE_URL"
        log_error "Убедитесь, что сервер запущен и доступен"
        exit 1
    fi

    log_success "Сервер доступен"
}

test_register() {
    log_info "Тестируем регистрацию пользователя..."

    # Генерируем уникальный email для каждого теста
    local timestamp=$(date +%s)
    local test_email="user${timestamp}@example.com"
    USER_EMAIL=$test_email

    local data=$(cat <<EOF
{
    "username": "testuser${timestamp}",
    "email": "$test_email",
    "password": "$USER_PASSWORD"
}
EOF
    )

    local response
    response=$(make_request "POST" "/auth/register" "$data" false)

    if echo "$response" | grep -q '"token"'; then
        USER_TOKEN=$(echo "$response" | grep -o '"token":"[^"]*' | cut -d'"' -f4)
        USER_ID=$(echo "$response" | grep -o '"id":"[^"]*' | cut -d'"' -f4)
        log_success "Регистрация успешна"
        log_debug "Получен токен: ${USER_TOKEN:0:20}..."
        print_json "$response"
        return 0
    elif echo "$response" | grep -q "already exists"; then
        log_warning "Пользователь уже существует, тестируем логин..."
        test_login
        return 1
    else
        log_error "Регистрация не удалась"
        print_json "$response"
        return 1
    fi
}

test_login() {
    log_info "Тестируем вход пользователя..."

    local data=$(cat <<EOF
{
    "email": "$USER_EMAIL",
    "password": "$USER_PASSWORD"
}
EOF
    )

    local response
    response=$(make_request "POST" "/auth/login" "$data" false)

    if echo "$response" | grep -q '"token"'; then
        USER_TOKEN=$(echo "$response" | grep -o '"token":"[^"]*' | cut -d'"' -f4)
        USER=$(echo "$response" | grep -o '"user":"[^"]*' | cut -d'"' -f4)
        log_success "Вход успешен"
        print_json "$response"
        return 0
    else
        log_error "Вход не удался"
        print_json "$response"
        return 1
    fi
}

test_create_post() {
    log_info "Тестируем создание поста..."

    local timestamp=$(date +%s)
    local data=$(cat <<EOF
{
    "title": "Test Post $timestamp",
    "content": "This is a test post created at $(date). This post contains test content for API testing purposes."
}
EOF
    )

    local response
    response=$(make_request "POST" "/posts" "$data" true)

    if echo "$response" | grep -q '"id"'; then
        POST_ID=$(echo "$response" | grep -o '"id": [0-9]\+' | cut -d" " -f2)
        log_success "Пост создан успешно"
        log_info "ID поста: $POST_ID"
        print_json "$response"
        return 0
    else
        log_error "Создание поста не удалось"
        print_json "$response"
        return 1
    fi
}

test_get_post() {
    if [ -z "$POST_ID" ]; then
        log_error "ID поста не задан, сначала создайте пост"
        return 1
    fi

    log_info "Тестируем получение поста (ID: $POST_ID)..."

    local response
    response=$(make_request "GET" "/posts/$POST_ID" "" false)

    if echo "$response" | grep -q '"id"'; then
        log_success "Получение поста успешно"
        print_json "$response"
        return 0
    else
        log_error "Получение поста не удалось"
        print_json "$response"
        return 1
    fi
}

test_update_post() {
    if [ -z "$POST_ID" ]; then
        log_error "ID поста не задан, сначала создайте пост"
        return 1
    fi

    log_info "Тестируем обновление поста (ID: $POST_ID)..."

    local timestamp=$(date +%s)
    local data=$(cat <<EOF
{
    "title": "Updated Test Post $timestamp",
    "content": "This post was updated at $(date). The content has been modified for testing purposes."
}
EOF
    )

    local response
    response=$(make_request "PUT" "/posts/$POST_ID" "$data" true)

    if echo "$response" | grep -q '"id"'; then
        log_success "Обновление поста успешно"
        print_json "$response"
        return 0
    else
        log_error "Обновление поста не удалось"
        print_json "$response"
        return 1
    fi
}

test_delete_post() {
    if [ -z "$POST_ID" ]; then
        log_error "ID поста не задан, сначала создайте пост"
        return 1
    fi

    log_info "Тестируем удаление поста (ID: $POST_ID)..."

    local response
    response=$(make_request "DELETE" "/posts/$POST_ID" "" true)

    # DELETE обычно возвращает 204 No Content или пустой ответ
    if [ -z "$response" ] || echo "$response" | grep -q '"success":true'; then
        log_success "Удаление поста успешно"

        # Проверяем, что пост действительно удален
        log_info "Проверяем, что пост удален..."
        local get_response
        get_response=$(make_request "GET" "/posts/$POST_ID" "" false 2>/dev/null || true)

        if echo "$get_response" | grep -q "not found\|NotFound"; then
            log_success "Пост успешно удален из системы"
        else
            log_warning "Пост может быть еще доступен"
        fi

        return 0
    else
        log_error "Удаление поста не удалось"
        print_json "$response"
        return 1
    fi
}

test_list_posts() {
    log_info "Тестируем получение списка постов..."

    log_info "Получаем первые 5 постов..."
    local response
    response=$(make_request "GET" "/posts?limit=5&offset=0" "" false)

    if echo "$response" | grep -q '"posts"'; then
        log_success "Получение списка постов успешно"
        local post_count=$(echo "$response" | grep -o '"posts":\[.*\]' | grep -o '{"id"' | wc -l || echo "0")
        log_info "Получено постов: $post_count"
        print_json "$response"
        return 0
    else
        log_error "Получение списка постов не удалось"
        print_json "$response"
        return 1
    fi
}

test_pagination() {
    log_info "Тестируем пагинацию..."

    log_info "Создаем несколько тестовых постов..."
    for i in {1..3}; do
        local data=$(cat <<EOF
{
    "title": "Pagination Test Post $i",
    "content": "Post $i for pagination testing"
}
EOF
        )
        make_request "POST" "/posts" "$data" true > /dev/null 2>&1
        log_debug "Создан пост $i"
    done

    log_info "Тестируем offset=0, limit=2..."
    local response1
    response1=$(make_request "GET" "/posts?offset=0&limit=2" "" false)

    log_info "Тестируем offset=2, limit=2..."
    local response2
    response2=$(make_request "GET" "/posts?offset=2&limit=2" "" false)

    local ids1=$(echo "$response1" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | sort)
    local ids2=$(echo "$response2" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | sort)

    # Проверяем, что посты разные (если достаточно постов)
    if [ -n "$ids1" ] && [ -n "$ids2" ]; then
        local common=$(comm -12 <(echo "$ids1") <(echo "$ids2"))
        if [ -z "$common" ]; then
            log_success "Пагинация работает корректно"
        else
            log_warning "Возможно дублирование постов при пагинации"
        fi
    fi

    return 0
}

test_error_cases() {
    log_info "Тестируем обработку ошибок..."

    log_info "1. Регистрация с существующим email..."
    local data=$(cat <<EOF
{
    "username": "existinguser",
    "email": "$USER_EMAIL",
    "password": "password123"
}
EOF
    )
    local response
    response=$(make_request "POST" "/auth/register" "$data" false)
    if echo "$response" | grep -q "already exists\|409"; then
        log_success "Конфликт при регистрации обрабатывается корректно"
    else
        log_warning "Ожидалась ошибка 409 Conflict"
    fi

    log_info "2. Вход с неверными учетными данными..."
    local data=$(cat <<EOF
{
    "email": "wrong@example.com",
    "password": "wrongpassword"
}
EOF
    )
    response=$(make_request "POST" "/auth/login" "$data" false)
    if echo "$response" | grep -q "unauthorized\|401"; then
        log_success "Неверные учетные данные обрабатываются корректно"
    else
        log_warning "Ожидалась ошибка 401 Unauthorized"
    fi

    log_info "3. Создание поста без аутентификации..."
    local data=$(cat <<EOF
{
    "title": "Unauthorized Post",
    "content": "This should fail"
}
EOF
    )
    response=$(make_request "POST" "/posts" "$data" false)
    if echo "$response" | grep -q "unauthorized\|401"; then
        log_success "Защищенные endpoint'ы требуют аутентификации"
    else
        log_warning "Ожидалась ошибка 401 Unauthorized"
    fi

    log_info "4. Получение несуществующего поста..."
    response=$(make_request "GET" "/posts/nonexistent-id-12345" "" false)
    if echo "$response" | grep -q "not found\|404"; then
        log_success "Несуществующий пост обрабатывается корректно"
    else
        log_warning "Ожидалась ошибка 404 Not Found"
    fi
}

test_unauthorized_update_delete() {
    log_info "Тестируем обновление/удаление чужого поста..."

    # Создаем новый пользователь для теста
    local timestamp=$(date +%s)
    local other_email="otheruser${timestamp}@example.com"

    # Регистрируем второго пользователя
    local data=$(cat <<EOF
{
    "username": "otheruser${timestamp}",
    "email": "$other_email",
    "password": "OtherPassword123!"
}
EOF
    )

    local response
    response=$(make_request "POST" "/auth/register" "$data" false)
    local other_token=""

    if echo "$response" | grep -q '"token"'; then
        other_token=$(echo "$response" | grep -o '"token":"[^"]*' | cut -d'"' -f4)

        # Создаем пост от имени второго пользователя
        local post_data=$(cat <<EOF
{
    "title": "Other User Post",
    "content": "This post belongs to another user"
}
EOF
        )

        local post_response
        post_response=$(curl -s -X POST "$API_URL/posts" \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer $other_token" \
            -d "$post_data")

        local other_post_id=$(echo "$post_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4)

        if [ -n "$other_post_id" ]; then
            log_info "Пытаемся обновить чужой пост с токеном первого пользователя..."

            local update_data=$(cat <<EOF
{
    "title": "Trying to update other's post",
    "content": "This should fail"
}
EOF
            )

            local update_response
            update_response=$(make_request "PUT" "/posts/$other_post_id" "$update_data" true)

            if echo "$update_response" | grep -q "forbidden\|403\|unauthorized"; then
                log_success "Запрет на обновление чужого поста работает"
            else
                log_warning "Ожидалась ошибка 403 Forbidden при обновлении чужого поста"
            fi

            # Очистка: удаляем пост второго пользователя
            curl -s -X DELETE "$API_URL/posts/$other_post_id" \
                -H "Authorization: Bearer $other_token" > /dev/null
        fi
    fi
}

print_summary() {
    log_info "========================================"
    log_info "ТЕСТИРОВАНИЕ ЗАВЕРШЕНО"
    log_info "========================================"
    log_info "Сервер: $BASE_URL"
    log_info "Пользователь: $USER_EMAIL"
    log_info "ID пользователя: ${USER_ID:0:8}..."
    log_info "Токен: ${USER_TOKEN:0:20}..."
    log_info "========================================"
}

cleanup() {
    log_info "Очистка тестовых данных..."

    # Удаляем созданный пост, если он еще существует
    if [ -n "$POST_ID" ]; then
        curl -s -X DELETE "$API_URL/posts/$POST_ID" \
            -H "Authorization: Bearer $USER_TOKEN" > /dev/null 2>&1 || true
    fi

    log_info "Очистка завершена"
}

main() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}  ТЕСТИРОВАНИЕ HTTP API СЕРВЕРА BLOG  ${NC}"
    echo -e "${BLUE}========================================${NC}"

    # Проверяем зависимости
    if ! command -v curl &> /dev/null; then
        log_error "Требуется curl. Установите: sudo apt-get install curl"
        exit 1
    fi

    if [ "$DEBUG" = true ]; then
        log_warning "Режим отладки включен"
    fi

    # Параметры командной строки
    while [[ $# -gt 0 ]]; do
        case $1 in
            -d|--debug)
                DEBUG=true
                shift
                ;;
            -u|--url)
                BASE_URL="$2"
                API_URL="$BASE_URL/api"
                shift 2
                ;;
            -h|--help)
                echo "Использование: $0 [BASE_URL]"
                echo "  BASE_URL - URL сервера (по умолчанию: http://localhost:8080)"
                echo "  -d, --debug - включить режим отладки"
                echo "  -u, --url URL - указать URL сервера"
                exit 0
                ;;
            *)
                BASE_URL="$1"
                API_URL="$BASE_URL/api"
                shift
                ;;
        esac
    done

    # Ловим прерывание для очистки
    trap cleanup EXIT
    trap 'log_error "Прервано пользователем"; exit 1' INT TERM

    # Запускаем тесты
    check_server
    test_register || test_login
    test_create_post
    test_get_post
    test_update_post
    test_list_posts
    test_pagination
    test_delete_post
    test_error_cases
    test_unauthorized_update_delete

    print_summary

    log_success "Все тесты пройдены успешно!"

    # Очистка выполнится автоматически через trap EXIT
}

# Запуск основного скрипта
main "$@"