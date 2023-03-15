<div align="center">
<img src="https://i.imgur.com/zsSWEnH.png" width="300" />

CLI helper for managing PlantUML diagram sources and their previews in Markdown files
</div>

---

## Motivation

PlantUML is a great tool for managing software spec diagrams. Unless you are on GitLab, which has [built-in support](https://docs.gitlab.com/ee/administration/integration/plantuml.html) for interpreting PlantUML diagram sources you need to get [hacky and inconvenient](https://stackoverflow.com/a/32771815) and even more so for private repos.

Plantaznik is a tool you can integrate into your workflow, that does the menial job for you: find referenced PlantUML sources, generate links and update in the Markdown docs.

<!-- plantaznik:./docs/flow.plantuml -->
![](https://www.plantuml.com/plantuml/svg/pLHBJ-Cm4BxxLmpUbNRtK3lDxaghgW_O1HOE5L8vW8HwCB6JMdNio74f3vNVpYHjGeeuSCI7oEF9p7mF9uTb43wiSaE-mR0C3qQDSUTL4eJDZ88dZAerrLRdg_nAox20VXH5pRcurxZTLEMyhJSg3H0SU9qj0aXTzTFEOgPSQbi8BtAGXOQVsETrLj47KGAzLQdpYiANGjib0gZ6f1hplWytTLIjqMW4ivEZu-a9ooLfZtkuse1InXYbdNXuyDphGM643UBHwkM0SUcIaaijCWHLyjGvLgTH4cmsxbGF4sUS7rnuHAjcIcBku6R-VJAP_DYRyJfpliNkZ5GUv9RT_ZowuzToJDzoR02j0dhqFaq49b7s9iJvxVwlkuNU68wCJjIxBmpvVl4MyePi24c4bnsfThZg1vgjysom2A4e1vnhXj7AiyJb_FZFxFVqz1ShRBRjYXme_TfjXEgGEUaQ7zVIY3GeN_akZCgyKdRF_FWpcF-_WhKplc6_Ng_Rgbg29k3bNSQZytzHE7sCBiFPzQL4I1-xBF67BalyQIuBVhieM4TSqZ6ypiI472ehgr_D2m00)

## Getting started

### Installation
```sh
$ cargo install plantaznik
```

### How to use

In your Markdown files, include the following declarations in the comment:
```html
<!-- plantaznik:./path/to/plantuml/source.plantuml -->
EMPTY_OR_PLACEHOLDER_LINE_THAT_WILL_GET_REPLACED
```

The declarations include a path to the source file, which is relative to the current file.

```sh
$ plantaznik '**/*.md'
```
All targeted input files (assume utf8 encoded markdown files with `\n` lines) are processed and the lines following the declerations are replaced by a Markdown syntax image, pointing to official PlantUML server with your source code encoded in the link.

## TODO

 - [x] Clean up readme
 - [x] Add example diagrams to readme (_taste your own champagnes_)
 - [x] Improve debug logging
 - [ ] Add dry run
 - [ ] Add switch for output (svg/png)
 - [ ] Add img alts
 - [x] Continue on error
 - [ ] Skip comments in code blocks
 - [x] Add license
 - [ ] Add related projects
 - [ ] Add pipeline + publish
