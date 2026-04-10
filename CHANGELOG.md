# Changelog

All notable changes to `app` will be documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

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
