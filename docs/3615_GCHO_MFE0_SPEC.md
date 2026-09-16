# 3615 GCHO — MFE-0 contract

3615 GCHO is a fictional French Minitel/Videotex service built with GPE. The browser page is the terminal: no modern navigation chrome, landing page or HTML controls around it.

The MFE must remain deliberately small and game-local. It must provide a short connection sequence, keyboard navigation, the pages ACCUEIL, ARCADE, MESSAGERIE, INFOS, GPE, NOEUD 7 and AIDE, and a restrained Videotex visual language. Navigation uses digits plus ENVOI, CORRECTION, RETOUR and SOMMAIRE semantics.

Editorial content lives outside Rust in small JSON snapshots. At least one category is regenerated periodically by a repository-local deterministic tool. Generation must be atomic and failure must leave a valid committed snapshot usable for the Web build.

The Web/WASM build deploys through GitHub Pages and must work from `/gpe_3615_gcho/`. GPE capabilities are reused as-is; engine changes are forbidden unless a missing capability cannot reasonably be solved game-locally.

MFE-0 explicitly excludes a backend, accounts, real chat, database, CMS, authentic Minitel protocols, generic terminal/router abstractions, live LLM content and engine refactoring.
