use axum::{routing::post, Json, Router};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::str::FromStr;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

use anchor_client::{
    solana_sdk::signature::{Keypair, Signature},
    solana_sdk::signer::Signer,
    solana_sdk::pubkey::Pubkey,
    Client, Cluster,
};

use anchor_lang::system_program;
use ephemeral_vault;

// ------------------ MEMORY STORE ------------------
pub static EPHEMERAL_MAP: Lazy<Mutex<HashMap<String, Arc<Keypair>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

// ----------------- TYPES -----------------
#[derive(Serialize)]
struct SessionResponse {
    ok: bool,
    parent: String,
    ephemeral: String,
    vault: String,
    tx_or_error: String,
}

#[derive(Deserialize)]
struct RevokeRequest {
    vault: String,
}

#[derive(Deserialize)]
struct TradeRequest {
    vault: String,
    size: i64,
    price: i64,
}

#[derive(Deserialize)]
struct DepositRequest {
    amount: u64,
    vault: String,
}

// ----------------- MAIN -----------------
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/session/create", post(create_session))
        .route("/session/deposit", post(deposit_sol))
        .route("/session/revoke", post(revoke_session))
        .route("/session/trade", post(place_trade));

    println!("Backend running on http://localhost:8080");

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:8080")
            .await
            .expect("failed to bind port"),
        app,
    )
    .await
    .unwrap();
}

// --------------------------------------------------
//  SESSION CREATE
// --------------------------------------------------
async fn create_session() -> Json<SessionResponse> {
    let result = tokio::task::spawn_blocking(run_anchor_session_creation).await;

    match result {
        Ok(Ok(resp)) => Json(resp),
        Ok(Err(e)) => Json(error_resp(e)),
        Err(join_err) => Json(error_resp(format!("join error: {join_err}"))),
    }
}

fn run_anchor_session_creation() -> Result<SessionResponse, String> {
    let wallet = read_keypair("~/.config/solana/devnet.json")?;
    let client = Client::new(Cluster::Devnet, Arc::new(wallet));

    // Parse program id (constant, should never fail; if it does, String error)
    let program_id =
        Pubkey::from_str("2Y2AseLPmKvaGRXsU4yB3hjjMgXyhh9Y4LVgsgkSzCoT").map_err(|e| e.to_string())?;

    let program = client
        .program(program_id)
        .map_err(|e| e.to_string())?;

    let parent_pubkey = program.payer();

    // ephemeral wallet creation
    let ephemeral = Keypair::new();
    let ephemeral_pubkey = ephemeral.pubkey();

    // PDA for vault (versioned)
    let (vault_pda, _bump) = Pubkey::find_program_address(
    &[
        b"vault",
        b"v2",
        parent_pubkey.as_ref(),
    ],
    &program_id,
    );

    // store ephemeral in memory
    EPHEMERAL_MAP
        .lock()
        .map_err(|e| e.to_string())?
        .insert(vault_pda.to_string(), Arc::new(ephemeral));

    // send tx
    let tx: Signature = program
        .request()
        .accounts(ephemeral_vault::accounts::CreateEphemeralVault {
            parent_wallet: parent_pubkey,
            ephemeral_wallet: ephemeral_pubkey,
            vault: vault_pda,
            system_program: system_program::ID,
        })
        .args(ephemeral_vault::instruction::CreateEphemeralVault {
            session_duration: 3600,
        })
        .send()
        .map_err(|e| format!("transaction failed: {e}"))?;

    Ok(SessionResponse {
        ok: true,
        parent: parent_pubkey.to_string(),
        ephemeral: ephemeral_pubkey.to_string(),
        vault: vault_pda.to_string(),
        tx_or_error: tx.to_string(),
    })
}

// --------------------------------------------------
//  DEPOSIT
// --------------------------------------------------
async fn deposit_sol(Json(req): Json<DepositRequest>) -> Json<SessionResponse> {
    let result = tokio::task::spawn_blocking(move || run_anchor_deposit(req)).await;

    match result {
        Ok(Ok(resp)) => Json(resp),
        Ok(Err(e)) => Json(error_resp(e)),
        Err(join_err) => Json(error_resp(format!("join error: {join_err}"))),
    }
}
fn run_anchor_deposit(req: DepositRequest) -> Result<SessionResponse, String> {
    let wallet = read_keypair("~/.config/solana/devnet.json")
        .map_err(|e| e.to_string())?;

    let client = Client::new(Cluster::Devnet, Arc::new(wallet));

    let program_id = Pubkey::from_str("2Y2AseLPmKvaGRXsU4yB3hjjMgXyhh9Y4LVgsgkSzCoT")
        .map_err(|e| e.to_string())?;
    let program = client.program(program_id).map_err(|e| e.to_string())?;

    let parent = program.payer();

    // ❗ MUST RE-GENERATE PDA USING SEEDS — do NOT use req.vault
    let (vault_pda, _bump) =
        Pubkey::find_program_address(&[b"vault", b"v2", parent.as_ref()], &program_id);

    let tx = program
        .request()
        .accounts(ephemeral_vault::accounts::DepositSol {
            parent_wallet: parent,
            vault: vault_pda,
            system_program: anchor_lang::system_program::ID,
        })
        .args(ephemeral_vault::instruction::DepositSol {
            amount: req.amount,
        })
        .send()
        .map_err(|e| format!("transaction failed: {e}"))?;

    Ok(SessionResponse {
        ok: true,
        parent: parent.to_string(),
        ephemeral: "".into(),
        vault: vault_pda.to_string(),
        tx_or_error: tx.to_string(),
    })
}

// --------------------------------------------------
//  REVOKE
// --------------------------------------------------
async fn revoke_session(Json(req): Json<RevokeRequest>) -> Json<SessionResponse> {
    let result = tokio::task::spawn_blocking(move || run_anchor_revoke(req)).await;

    match result {
        Ok(Ok(resp)) => Json(resp),
        Ok(Err(e)) => Json(error_resp(e)),
        Err(join_err) => Json(error_resp(format!("join error: {join_err}"))),
    }
}

fn run_anchor_revoke(req: RevokeRequest) -> Result<SessionResponse, String> {
    let wallet = read_keypair("~/.config/solana/devnet.json")?;
    let client = Client::new(Cluster::Devnet, Arc::new(wallet));

    let program_id =
        Pubkey::from_str("2Y2AseLPmKvaGRXsU4yB3hjjMgXyhh9Y4LVgsgkSzCoT").map_err(|e| e.to_string())?;
    let program = client
        .program(program_id)
        .map_err(|e| e.to_string())?;
    let parent_pubkey = program.payer();

    let vault_pubkey = Pubkey::from_str(&req.vault).map_err(|e| e.to_string())?;

    let tx: Signature = program
        .request()
        .accounts(ephemeral_vault::accounts::RevokeSession {
            parent_wallet: parent_pubkey,
            vault: vault_pubkey,
        })
        .args(ephemeral_vault::instruction::RevokeSession {})
        .send()
        .map_err(|e| format!("transaction failed: {e}"))?;

    // Drop ephemeral from memory
    EPHEMERAL_MAP
        .lock()
        .map_err(|e| e.to_string())?
        .remove(&req.vault);

    Ok(SessionResponse {
        ok: true,
        parent: parent_pubkey.to_string(),
        ephemeral: "".into(),
        vault: vault_pubkey.to_string(),
        tx_or_error: tx.to_string(),
    })
}

// --------------------------------------------------
//  TRADE
// --------------------------------------------------
async fn place_trade(Json(req): Json<TradeRequest>) -> Json<SessionResponse> {
    let result = tokio::task::spawn_blocking(move || run_anchor_trade(req)).await;

    match result {
        Ok(Ok(resp)) => Json(resp),
        Ok(Err(e)) => Json(error_resp(e)),
        Err(join_err) => Json(error_resp(format!("join error: {join_err}"))),
    }
}

fn run_anchor_trade(req: TradeRequest) -> Result<SessionResponse, String> {
    let wallet = read_keypair("~/.config/solana/devnet.json")
        .map_err(|e| e.to_string())?;
    let client = Client::new(Cluster::Devnet, Arc::new(wallet));

    let program_id = Pubkey::from_str("2Y2AseLPmKvaGRXsU4yB3hjjMgXyhh9Y4LVgsgkSzCoT")
        .map_err(|e| e.to_string())?;
    let program = client.program(program_id).map_err(|e| e.to_string())?;

    let parent = program.payer();

    // ❗ MUST RE-GENERATE PDA
    let (vault_pda, _bump) =
        Pubkey::find_program_address(&[b"vault", b"v2", parent.as_ref()], &program_id);

    // get ephemeral wallet for this PDA
    let ephemeral = {
        let map = EPHEMERAL_MAP.lock().map_err(|_| "mutex poisoned".to_string())?;
        map.get(&vault_pda.to_string())
            .ok_or("Ephemeral session not found".to_string())?
            .clone()
    };

    let ephemeral_pubkey = ephemeral.pubkey();

    let tx = program
        .request()
        .signer(&ephemeral)
        .accounts(ephemeral_vault::accounts::PlaceTrade {
            parent_wallet: parent,
            ephemeral_wallet: ephemeral_pubkey,
            vault: vault_pda,
        })
        .args(ephemeral_vault::instruction::PlaceTrade {
            size: req.size,
            price: req.price,
        })
        .send()
        .map_err(|e| format!("transaction failed: {e}"))?;

    Ok(SessionResponse {
        ok: true,
        parent: parent.to_string(),
        ephemeral: ephemeral_pubkey.to_string(),
        vault: vault_pda.to_string(),
        tx_or_error: tx.to_string(),
    })
}


// --------------------------------------------------
//  UTIL
// --------------------------------------------------
fn read_keypair(path: &str) -> Result<Keypair, String> {
    let home = std::env::var("HOME").map_err(|e| format!("HOME env missing: {e}"))?;
    let path = path.replace('~', &home);
    let data = std::fs::read(&path).map_err(|e| format!("fs read {path}: {e}"))?;
    let nums: Vec<u8> = serde_json::from_slice(&data)
        .map_err(|e| format!("parse json keypair {path}: {e}"))?;
    Keypair::from_bytes(&nums).map_err(|e| format!("Keypair::from_bytes: {e}"))
}

fn error_resp(msg: String) -> SessionResponse {
    SessionResponse {
        ok: false,
        parent: "".into(),
        ephemeral: "".into(),
        vault: "".into(),
        tx_or_error: msg,
    }
}
