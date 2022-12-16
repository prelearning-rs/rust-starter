# rust-starter

This repository is used as a project starter and upstream for general
project-level configurations across the prelearning-rs ecosystem.

## Usage

To use this repository, new projects must be created from its history.
This cannot be a template repository however, as history is not preserved
and we will not be able to use this repository as an upstream.

### 1. Clone this repository into a new folder

```bash
git clone https://github.com/prelearning-rs/rust-starter.git foo
cd foo
```

### 2. Set the upstreams

```bash
git remote rename origin template
git remote set-url --push template no_push

git remote set-url origin <new-repository-remote>
```

### 3. Sync changes

```bash
git fetch
git merge template/main
```

## Acknowledgements

* [Jon Gjengset (jonhoo)](https://github.com/jonhoo) -- The GitHub actions are largely inspired by the actions in their repositories.
