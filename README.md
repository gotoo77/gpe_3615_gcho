# 3615 GCHO

**3615 GCHO** est un faux service Minitel/Videotex francais construit avec [GPE](https://github.com/gotoo77/gotoo-pixel-engine). La page Web est le terminal : pas de barre de navigation moderne, pas de boutons HTML autour du framebuffer.

> 3615 GCHO is a fictional service and is not affiliated with any government administration or historic Minitel service.

## MFE-0

Le premier jalon contient une courte sequence de connexion puis sept pages : `ACCUEIL`, `ARCADE`, `MESSAGERIE`, `INFOS`, `GPE`, `NOEUD 7` et `AIDE`. Le clavier reprend des semantiques Minitel simples : chiffres + Entree (`ENVOI`), Retour arriere (`CORRECTION`), Echap (`RETOUR`) et Home (`SOMMAIRE`).

Le contenu editorial est volontairement separe du Rust dans `content/*.json`. Le runtime embarque ces snapshots au build et possede en plus des fallbacks internes afin qu'un snapshot invalide ne rende pas le service inutilisable.

## MFE-1 — profondeur de service

La seconde tranche remplace plusieurs reponses d'une ligne par de vrais sous-ecrans Videotex :

- `ARCADE / CLASSEMENT` ;
- `MESSAGERIE / MA BOITE` ;
- `INFOS / SERVICE PUBLIC` ;
- `GPE / PROJETS` ;
- `NOEUD 7 / LE FICHIER`.

Ces ecrans restent volontairement game-local : un simple `DetailId` complete l'etat existant, sans router generique ni framework de pages. `RETOUR` ferme d'abord le sous-ecran et revient au service parent ; un second `RETOUR` remonte ensuite dans l'historique. `SOMMAIRE` efface tout et retourne a l'accueil.

## Lancer en natif

```bash
cargo run --bin gcho
```

## Verifier

```bash
python3 tools/generate_content.py --check
cargo fmt --all --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release --target wasm32-unknown-unknown --bin gcho-web
```

## Construire le Web

Le workflow Pages utilise Rust `1.97.1` et `wasm-bindgen-cli 0.2.127`, puis assemble `dist/` avec :

```bash
cargo build --release --target wasm32-unknown-unknown --bin gcho-web
wasm-bindgen --target web --out-name gcho-web --out-dir dist/pkg target/wasm32-unknown-unknown/release/gcho-web.wasm
cp web/index.html dist/index.html
```

Servir ensuite `dist/` avec un serveur HTTP local. Le HTML n'utilise que des chemins relatifs, donc le deploiement fonctionne sous `/gpe_3615_gcho/` et ne suppose pas la racine du domaine.

## Contenu externe et regeneration

Snapshots :

- `content/service_public.json`
- `content/news.json`
- `content/messages.json`
- `content/secrets.json`

`tools/generate_content.py` regenere uniquement `service_public.json` a partir de templates locaux approuves. La selection est deterministe pour une date UTC donnee :

```bash
python3 tools/generate_content.py --date 2026-09-15
python3 tools/generate_content.py --check
```

L'ecriture passe par un fichier temporaire puis `os.replace`, donc une erreur avant le remplacement conserve le dernier snapshot valide. Lors du workflow Pages quotidien, une erreur de regeneration produit un avertissement mais le snapshot versionne est ensuite valide et utilise.

Aucune API tierce n'est requise au runtime ou au build des deux premieres tranches.

## GitHub Pages

`.github/workflows/pages.yml` :

1. tente de regenerer le contenu ;
2. valide les snapshots ;
3. compile `gcho-web` en WASM ;
4. genere le glue JavaScript avec `wasm-bindgen` ;
5. assemble `dist/` ;
6. configure ou initialise le site GitHub Pages avec GitHub Actions ;
7. publie l'artifact Pages.

Le workflow se declenche sur `main`, manuellement, et chaque jour a `04:23 UTC`. Le premier deploiement tente d'activer Pages automatiquement via `actions/configure-pages` ; la disponibilite finale reste soumise aux capacites Pages du compte et du depot GitHub.

## Limites actuelles

Pas de vrai protocole Minitel, modem, compte, chat, backend, base de donnees, CMS, analytics, publicite, scraper generique ou contenu LLM live. Les services restent des simulations textuelles volontairement petites.

## Architecture

Le code reste volontairement petit :

- `src/service.rs` : etat, navigation et sous-ecrans ;
- `src/content.rs` : parsing et fallbacks editoriaux ;
- `src/app.rs` : adaptation input GPE + boucle runtime ;
- `src/render.rs` : rendu Videotex framebuffer-only ;
- `tools/generate_content.py` : generation/validation deterministe ;
- `web/index.html` : shell plein ecran minimal.

**Aucune modification de GPE n'est requise.** Les chiffres et commandes d'edition passent par l'API publique `TextInputEvent`, tandis qu'Entree/Echap utilisent les touches physiques deja exposees.
