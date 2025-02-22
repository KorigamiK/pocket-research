<div class="oranda-hide" align="center">

# Research Pocket 🔖

</div>

<div align="center">
  <strong> The <em>last</em> save-it-later tool you'll ever need </strong>
</div>
<br />
<div align="center">
  <!-- Github Actions -->
  <a
    href="https://github.com/ResearchPocket/ResearchPocket/actions/workflows/ci-biulds.yml"
  >
    <img
      alt="GitHub Actions Workflow Status"
      src="https://img.shields.io/github/actions/workflow/status/KorigamiK/ResearchPocket/ci-biulds.yml"
    />
  </a>
  <!-- Version -->
  <a href="https://crates.io/crates/research">
    <img
      src="https://img.shields.io/crates/v/research.svg?style=flat-square"
      alt="Crates.io version"
    />
  </a>
  <!-- Docs -->
  <a href="https://docs.rs/research">
    <img
      src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs"
    />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/research">
    <img
      src="https://img.shields.io/crates/d/research.svg?style=flat-square"
      alt="Download"
    />
  </a>
</div>

<br />

A self-hostable save-it-later tool that integrates with
[getpocket.com](https://getpocket.com) (and others soon). works on the web and
terminal

> [!TIP]
> Find the guide to creating your own static site on GitHub pages here
> [ResearchPocket/my-list](https://github.com/ResearchPocket/my-list) 📚

## How it works

<picture>
  <source
    media="(prefers-color-scheme: dark)"
    srcset="./.github/explainer-dark.png"
  />
  <source
    media="(prefers-color-scheme: light)"
    srcset="./.github/explainer.png"
  />
  <img alt="Hashnode logo" src="./.github/explainer.png" />
</picture>

## Installation

- Get the latest release binary for your desktop through the
  [releases page](https://github.com/KorigamiK/ResearchPocket/releases)

- Using Cargo
  ```sh
  $ cargo install research
  ```

## Generate your site

This requires that you have
[tailwindcss](https://tailwindcss.com/blog/standalone-cli) installed and
available in your `$PATH`

```sh
# Initialize the database
$ research init

# Authenticate with Pocket
$ research pocket auth

# Fetch your articles
$ research fetch

# Generate your site
# add --download-tailwind if you don't have tailwindcss installed in your $PATH
$ research --db ./research.sqlite generate .

```

## URL Handler

Research Pocket includes a custom URL handler for the `research://` protocol.
This allows you to save web pages directly from your browser using a
bookmarklet.

### Registering the URL Handler

To register the URL handler on your system, use the following command:

```sh
$ research register
```

This will set up the necessary configurations for your operating system to
recognize and handle `research://` URLs.

### Unregistering the URL Handler

If you want to remove the URL handler, use:

```sh
$ research unregister
```

### Bookmarklet

You can use the following bookmarklet to quickly save web pages to Research
Pocket:

```javascript
javascript: (function () {
  var currentUrl = encodeURIComponent(window.location.href);
  var tags = prompt("Enter tags (comma-separated):", "");
  var dbPath = "/path/to/research.sqlite";
  if (tags !== null && dbPath !== null) {
    var encodedTags = encodeURIComponent(tags);
    var encodedDbPath = encodeURIComponent(dbPath);
    var researchUrl =
      `research://save?url=${currentUrl}&provider=local&tags=${encodedTags}&db_path=${encodedDbPath}`;
    window.location.href = researchUrl;
  }
})();
```

To use this bookmarklet:

1. Create a new bookmark in your browser.
2. Set the name to something like "Save to Research Pocket".
3. In the URL or location field, paste the above JavaScript code.
4. Replace `/path/to/research.sqlite` with the actual path to your Research
   Pocket database.

Now, when you click this bookmarklet on any web page, it will prompt you for
tags and then save the page to your Research Pocket

## Adding Notes

You can add notes to any saved item locallh the `notes` command:

```sh
# Add or update notes for an item
$ research notes <url> "Your notes here"
```

Notes will be displayed when listing items and included in CSV exports. For existing databases, the notes feature will be automatically enabled the next time you run any command.

## Contributing

We welcome contributions to ResearchPocket! If you're interested in helping out,
here are a few ways you can contribute:

- Reporting bugs
- Suggesting enhancements
- Writing documentation
- Submitting pull requests

We participate in Hacktoberfest! During October, we encourage contributors to
join us in improving ResearchPocket. Look for issues tagged with `hacktoberfest`
to get started.

For more detailed information about contributing to this project, including our
Hacktoberfest participation, please check out our
[CONTRIBUTING.md](CONTRIBUTING.md) guide.

## Cli help

- Basic Help

  ```sh
  RESEARCH 🔖

  Manage your reading lists and generate a static site with your saved articles.

  Usage: research [OPTIONS] [COMMAND]

  Commands:
    pocket    Pocket related actions
    local     Add a new item to the database stored locally
    fetch     Gets all data from authenticated providers
    list      Lists all items in the database
    init      Initializes the database
    generate  Generate a static site
    export    Export data from the current database
    handle    Handle operations related to the research:// URL scheme
    notes     Add or update notes for an item
    help      Print this message or the help of the given subcommand(s)

  Options:
        --db <DB>   Database url [env: DATABASE_URL=] [default: ./research.sqlite]
    -d, --debug...  Turn debugging information on
    -h, --help      Print help
    -V, --version   Print version
  ```

- List

  ```sh
  Lists all items in the database

  Usage: research list [OPTIONS]

  Options:
    -t, --tags <TAGS>...       Filter by tags separated by commas Example: --tags rust,sql
    -l, --limit <LIMIT>        Limit the number of items to display
    -f, --favorite-only        Favorite items only (Default: false)
        --timezone <TIMEZONE>  Optional timezone (e.g., "America/New_York", "UTC")
    -h, --help                 Print help
  ```

- Init

  ```sh
  Initializes the database

  Usage: research init <PATH>

  Arguments:
    <PATH>

  Options:
    -h, --help  Print help
  ```

- Local

  ```sh
  Add a new item to the database stored locally

  Usage: research local <COMMAND>

  Commands:
    add       Add an item to the local provider in the database
    list      List all items in the local provider
    favorite  Mark an item as favorite in the local provider
    help      Print this message or the help of the given subcommand(s)

  Options:
    -h, --help  Print help
  ```

- Pocket

  ```sh
  Pocket related actions

  Usage: research pocket <COMMAND>

  Commands:
    auth      Authenticate using a consumer key
    fetch     Fetch items from pocket
    add       Add an item to pocket
    favorite  Mark an item as favorite in pocket
    help      Print this message or the help of the given subcommand(s)

  Options:
    -h, --help  Print help

  ```

- Fetch

  ```sh
  Gets all data from authenticated providers

  Usage: research fetch

  Options:
    -h, --help  Print help
  ```

- Generate

  Here's an example of how to generate a static site:

  ```sh
  $ research --db <path/to/research.sqlite> generate --assets <path/to/assets> <path/to/output>
  ```

  Optionally add `--download-tailwind` to download and reuse the `tailwindcss`
  binary in the assets directory.

  ```sh
  Generate a static site

  Usage: research generate [OPTIONS] <OUTPUT>

  Arguments:
    <OUTPUT>  The path to the output directory

  Options:
        --assets <ASSETS>      Path to required site assets (main.css, search.js, tailwind.config.js) [default: ./assets]
        --download-tailwind    Download Tailwind binary to <ASSETS>/tailwindcss if not found
        --timezone <TIMEZONE>  Optional timezone (e.g., "America/New_York", "UTC")
    -h, --help                 Print help

  ```
