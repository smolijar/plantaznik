<div align="center">
<img src="https://i.imgur.com/zsSWEnH.png" width="300" />

CLI helper for managing PlantUML diagram sources and their previews in Markdown files


[![](https://flat.badgen.net/github/checks/grissius/plantaznik)](https://github.com/grissius/plantaznik/actions/workflows/test.yaml?query=branch%3Amaster++)
[![](https://flat.badgen.net/crates/v/plantaznik)](https://crates.io/crates/plantaznik)
</div>

---

## Motivation

PlantUML is a great tool for managing software spec diagrams. Unless you are on GitLab, which has [built-in support](https://docs.gitlab.com/ee/administration/integration/plantuml.html) for interpreting PlantUML diagram sources you need to get [hacky and inconvenient](https://stackoverflow.com/a/32771815) and even more so for private repos.

Plantaznik is a tool you can integrate into your workflow, that does the menial job for you: find referenced PlantUML sources, generate links and update in the Markdown docs.

<!-- plantaznik:./docs/flow.plantuml -->
![](https://www.plantuml.com/plantuml/svg/pLDFRu8m5B_tKzHkoRO3hP8jMGZ-szHj1xD4BqkCXqeB8m8bfSx4nNtsFK0PB1uysWFXyNZl_Vwy3ZB3jTd44TBICpm98edJ3kOP9YlfAoq9lah3P8s_4ORZXv3N7Rmeelnqkrqy6mz7qp6D1Qe-Mt2gW11AAI6rs6tzM1PE8vuOlal2jKEPK5x6HCW3pUEC-KhH8WtGe5clLkrwAbBQUTU0LiwaW5nhiRpjz_jtZHb_CvSbTXK9gR4esJNdaGE_YgF_d-CHqCQ0LgTf0Z0XUrK8ywlt9xN2LWyFez2J9tzmMREuXBn7UuGyhaLDgWEkud63tJ0Ed4zZqinXBAGGRJJrLCn6ZxE7wUIUfabGTWKEX5pN6u4wQ8dgnjk5DDmtKkU-yqW6MigaORvz2UQ_vB0EnX_PRxVRgggGu1Ymx2jWxRVdjfdisdFp-Z4N0EcyPR7FMXRxOfO5xYu8LXCNrGmlCZP20vc8_Ch_0W00)

## Getting started

### Installation
```sh
$ cargo install plantaznik
```

Alternatively download a precompiled version from [releases](https://github.com/grissius/plantaznik/releases).

### How to use

In your Markdown files, include the following declarations in the comment:
```html
<!-- plantaznik:./path/to/plantuml/source.plantuml -->
(this line will be replaced)
```

The declarations include a path to the source file, which is relative to the current file.

```sh
$ plantaznik README.md
```
All targeted input files (assume utf8 encoded markdown files with `\n` lines) are processed and the lines following the declerations are replaced by a Markdown syntax image, pointing to official PlantUML server with your source code encoded in the link.

Example of verbose output:
```sh
$ plantaznik README.md -vvv
[DEBUG] Replacement README.md:4
[DEBUG] - ![]()
[DEBUG] + ![](https://www.plantuml.com/plantuml/svg/SyfFKj2rKt3CoKnELR1Io4ZDoSa70000)
[DEBUG] Replacement README.md:42 (no change)
[WARN ] Replacement README.md:93: Error accessing file: Read ./missing-diagram.plantuml (caused by: No such file or directory (os error 2))
[INFO ] File README.md processed (2/3 successful replacements)
```

## TODO

 - [x] Clean up readme
 - [x] Add example diagrams to readme (_taste your own champagnes_)
 - [x] Improve debug logging
 - [x] Add dry run
 - [ ] Add check + status codes
 - [ ] Add anyhow
 - [ ] Add switch for output (svg/png)
 - [ ] Add img alts
 - [x] Continue on error
 - [ ] Skip comments in code blocks
 - [x] Add license
 - [ ] Add related projects
 - [x] Add pipeline + publish
