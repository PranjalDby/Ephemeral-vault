# Ephemeral-vault
GoQuant Assignment
# Ephemeral Vault – Temporary Delegated Trading on Solana

This project implements a secure "Ephemeral Wallet Session" system for Solana.
A parent wallet can create a temporary delegated vault that:
✔ Receives temporary SOL deposits  
✔ Allows an ephemeral keypair to place limited trades  
✔ Can be revoked anytime  
✔ Auto-expires after a set session time

---

## 🔥 Key Features

| Feature | Status |
|--------|--------|
| Create Ephemeral Vault Session | ✅ |
| Deposit SOL | ✅ |
| Place Trade using Ephemeral Wallet | ✅ |
| Revoke Session (invalidates Ephemeral Wallet) | ✅ |
| Session Expiry Check | 🚀 supported on-chain |

---

## 🧠 API Endpoints (Backend)

| Endpoint | Method | Body | Description |
|---------|--------|------|-------------|
| `/session/create` | POST | none | Creates ephemeral session |
| `/session/deposit` | POST | `{vault, amount}` | Deposits SOL |
| `/session/trade` | POST | `{vault, size, price}` | Places trade using ephemeral key |
| `/session/revoke` | POST | `{vault}` | Revokes session |

---

## ▶️ CURL Examples

### Create Session
curl -X POST http://localhost:8080/session/create


### Deposit
curl -X POST http://localhost:8080/session/deposit
-H "Content-Type: application/json"
-d '{"vault": "<vault>", "amount": 1000000}'


### Trade
curl -X POST http://localhost:8080/session/trade
-H "Content-Type: application/json"
-d '{"vault": "<vault>", "size": 25, "price": 29250}'


### Revoke
curl -X POST http://localhost:8080/session/revoke
-H "Content-Type: application/json"
-d '{"vault": "<vault>"}'


---

## 🏗 Build & Run
anchor build
anchor deploy
cd backend && cargo run