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

## MFE-2 — canal editorial externe optionnel

`INFOS / BREVES` consomme desormais `content/news.json`. Le snapshot peut etre remplace au moment du deploiement par **un unique feed JSON HTTPS explicitement configure**. Le runtime GPE ne fait aucun appel reseau.

Le fetcher `tools/fetch_external_feed.py` impose :

- HTTPS uniquement, sans identifiants dans l'URL ;
- reponse bornee a 64 Kio ;
- maximum 8 messages ;
- maximum 5 lignes par message ;
- texte normalise et borne a 46 caracteres par champ/ligne ;
- validation stricte avant remplacement ;
- ecriture atomique du snapshot.

Pour activer ce canal, definir dans le depot GitHub une variable Actions nommee `GCHO_FEED_URL` contenant l'URL HTTPS du JSON. Si la variable est absente, ou si le fetch/JSON est invalide, le build conserve simplement `content/news.json` versionne.

Schema attendu :

```json
{
  "generated_at": "2026-09-16T06:00:00Z",
  "messages": [
    {
      "id": "wire-1",
      "category": "breve",
      "title": "TITRE",
      "body": ["LIGNE 1", "LIGNE 2"]
    }
  ]
}
```

## Lancer en natif

```bash
cargo run --bin gcho
```

## Verifier

```bash
python3 tools/generate_content.py --check
python3 -m unittest discover -s tools -p 'test_*.py'
cargo fmt --all --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release --target wasm32-unknown-unknown --bin gcho-web
```

Pour tester le feed sans reseau :

```bash
python3 tools/fetch_external_feed.py --source-file mon_feed.json --output /tmp/gcho_news.json
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

`tools/generate_content.py` regenere `service_public.json` a partir de templates locaux approuves. La selection est deterministe pour une date UTC donnee :

```bash
python3 tools/generate_content.py --date 2026-09-15
python3 tools/generate_content.py --check
```

`tools/fetch_external_feed.py` ne modifie `news.json` qu'apres telechargement, parsing et normalisation reussis. Une erreur conserve donc le dernier snapshot valide.

## GitHub Pages

`.github/workflows/pages.yml` :

1. regenere le contenu local ;
2. tente le feed externe si `GCHO_FEED_URL` est configure ;
3. valide les snapshots ;
4. compile `gcho-web` en WASM ;
5. genere le glue JavaScript avec `wasm-bindgen` ;
6. assemble `dist/` ;
7. publie l'artifact Pages.

Le workflow se declenche sur `main`, manuellement, et chaque jour a `04:23 UTC`.

**Activation initiale requise :** GitHub a confirme en CI que le `GITHUB_TOKEN` du workflow peut construire le site mais ne peut pas creer le site Pages du depot (`Resource not accessible by integration`). Une seule activation manuelle est donc necessaire dans **Settings > Pages**, avec la source **GitHub Actions**. Apres cette activation, les deploiements restent automatiques.

## Limites actuelles

Pas de vrai protocole Minitel, modem, compte, chat, backend, base de donnees, CMS, analytics, publicite, scraper generique ou contenu LLM live. Le canal externe est volontairement un feed JSON unique et borne, pas une plateforme d'ingestion generique.

## Architecture

Le code reste volontairement petit :

- `src/service.rs` : etat, navigation et sous-ecrans ;
- `src/content.rs` : parsing et fallbacks editoriaux ;
- `src/app.rs` : adaptation input GPE + boucle runtime ;
- `src/render.rs` : rendu Videotex framebuffer-only ;
- `tools/generate_content.py` : generation locale deterministe ;
- `tools/fetch_external_feed.py` : ingestion build-time optionnelle et bornee ;
- `web/index.html` : shell plein ecran minimal.

**Aucune modification de GPE n'est requise.** Les chiffres et commandes d'edition passent par l'API publique `TextInputEvent`, tandis qu'Entree/Echap utilisent les touches physiques deja exposees.
