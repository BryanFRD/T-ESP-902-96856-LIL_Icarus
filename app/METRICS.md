# Métriques Prometheus

Ce document liste toutes les métriques Prometheus exposées par l'application sur `/metrics`.

## Types de métriques et utilisation dans Grafana

### Counters
Les **counters** augmentent toujours et ne sont jamais reset (ex: `http_requests_total`, `db_queries_total`).

**IMPORTANT:** Ne jamais grapher un counter directement. Utiliser `rate()` ou `irate()` dans Grafana :

```promql
rate(http_requests_total[1m])      # Taux moyen sur 1 minute
irate(http_requests_total[1m])     # Taux instantané
```

Si vous graphez directement `http_requests_total`, le graphe va toujours monter (c'est normal).

### Gauges
Les **gauges** reflètent une valeur actuelle qui peut monter et descendre (ex: `users_total`, `db_connections_active`).

Peuvent être graphées directement sans transformation.

### Histograms
Les **histograms** capturent la distribution des valeurs (ex: `http_request_duration_seconds`).

Utilisez `histogram_quantile()` pour calculer les percentiles :

```promql
histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))  # p95
```

---

## Métriques HTTP

### `http_request_duration_seconds` (histogram)
Durée des requêtes HTTP en secondes.

**Labels:**
- `method` : méthode HTTP (GET, POST, etc.)
- `path` : chemin normalisé (`/v1/teams/:id`)
- `status` : code de statut HTTP

**Buckets:** 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0

**Grafana:** Utilisez `histogram_quantile()` pour p50, p95, p99

---

### `http_requests_total` (counter)
Nombre total de requêtes HTTP.

**Labels:**
- `method` : méthode HTTP
- `path` : chemin normalisé
- `status` : code de statut HTTP

**Grafana:** `rate(http_requests_total[5m])` pour le taux de requêtes

---

### `http_requests_in_flight` (gauge)
Nombre de requêtes HTTP en cours de traitement.

**Labels:**
- `method` : méthode HTTP
- `path` : chemin normalisé

---

### `http_errors_total` (counter)
Nombre total d'erreurs HTTP (4xx et 5xx).

**Labels:**
- `method` : méthode HTTP
- `path` : chemin normalisé
- `status` : code de statut HTTP
- `error_type` : `client_error` (4xx) ou `server_error` (5xx)

**Grafana:** `rate(http_errors_total{error_type="server_error"}[5m])` pour le taux d'erreurs serveur

---

## Métriques Système/Process

### `process_cpu_seconds_total` (gauge)
Temps CPU total consommé par le processus en secondes.

**Grafana:** `rate(process_cpu_seconds_total[1m])` pour l'utilisation CPU

---

### `process_resident_memory_bytes` (gauge)
Taille de la mémoire résidente (RSS) en bytes.

**Grafana:** `process_resident_memory_bytes / 1024 / 1024` pour la mémoire en MB

---

### `process_virtual_memory_bytes` (gauge)
Taille de la mémoire virtuelle (VMS) en bytes.

---

### `process_open_fds` (gauge)
Nombre de file descriptors ouverts.

---

### `process_start_time_seconds` (gauge)
Timestamp de démarrage du processus (secondes depuis epoch UNIX).

**Grafana:** `time() - process_start_time_seconds` pour l'uptime

---

## Métriques Base de Données

### `db_connections_active` (gauge)
Nombre de connexions actives au pool de base de données.

---

### `db_connections_idle` (gauge)
Nombre de connexions inactives (disponibles) dans le pool.

**Grafana:** `db_connections_active / (db_connections_active + db_connections_idle) * 100` pour le taux d'utilisation

---

### `db_queries_total` (counter)
Nombre total de requêtes exécutées sur la base de données.

**Implémentation:** Incrémenté à chaque appel à `get_db_conn()`.

**Grafana:** `rate(db_queries_total[1m])` pour queries/seconde

---

## Métriques Authentification

### `auth_login_total` (counter)
Nombre total de tentatives de login.

**Labels:**
- `status` : `success` ou `failure`

**Grafana:**
- `rate(auth_login_total{status="success"}[5m])` pour le taux de logins réussis
- `rate(auth_login_total{status="failure"}[5m])` pour détecter les attaques brute-force

---

### `auth_register_total` (counter)
Nombre total de tentatives d'inscription.

**Labels:**
- `status` : `success` ou `failure`

---

## Métriques Business

### `users_total` (gauge)
Nombre total d'utilisateurs dans la base de données.

---

### `teams_total` (gauge)
Nombre total d'équipes dans la base de données.

---

### `audits_total` (gauge)
Nombre total d'audits dans la base de données.

**Labels:**
- `archived` : `true` ou `false`

**Grafana:** Utilisez les deux labels pour voir audits actifs vs archivés

---

## Fréquence de Collecte

- **Métriques HTTP** : collectées en temps réel (middleware)
- **Métriques système** : collectées toutes les 15 secondes
- **Métriques DB pool** : collectées toutes les 15 secondes
- **Métriques business** : collectées toutes les 15 secondes
- **Métriques auth** : collectées en temps réel (endpoints)

---

## Endpoint

Toutes les métriques sont exposées sur :

```
GET /metrics
```

Format : Prometheus text exposition format

---

## Exemples de Requêtes PromQL

### Latence p95 par endpoint
```promql
histogram_quantile(0.95,
  rate(http_request_duration_seconds_bucket[5m])
)
```

### Taux d'erreurs 5xx
```promql
rate(http_errors_total{error_type="server_error"}[5m])
```

### Saturation du pool DB
```promql
db_connections_active / 10 * 100
```

### Uptime
```promql
time() - process_start_time_seconds
```

### Top 5 endpoints par volume
```promql
topk(5, rate(http_requests_total[5m]))
```
