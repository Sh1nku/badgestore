# Badgestore
![Lines of Code badge](https://api.badgestore.dev/badge/108db75934eb5ec0/local)

This is the front-end and back-end for [badgestore.dev](https://badgestore.dev). The website allows you to create badges, and update them.

There is support for badges in the following formats:
- [Badgestore.dev](https://badgestore.dev) Using [rsbadges](https://gitlab.com/tangram-vision/oss/rsbadges)
- [Shields.io](https://shields.io)
- [Badgen.net](https://badgen.net)

Note: This project is not affiliated with any of the above services/libraries.  
The front-end is written in [Svelte](https://svelte.dev) and the back-end is written using [Axum](https://github.com/tokio-rs/axum).

## How to use
### Creating a badge
#### Using the website
To create a badge, go to [badgestore.dev](https://badgestore.dev) and click on the `Generate` button.
#### Using the API
Take a look at [api.badgestore.dev](https://api.badgestore.dev)
### Updating a badge
#### Using the API
Take a look at [api.badgestore.dev](https://api.badgestore.dev)
#### Using the github action
Take a look at [Sh1nku/badgestore-update-badge-action](https://github.com/Sh1nku/badgestore-update-badge-action)
### Examples
A github action for updating the lines of code of the repo, and updating a badge given a secret called `BADGESTORE_RW_KEY` with the value `read_key:write_key`
```yaml
name: Count lines of code for the project, and upload to the badge store

on:
  push:
    branches:
      - 'master'

jobs:
  count-loc-and-upload:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write

    steps:
      - uses: actions/checkout@v3
      - id: loc
        name: Count lines of code
        uses: Sh1nku/count-loc-action@v1
        with:
          excluded: "*.json,*.svg,*.md"
      - uses: Sh1nku/badgestore-update-badge-action@v1
        name: Update badge
        id: badge
        with:
          right-label: ${{ steps.loc.outputs.Total_code }}
          read-write-key: ${{ secrets.BADGESTORE_RW_KEY }}
      - uses: koki-develop/hub-purge-action@v1
```