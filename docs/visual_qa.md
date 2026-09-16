# 3615 GCHO — Visual QA matrix

Reference framebuffer: **320x240**.

This checklist is intentionally app-level. A failed check is not a GPE/GPE.UI requirement by itself; promote a need upstream only when the same reusable capability is demonstrated by multiple consumers.

## Reserved vertical bands

- content / notices must finish before `y = 200`
- live status occupies `y = 200..209`
- function keys start at `y = 212`
- Accueil public-service banner occupies `y = 160..197` when no notice is active

## Startup

- [ ] BOOT: all labels and progress bar visible, no clipping
- [ ] BRANDING: logo aspect ratio preserved, nearest-neighbour appearance acceptable
- [ ] Enter/Escape skip each startup phase cleanly
- [ ] AIDE -> REVOIR CONNEXION replays Boot -> Branding -> Accueil

## Main pages

For each page: verify header/page number, selected-row highlight, transmission reveal, live status, function keys, RETOUR/SOMMAIRE/GUIDE and no text under the live-status band.

- [ ] ACCUEIL
- [ ] ARCADE
- [ ] MESSAGERIE
- [ ] INFOS
- [ ] GPE
- [ ] NOEUD 7
- [ ] AIDE

## Detail screens

- [ ] ARCADE / CLASSEMENT
- [ ] ARCADE / PIXEL MAZE
- [ ] MESSAGERIE / MA BOITE
- [ ] MESSAGERIE / SALONS
- [ ] MESSAGERIE / RENCONTRES
- [ ] INFOS / BREVES
- [ ] INFOS / SERVICE PUBLIC
- [ ] INFOS / ALERTES
- [ ] GPE / PROJETS
- [ ] NOEUD 7 / LE FICHIER
- [ ] NOEUD 7 / LES TEMOINS

## Notice/error states

- [ ] short notice fits entirely above live status
- [ ] two-line notice fits entirely above live status
- [ ] notice replaces conflicting lower-page content rather than being painted underneath the status bar
- [ ] function keys remain clickable and visually unobstructed

## Audio / transmission

- [ ] boot cue occurs once per connection replay
- [ ] send/navigation/error cues match the action
- [ ] data-chunk tick stops when the terminal transmission is fully revealed
- [ ] data-chunk tick remains subtle relative to navigation cues

## GPE.UI promotion gate

Record an upstream candidate only when GCHO demonstrates a reusable need also shared by another consumer. Current watch items:

- scrollable/paginated text list
- selectable list layout
- reusable reserved-area/layout primitive

None is promoted by this checklist alone.
