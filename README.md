<div align="center">
<img src="https://i.imgur.com/zsSWEnH.png" width="300" />

CLI helper for managing PlantUML diagram sources and their previews in Markdown files


⚡&nbsp;Fast |
💪&nbsp;Powerful&nbsp;file&nbsp;selection |
✅&nbsp;Check&nbsp;mode&nbsp;for&nbsp;CI |
🎨&nbsp;Customizable&nbsp;output |
🪝&nbsp;Ideal&nbsp;for&nbsp;git&nbsp;hooks

[![](https://flat.badgen.net/github/checks/grissius/plantaznik)](https://github.com/grissius/plantaznik/actions/workflows/test.yaml?query=branch%3Amaster++)
[![](https://flat.badgen.net/crates/v/plantaznik)](https://crates.io/crates/plantaznik)
</div>

## Motivation

PlantUML is a great tool for managing software spec diagrams. Unless you are on GitLab, which has [built-in support](https://docs.gitlab.com/ee/administration/integration/plantuml.html) for interpreting PlantUML diagram sources you need to get [hacky and inconvenient](https://stackoverflow.com/a/32771815) and even more so for private repos.


<!-- plantaznik:./docs/steps.plantuml -->
![](https://www.plantuml.com/plantuml/svg/9Ov12i8m44NtESM86rruWHQaOWikB4Z1bRWOcf64fjEIJ5hgwMtZNxx7XrS4GI-nTsedXdg2q96qPb6piOxWS5ImBcleGvwu9NWuP7dMzyJ34FtAB4DJNSgwoXFTb-YxwuASDP5Xt0xuPTlzRNR3YI2E80De54fmz3HPcWCuort85R5E0UhqCbPhulVfh3G9jaOLgzo2Vm00)

Plantaznik is a tool you can integrate into your workflow, that does the menial job for you: find referenced PlantUML sources, generate links and update in the Markdown docs.

<!-- plantaznik:./docs/before-after.plantuml -->
![](https://www.plantuml.com/plantuml/svg/pLF1Re904BtlLwoR9cqFxYffauOWQgkcFPWcUcbYFApiW4HWoRBMrCP-UrSe28q7Z-w1y1XcvhqtiuDCSwMtSOHGIwyX1fnjG7lhBwJ0qpm98i1f5_CCanNuKW71tmYRuuScehfuaQFZwVLwU3OUZgPZ6WjKVMwRKm52AAM4rB3J-cctSHhnHFDz4cvioeJqCYP27YWUPyoNakPXGz8KSslPZYSZgUntPNZ238Jhh4oujLZUzVlz-qQRKzjbGLz60XGMXRHcor9Y9TF_BxFajDL7uyugDTm4k1Kq_QltPxd2RHSFez23ipyuh9bSK3wW0q8ULwBcrX-l_76BdJ0ExBNMQMOp5bA3jefwCcQZfzdZTF90qoGegXeDXDpK2nbtq1BLfsxbrd1VWseEtRg6eQSyWa019AUXbyftkrsLdCil0Si-0zPvU-deoRuprwyVSs6ONROeraMBObtDefXBQtohUOngmyuZ5a83ICJnxly2)

## Installation
```sh
$ cargo install plantaznik
```

Alternatively download a precompiled version from [releases](https://github.com/grissius/plantaznik/releases).

## Getting started

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
[ERROR] Replacement README.md:93: Error accessing file: Read ./missing-diagram.plantuml (caused by: No such file or directory (os error 2))
[INFO ] File README.md processed (2/3 successful replacements)
```

## Usage

```
CLI helper for managing PlantUML diagram sources and their previews in Markdown files

Usage: plantaznik [OPTIONS] <GLOB>

Arguments:
  <GLOB>  README files glob pattern to process

Options:
  -d, --dry-run     Dry run (skips file writes)
  -c, --check-only  Check (skips file writes and error if updates would take place)
  -v, --verbose...  More output per occurrence
  -q, --quiet...    Less output per occurrence
  -h, --help        Print help
  -V, --version     Print version
```

### Advanced usage
 - Use globbing to target more files `$ plantaznik '**/*.md'`. The links are resolved relative to the current markdown file.
 - Increase verbosity with repeated `v` swtich `$ plantaznik README.md -vvvv`. Error (default), Warning, Info, Trace. Mute output with `-q`
 - `--help` to see usage and options

### Preservation mode
Are you not happy with the default generated links? Run once and modify the generated line to your liking.

If the replacement line already contains a PlantUML link (duck-regex-typed), only the encoded source code will be replaced. Use this to your adventage by customizing the image URL placement, alt text or switching to PNG formats. The given line must not contain links to other diagrams.


When running for the first time, the following code is produced: `![](https://www.plantuml.com/plantuml/svg/ENCODED_SOURCE)`, here are some examples on how you can change the line and still get the code synchronization:
 - `![](https://www.plantuml.com/plantuml/png/ENCODED_SOURCE)` - change the image to PNG
 - `![Diagram](https://www.plantuml.com/plantuml/svg/ENCODED_SOURCE)` - add alt text
 - `![](https://mycustomserver.com/plantuml/svg/ENCODED_SOURCE)` - change the server
 - `<div><img src="https://www.plantuml.com/plantuml/png/ENCODED_SOURCE"></div>` - use custom markup
 - `[![](https://www.plantuml.com/plantuml/svg/ENCODED_SOURCE)](https://www.plantuml.com/plantuml/uml/ENCODED_SOURCE)` - add hyperlink to edit on public server (uses the link twice)

### Notes
 - Declarations Markdown codeblocks are automatically skipped

### Using in CI

You can use the check mode (`--check-only`) to make sure all diagrams are in sync. Copy the GitHub Actions workflow directly ([plantaznik.yaml](https://github.com/grissius/plantaznik/tree/master/.github/workflows/plantaznik.yaml), change the glob pattern if needed) or use it as an inspiration for your CI.

You can preview [example runs on this repo](https://github.com/grissius/plantaznik/actions/workflows/plantaznik.yaml).

## Roadmap

Currently, I am happy with the features and functionality, if there is something that you would like to see, feel free to open an issue! 

## Alternatives

 - [puml-for-markdown](https://github.com/danielyaa5/puml-for-markdown) - Lot of features, including the url shortening. [I had troubles running it myself for some reason](https://github.com/danielyaa5/puml-for-markdown/issues/9). The advantage of plantaznik are in my opinion speed, simplicity, better file filtering (glob support) and relative path handing (not always towards to root, but relative to file).
 - [Workaround for public repos on GitHub](https://stackoverflow.com/a/32771815)
 - [GitLab support](https://docs.gitlab.com/ee/administration/integration/plantuml.html)
 - [Mermaid on GitHub](https://github.blog/2022-02-14-include-diagrams-markdown-files-mermaid/)