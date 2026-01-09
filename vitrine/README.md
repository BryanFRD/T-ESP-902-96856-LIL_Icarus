# Vitrine

Ce projet est une vitrine développée avec les meilleures pratiques de qualité de code, incluant ESLint, Prettier et Husky pour garantir la cohérence et la fiabilité du code.

## Prérequis

- **Node.js** : version recommandée `22.15.0`
- **npm** : version recommandée `10.9.0`
- Un gestionnaire de version Git

## Installation du projet

Clonez le dépôt via SSH puis installez les dépendances :

```bash
git clone git@git.icarus-cyber.fr:icarus/vitrine.git
cd vitrine
npm install
```

## Lancement du projet

Pour démarrer le projet en mode développement :

```bash
npm run dev
```

Pour lancer le projet en production (si applicable) :

```bash
npm run build
npm start
```

## Scripts utiles

- `npm run prepare` : Installe les hooks Husky.
- `npm run dev` : Démarre le serveur de développement (hot reload).
- `npm run build` : Génère la version de production.
- `npm start` : Démarre l’application en mode production.
- `npm run lint` : Lint le code avec ESLint.
- `npm run format` : Formate le code avec Prettier.
- `npm run format:check` : Vérifie le formatage du code sans l’appliquer.

## Qualité de code

### ESLint

ESLint est utilisé pour analyser le code et détecter les erreurs ou incohérences de style. La configuration se trouve dans `.eslint.config.mjs`.

- Lancer manuellement :
  ```bash
  npm run lint
  ```

### Prettier

Prettier est utilisé pour formater automatiquement le code selon des règles définies dans `.prettierrc.json`.

- Lancer manuellement :
  ```bash
  npm run format
  ```

### Husky

Husky permet d’ajouter des hooks Git, notamment pour lancer des vérifications avant chaque commit (ex : lint, format).

- Les hooks sont installés via :
  ```bash
  npm run prepare
  ```
- Exemple de hook : `pre-commit` lance automatiquement ESLint et Prettier avant chaque commit.
- **Si le code n'est pas conforme aux règles ESLint ou Prettier, le commit est annulé et doit être corrigé avant de pouvoir être appliqué.**
- Un hook `commit-msg` utilise **commitlint** pour vérifier que le message de commit respecte le format conventionnel. Si le message n'est pas valide, le commit est également refusé.
- Pour valider le message de commit, utilisez le format suivant :
  ```
  type(scope): description
  ```
  - `type` : type de changement (feat, fix, docs, style, refactor, perf, test, chore)
  - `scope` : portée du changement (facultatif)
  - `description` : description concise du changement

## Structure du projet

- `src/` : Code source principal
- `public/` : Fichiers statiques
- `.eslintrc`, `.prettierrc` : Configurations de lint et formatage
- `.husky/` : Hooks Git
