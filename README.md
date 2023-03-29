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

### Advanced usage
 - Use globbing to target more files `$ plantaznik '**/*.md'`. The links are resolved relative to the current markdown file.
 - Increase verbosity with repeated `v` swtich `$ plantaznik README.md -vvvv`. Error (default), Warning, Info, Trace. Mute output with `-q`
 - `--help` to see usage and options
 - Declarations Markdown codeblocks are automatically skipped

#### Preservation mode
Are you not happy with the default generated links? Run once and modify the generated line to your liking.

If the replacement line already contains a PlantUML link (duck-regex-typed), only the encoded source code will be replaced. Use this to your adventage by customizing the image URL placement, alt text or switching to PNG formats. The given line must not contain links to other diagrams.


When running for the first time, the following code is produced: `![](https://www.plantuml.com/plantuml/svg/ENCODED_SOURCE)`, here are some examples on how you can change the line and still get the code synchronization:
 - `![](https://www.plantuml.com/plantuml/png/ENCODED_SOURCE)` - change the image to PNG
 - `![Diagram](https://www.plantuml.com/plantuml/svg/ENCODED_SOURCE)` - add alt text
 - `![](https://mycustomserver.com/plantuml/svg/ENCODED_SOURCE)` - change the server
 - `<div><img src="https://www.plantuml.com/plantuml/png/ENCODED_SOURCE"></div>` - use custom markup
 - `[![](https://www.plantuml.com/plantuml/svg/ENCODED_SOURCE)](https://www.plantuml.com/plantuml/uml/ENCODED_SOURCE)` - add hyperlink to edit on public server (uses the link twice)

## TODO
 - [ ] Add check mode + status codes
 - [ ] Bail option


## Alternatives

 - [puml-for-markdown](https://github.com/danielyaa5/puml-for-markdown) - Lot of features, including the url shortening. [I had troubles running it myself for some reason](https://github.com/danielyaa5/puml-for-markdown/issues/9). The advantage of plantaznik are in my opinion speed, simplicity, better file filtering (glob support) and relative path handing (not always towards to root, but relative to file).
 - [Workaround for public repos on GitHub](https://stackoverflow.com/a/32771815)
 - [GitLab support](https://docs.gitlab.com/ee/administration/integration/plantuml.html)
 - [Mermaid on GitHub](https://github.blog/2022-02-14-include-diagrams-markdown-files-mermaid/)