# Changelog

All notable changes to `app` will be documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [1.7.1] - 2026-04-11

### Bug Fixes

- fix(ci): disable skipCi so tag pipelines trigger build/deploy

## [1.7.0] - 2026-04-11

### Features

- feat(agents): add self-update via TCP stream with version comparison

### Bug Fixes

- fix(ci): trigger build/deploy on tags only, remove git fetch hack
- fix(ci): use dnf instead of apk in buildah image, exclude done tickets from metrics
- fix(metrics): exclude completed tickets (last column) from dashboard counts
- fix(ci): git pull latest before docker build to include ferrflow version bump
- fix(tcp): bind agent listener on 0.0.0.0 instead of [::] for IPv4 compatibility

## [1.6.1] - 2026-04-11

### Bug Fixes

- fix(ci): git pull latest before docker build to include ferrflow version bump
- fix(tcp): bind agent listener on 0.0.0.0 instead of [::] for IPv4 compatibility

## [1.6.0] - 2026-04-11

### Features

- feat(metrics): add per-priority assigned ticket counts for segmented bars

### Bug Fixes

- fix(metrics): use lowercase priority values to match DB storage

## [1.5.0] - 2026-04-11

### Features

- feat(events): include metadata in audit log responses
- feat(events): add event logging for WebSocket room join
- feat(events): add AgentDownload event action for agent binary downloads
- feat(events): add event logging for agent download, shell, keylogger, revoke, and messages
- feat(agents): decouple agent download from creation with deployment tokens

### Bug Fixes

- fix: resolve clippy match_same_arms in event inference
- fix(metrics): count ticket priorities across all user audits, not just assigned
- fix(ci): pin ferrflow to v2.22.1
- fix(ci): remove unnecessary git checkout from ferrflow job
- fix(ci): use ferrflow version command, remove git dep from release job
- fix: resolve clippy map_unwrap_or in agent hostname detection

## [1.4.2] - 2026-04-10

### Bug Fixes

- fix(ci): auto-deploy dev on main push
- fix(ci): ferrflow 2.20.3, enable on MRs, skip pipelines on non-main commits

## [1.4.1] - 2026-04-10

### Bug Fixes

- fix: client dans le list_user_audits
- fix clippy

## [1.4.0] - 2026-04-10

### Features

- feat: add user metrics endpoint

## [1.3.0] - 2026-04-10

### Features

- feat: add admin dashboard metrics endpoint

## [1.2.4] - 2026-04-09

### Bug Fixes

- fix: resolve remaining clippy collapsible_if in linux-only code paths
- fix(ci): move CARGO_HOME inside project dir for cache compatibility
- fix: resolve clippy pedantic warnings
- fix(ci): cache target/debug instead of full target directory
- fix(api): add avatar_url to team members, audit clients, and event log users
- fix(ci): fix cache keys to avoid parallel job conflicts and define CARGO_HOME
- fix(ci): pin ferrflow to v2.20.0
- fix: merge feat/agent-system-info-and-shell and feat/audit into main
- fix(ci): use wildcard branch for ferrflow dev channel

## [1.2.3] - 2026-04-09

### Bug Fixes

- fix: fix cors

## [1.2.2] - 2026-04-09

### Bug Fixes

- fix: ws cors

## [1.2.1] - 2026-04-09

### Bug Fixes

- fix: ws cors

## [1.2.0] - 2026-04-08

### Features

- feat: use ferrflow GHCR image and enable PR mode with automerge
- feat: keylogger

### Bug Fixes

- fix(ci): update ferrflow config and release job for self-hosted GitLab
- fix(tests): corriger 2 tests d'integration qui echouaient
- fix(ci): utiliser l'image sonar-scanner-cli au lieu d'installer manuellement
- fix(ci): corriger le working directory du scanner SonarQube
- fix: update rust crate jsonwebtoken to v10
- fix: update rust crate which to v8
- fix: update rust crate which to v8
- fix: update rust crate jsonwebtoken to v10
- fix: update rust crate windows to 0.62
- fix: update rust crate windows to 0.62
- fix: update rust crate resend-rs to 0.22.0
- fix(ci): use needs instead of dependencies for deploy jobs
- fix(ci): remove git commands from ferrflow release job
- fix: add FERRFLOW_TOKEN to release job
