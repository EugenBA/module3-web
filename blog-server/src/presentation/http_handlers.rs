use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Scope};
use tracing::info;
use uuid::Uuid;

use crate::data::user_repository::InDbUserRepository;
use crate::domain::error::DomainError;
use crate::presentation::auth::AuthenticatedUser;


pub fn scope() -> Scope {
    web::scope("")
        .service(create_account)
        .service(get_account)
        .service(list_accounts)
        .service(deposit)
        .service(withdraw)
        .service(transfer)
}

fn ensure_owner(owner_id: i64, user: &AuthenticatedUser) -> Result<(), DomainError> {
    if owner_id != user.id {
        Err(DomainError::Unauthorized)
    } else {
        Ok(())
    }
}

#[post("/accounts")]
async fn create_account(
    req: HttpRequest,
    user: AuthenticatedUser,
    bank: web::Data<BankService<InMemoryAccountRepository>>,
    payload: web::Json<CreateAccountRequest>,
) -> Result<HttpResponse, BankError> {
    let account = bank
        .create_account(payload.id, user.id, payload.initial)
        .await?;

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        account_id = account.id,
        "account created"
    );

    Ok(HttpResponse::Created().json(AccountResponse::from(account)))
}

#[get("/accounts/{id}")]
async fn get_account(
    req: HttpRequest,
    user: AuthenticatedUser,
    bank: web::Data<BankService<InMemoryAccountRepository>>,
    path: web::Path<u32>,
) -> Result<HttpResponse, BankError> {
    let account = bank.get_account(path.into_inner()).await?;
    ensure_owner(account.owner_id, &user)?;
    let response = AccountResponse::from(account);

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        account_id = response.id,
        "account fetched"
    );

    Ok(HttpResponse::Ok().json(response))
}

#[get("/accounts")]
async fn list_accounts(
    req: HttpRequest,
    user: AuthenticatedUser,
    bank: web::Data<BankService<InMemoryAccountRepository>>,
) -> Result<HttpResponse, BankError> {
    let accounts = bank.list_accounts(user.id).await?;
    let response: Vec<_> = accounts.into_iter().map(AccountResponse::from).collect();

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        "accounts listed"
    );

    Ok(HttpResponse::Ok().json(response))
}

#[post("/accounts/{id}/deposit")]
async fn deposit(
    req: HttpRequest,
    user: AuthenticatedUser,
    bank: web::Data<BankService<InMemoryAccountRepository>>,
    path: web::Path<u32>,
    payload: web::Json<AmountRequest>,
) -> Result<HttpResponse, BankError> {
    let account_id = path.into_inner();
    let account = bank.get_account(account_id).await?;
    ensure_owner(account.owner_id, &user)?;

    let account = bank.deposit(account_id, payload.amount).await?;
    let response = AccountResponse::from(account);

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        account_id = response.id,
        amount = payload.amount,
        "deposit successful"
    );

    Ok(HttpResponse::Ok().json(response))
}

#[post("/accounts/{id}/withdraw")]
async fn withdraw(
    req: HttpRequest,
    user: AuthenticatedUser,
    bank: web::Data<BankService<InMemoryAccountRepository>>,
    path: web::Path<u32>,
    payload: web::Json<AmountRequest>,
) -> Result<HttpResponse, BankError> {
    let account_id = path.into_inner();
    let account = bank.get_account(account_id).await?;
    ensure_owner(account.owner_id, &user)?;

    let account = bank.withdraw(account_id, payload.amount).await?;
    let response = AccountResponse::from(account);

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        account_id = response.id,
        amount = payload.amount,
        "withdraw successful"
    );

    Ok(HttpResponse::Ok().json(response))
}

#[post("/transfers")]
async fn transfer(
    req: HttpRequest,
    user: AuthenticatedUser,
    bank: web::Data<BankService<InMemoryAccountRepository>>,
    payload: web::Json<TransferRequest>,
) -> Result<HttpResponse, BankError> {
    if payload.from == payload.to {
        return Err(BankError::Validation(
            "source and destination must differ".into(),
        ));
    }

    let account = bank.get_account(payload.from).await?;
    ensure_owner(account.owner_id, &user)?;

    bank.transfer(payload.from, payload.to, payload.amount).await?;

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        from = payload.from,
        to = payload.to,
        amount = payload.amount,
        "transfer successful"
    );

    Ok(HttpResponse::Ok().json(serde_json::json!({ "status": "transferred" })))
}

fn request_id(req: &HttpRequest) -> String {
    req.extensions()
        .get::<crate::presentation::middleware::RequestId>()
        .map(|rid| rid.0.clone())
        .unwrap_or_else(|| "unknown".into())
}

