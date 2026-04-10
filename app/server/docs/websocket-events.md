# Socket.IO Events — Référence

Documentation des events temps-réel exposés par le serveur ESP.

## Overview

| | |
|---|---|
| Crate | `socketioxide 0.18` |
| Path | `/v1/ws` |
| Namespace | `/` (unique) |
| Auth | JWT via cookie `auth` (validé dans `auth_middleware`, `server/src/api/ws/auth.rs:34`) |
| Step-up | Cookie `audit_access` requis pour rejoindre une room d'audit |

À la connexion, le middleware d'auth rejoint automatiquement le socket dans sa room personnelle `user:{user_id}`. Toute autre room doit être rejointe explicitement via `room:join`.

## Rooms

| Room | Quand on la rejoint | Pré-requis |
|---|---|---|
| `user:{user_id}` | Auto à la connexion | JWT valide |
| `audit:{audit_id}` | Via `room:join` `Audit { id }` | Membre de l'équipe OU client de l'audit OU admin **+** step-up `audit_access` |

Permissions vérifiées dans `server/src/api/ws/rooms/permissions.rs` (`can_join_room`, `validate_step_up_from_headers`).

## Client → Server

### `room:join`
Rejoint une room.

- **Input** : `RoomPayload`
  - `{ "type": "me" }` — room personnelle
  - `{ "type": "audit", "id": "<audit_id>" }` — room d'audit
- **Réponse** : `room:joined { room }` ou `room:error { code, message }`
- **Source** : `server/src/api/ws/rooms/handlers.rs:31`

### `room:leave`
Quitte une room.

- **Input** : `RoomPayload` (idem `room:join`)
- **Réponse** : `room:left { room }` ou `room:error`
- **Source** : `server/src/api/ws/rooms/handlers.rs:90`

### `agent:shutdown`
Demande l'arrêt d'un agent connecté à un audit. Action loggée (`EventAction::AgentShutdown`).

- **Room requise** : être dans `audit:{audit_id}` avec permission de modifier l'audit
- **Input** :
  ```json
  { "audit_id": "...", "agent_id": "...", "reason": "optional" }
  ```
- **Réponse** : ack émis directement au socket appelant, ou `room:error`
- **Source** : `server/src/api/ws/rooms/handlers.rs:157`

### `agent:command`
Ouvre un stream multiplexé sur l'agent et lui envoie une commande shell. Action loggée (`EventAction::AgentCommand`). La sortie revient en streaming via `agent:output`.

- **Room requise** : être dans `audit:{audit_id}` avec permission de modifier l'audit
- **Input** :
  ```json
  { "audit_id": "...", "agent_id": "...", "command": "..." }
  ```
- **Réponse** : ack direct, puis flux d'`agent:output` ; `room:error` en cas d'échec
- **Source** : `server/src/api/ws/rooms/handlers.rs:277`

## Server → Client

### Réponses room management

| Event | Cible | Payload | Émis quand | Source |
|---|---|---|---|---|
| `room:joined` | socket appelant | `{ room: string }` | succès `room:join` | `server/src/api/ws/rooms/handlers.rs:84` |
| `room:left` | socket appelant | `{ room: string }` | succès `room:leave` | `server/src/api/ws/rooms/handlers.rs:110` |
| `room:error` | socket appelant | `{ code: string, message: string }` | toute erreur de handler WS | `server/src/api/ws/rooms/handlers.rs:116`, `server/src/api/ws/error.rs` |

### Events agent (room `audit:{audit_id}`)

Tous broadcastés à tous les sockets de la room d'audit. Source : `server/src/api/ws/events/agent.rs`.

| Event | Payload | Émis quand | Ligne |
|---|---|---|---|
| `agent:output` | `{ agent_id, stream_id (u32), data (base64) }` | TCP `StreamData` reçu de l'agent | `agent.rs:12` |
| `agent:status` | `{ agent_id, online (bool) }` | Connexion/déconnexion TCP de l'agent | `agent.rs:39` |
| `agent:stream_closed` | `{ agent_id, stream_id }` | TCP `StreamClose` reçu | `agent.rs:57` |
| `agent:keylogger` | `{ agent_id, data (base64) }` | TCP `KeyloggerData` reçu | `agent.rs:82` |

### Events audit (room `audit:{audit_id}`)

| Event | Payload | Émis quand | Source |
|---|---|---|---|
| `audit:message` | `AuditMessage` complet | `POST /v1/audits/{id}/messages` | `server/src/api/audits/messages/endpoints.rs:102` |
| `audit:message_attachment` | `AuditMessageAttachment` complet | `POST /v1/audits/{id}/messages/{msg_id}/attachments` | `server/src/api/audits/messages/endpoints.rs:194` |

### Events utilisateur (room `user:{user_id}`)

| Event | Payload | Émis quand | Source |
|---|---|---|---|
| `notification:new` | `Notification` complet (id, user_id, type, is_read, metadata, created_at, updated_at) | Création d'une notification serveur (invitations équipe/audit, etc.) | `server/src/api/notifications/services.rs:96` |

Helpers définis mais non câblés (réservés pour usage futur, marqués `dead_code`) :

- `user:notification` — payload `{ title, message, notification_type }` (`server/src/api/ws/events/user.rs:11`)
- `user:invitation_received` — payload `{ invitation_id, team_name?, audit_name?, invited_by }` (`server/src/api/ws/events/user.rs:24`)

## Codes d'erreur (`room:error`)

Énumérés dans `server/src/api/ws/error.rs`.

| Code | Sens |
|---|---|
| `AUTH_FAILED` | Socket non authentifié (claims absents/invalides) |
| `ROOM_ACCESS_DENIED` | L'utilisateur n'a pas le droit d'accéder à la room demandée |
| `AUDIT_STEP_UP_REQUIRED` | Cookie `audit_access` manquant pour rejoindre une room d'audit |
| `AUDIT_STEP_UP_INVALID` | Cookie `audit_access` invalide ou expiré |
| `INVALID_PAYLOAD` | Payload reçu non conforme au schéma attendu |
| `DATABASE_ERROR` | Erreur interne lors d'un accès base |
| `AUDIT_NOT_FOUND` | L'audit ciblé n'existe pas |
| `AUDIT_ARCHIVED` | L'audit est archivé, action interdite |
