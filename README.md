# 3615 GCHO

<p align="center">
  <img src="assets/branding/3615_gcho_logo.png" alt="3615 GCHO — La canicule en réseau" width="600">
</p>

**3615 GCHO** est un faux service Minitel/Videotex francais construit avec [GPE](https://github.com/gotoo77/gotoo-pixel-engine). La page Web est le terminal : pas de barre de navigation moderne, pas de boutons HTML autour du framebuffer.

> 3615 GCHO is a fictional service and is not affiliated with any government administration or historic Minitel service.

## Experience actuelle

Le service contient sept pages principales : `ACCUEIL`, `ARCADE`, `MESSAGERIE`, `INFOS`, `GPE`, `NOEUD 7` et `AIDE`, avec plusieurs sous-ecrans Videotex. L'illusion est volontairement plus importante que l'architecture : pas de backend, pas de compte, pas de chat reel et aucun appel reseau au runtime.

Navigation :

- chiffres `0-9` + Entree (`ENVOI`) ;
- fleches Haut/Bas pour parcourir les choix avec bouclage ;
- fleche Gauche ou Echap pour `RETOUR` ;
- Retour arriere / Suppr pour `CORRECTION` ;
- Home pour `SOMMAIRE` ;
- `H` pour `GUIDE` ;
- grosses touches Minitel visibles et cliquables en bas du terminal.

Le demarrage joue volontairement un vieux handshake de modem, meme si cet effet est historiquement anachronique avec le Minitel. Des bips locaux accompagnent touches, validation, navigation et erreurs.

## MFE-0 — squelette

Premiere boucle navigable GPE, contenu editorial separe dans `content/*.json`, fallbacks internes et builds natif/Web.

## MFE-1 — profondeur de service

Premiers vrais sous-ecrans : `ARCADE / CLASSEMENT`, `MESSAGERIE / MA BOITE`, `INFOS / SERVICE PUBLIC`, `GPE / PROJETS`, `NOEUD 7 / LE FICHIER`.

## MFE-2 — canal editorial externe optionnel

`INFOS / BREVES` consomme `content/news.json`. Le snapshot peut etre remplace au build par **un unique feed JSON HTTPS explicitement configure**. Le runtime GPE ne fait aucun appel reseau.

Le fetcher `tools/fetch_external_feed.py` impose HTTPS, une reponse bornee a 64 Kio, maximum 8 messages, maximum 5 lignes par message, texte normalise, validation stricte et ecriture atomique.

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

## MFE-3 — profondeur supplementaire

Nouveaux sous-ecrans reels : `ARCADE / PIXEL MAZE`, `MESSAGERIE / SALONS`, `INFOS / ALERTES` et `NOEUD 7 / LES TEMOINS`.

## MFE-4 — facade Minitel et audio retro

Ajout des grosses touches pseudo-physiques `SOMMAIRE`, `RETOUR`, `CORRECT.`, `GUIDE`, `ENVOI`, avec feedback visuel et souris. `ENVOI` utilise un traitement vert et `CORRECT.` un traitement jaune. Le sound design est genere localement sous forme de WAV PCM deterministes : boot/modem, touche, envoi, navigation et erreur.

## MFE-5 — debit et navigation directionnelle

Le terminal simule par defaut **1200 bit/s**, soit environ **120 caracteres/s**, avec un petit delai de reponse avant transmission. Les changements de page recommencent une transmission progressive. En natif, le profil peut etre surcharge :

```bash
GCHO_BAUD=600 GCHO_RESPONSE_MS=500 cargo run --bin gcho
```

Sous PowerShell :

```powershell
$env:GCHO_BAUD=600
$env:GCHO_RESPONSE_MS=500
cargo run --bin gcho
```

## MFE-6 — service vivant

Le terminal donne maintenant l'impression qu'un service continue de vivre derriere l'ecran sans reseau runtime :

- le message de service public de l'accueil tourne deterministiquement entre le slogan canonique et les messages versionnes de `content/service_public.json` ;
- une petite barre d'etat fait cycler des evenements de session (`1 NOUVEAU MESSAGE`, activite `NOEUD 7`, alerte administrative) ;
- quand une selection aux fleches est active, cette barre affiche explicitement `SELECTION : N / ENVOI`, meme pendant la transmission lente.

Tout reste local, deterministe et reproductible.

## MFE-7 — bruit de donnees recues

Pendant une transmission, un cue tres court et discret accompagne chaque **chunk logique de 16 caracteres recus**. Le son n'est pas joue pendant le delai de reponse initial et ne se repete pas entre deux frames tant qu'aucune nouvelle frontiere de chunk n'est franchie. Comme les autres sons GCHO, il est genere localement et deterministiquement, sans asset audio externe.

## MFE-8 — branding accueil

L'accueil integre le logo Minitel fourni dans le framebuffer GPE. L'image est embarquee dans le binaire/WASM, decodee une seule fois au demarrage puis rendue en `Contain` avec filtrage `Nearest`. Le branding reste limite a l'accueil ; les autres pages conservent leur header texte compact.

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

## Construire le Web localement

Le workflow utilise Rust `1.97.1` et `wasm-bindgen-cli 0.2.127`. Construction manuelle :

```bash
cargo build --release --target wasm32-unknown-unknown --bin gcho-web
wasm-bindgen --target web --out-name gcho-web --out-dir dist/pkg target/wasm32-unknown-unknown/release/gcho-web.wasm
cp web/index.html dist/index.html
```

Servir ensuite `dist/` avec un serveur HTTP local. Le HTML n'utilise que des chemins relatifs.

Le depot contient encore le workflow `.github/workflows/pages.yml`, mais le deploiement public GitHub Pages est volontairement differe. Le developpement et les validations courantes restent local-first.

## Contenu et regeneration

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

`tools/fetch_external_feed.py` ne modifie `news.json` qu'apres telechargement, parsing et normalisation reussis. Une erreur conserve le dernier snapshot valide.

## Limites actuelles

Pas de vrai protocole Minitel, compte, chat, backend, base de donnees, CMS, analytics, publicite, scraper generique ou contenu LLM live. Le canal externe reste volontairement un feed JSON unique et borne, pas une plateforme d'ingestion generique.

## Architecture

Le code reste volontairement petit :

- `src/service.rs` : etat, navigation et sous-ecrans ;
- `src/content.rs` : parsing et fallbacks editoriaux ;
- `src/experience.rs` : debit simule, chunks, rotation et etat vivant de session ;
- `src/branding.rs` : asset Minitel embarque et composition specifique de l'accueil ;
- `src/app.rs` : adaptation input GPE + boucle runtime ;
- `src/render.rs` : rendu Videotex framebuffer-only ;
- `src/sound.rs` : cues audio locaux ;
- `tools/generate_content.py` : generation locale deterministe ;
- `tools/fetch_external_feed.py` : ingestion build-time optionnelle et bornee ;
- `web/index.html` : shell plein ecran minimal.

**Aucune modification de GPE n'est requise.** L'integration du branding reutilise les primitives image existantes (`Image`, `ImageFit::Contain`, `ImageFilter::Nearest`).
