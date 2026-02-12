# Netero

A command-line LLM assistant written in Rust, designed for terminal-centric workflows.

**Also available in Spanish:** [Ver README en español](docs/lang/readme.es.md)

## Project status

Netero is experimental software. Features are incomplete and subject to change.

## Environment variables

Netero can be configured dynamically via environment variables.

Default provider:

* `CODE_API_KEY`
  API key used for the default `codestral` provider.

Custom provider (OpenAI-compatible):

* `NETERO_URL`
  Full chat completions endpoint URL.

* `NETERO_MODEL`
  Model name sent to the provider.

* `NETERO_API_KEY`
  Optional API key for the custom provider.

`NETERO_URL` and `NETERO_MODEL` must be set together. If both are set, they override the default provider.

## Usage

CLI for interacting with language models.

```
Usage: netero [OPTIONS] [PROMPT] [COMMAND]
```

If input is provided via `stdin`, it will be used as additional context for the prompt.

### Commands

* `chat`
  Open a minimal chat session

* `commit`
  Generate a commit message from staged changes

* `prompt`
  Send a prompt to the language model and print the response

### Arguments

* `[PROMPT]`
  Prompt passed to the language model

### Options

* `-v, --verbose`
  Enable verbose output

* `-h, --help`
  Print help

* `-V, --version`
  Print version

## Examples

### Basic prompt

```sh
netero "Explain the difference between hard links and symbolic links"
```

### Using stdin for longer prompts

```sh
cat README.md | netero "Summarize the project README"
```

### Generate a Git commit message

```sh
netero commit | git commit -F - --edit
```

### Using a custom provider

```sh
export NETERO_URL="https://api.example.com/v1/chat/completions"
export NETERO_MODEL="my-model"
export NETERO_API_KEY="your-api-key"
netero "Explain how systemd manages services"
```

### Verbose output

```sh
netero -v "Explain the Rust ownership model"
```

### Process a man page

```sh
man tmux | netero "How can I split a tmux window?"
```

### Analyze command output

```sh
ps aux | netero "Which processes are consuming the most resources?"
```

### Pipe output to another command
```sh
ss -tulpen | netero "Summarize active listening sockets" | mdless
```

## License

BSD 2-Clause
