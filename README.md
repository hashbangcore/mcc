# Netero

A Rust CLI LLM assistant made for terminal-first workflows, aimed at advanced users.

**Also in Spanish:** [Ver README en español](docs/lang/readme.es.md)

## Project status

Experimental. Features are incomplete and may change.

## Environment variables

Configure via environment variables.

Default provider (`codestral`):

* `CODE_API_KEY`
  API key for the default provider.

Custom provider (OpenAI-compatible):

* `NETERO_URL`
  Chat completions endpoint URL.

* `NETERO_MODEL`
  Model name sent to the provider.

* `NETERO_API_KEY`
  Optional API key for the custom provider.

`NETERO_URL` and `NETERO_MODEL` must be set together. If both are set, they override the default provider.

## Usage

CLI to interact with language models.

```
Usage: netero [OPTIONS] [PROMPT] [COMMAND]
```

If input is provided via `stdin`, it is added as extra context.
Expressions like `$(...)` are executed by your shell and their output is appended to the prompt.

### Commands

* `chat`
  Open a minimal chat session

* `commit`
  Generate a commit message from staged changes

* `completion`
  Generate shell completion scripts

* `prompt`
  Send a prompt and print the response

#### Chat commands

* `/help`
  Show help

* `/clean`
  Clear chat history

* `/trans`
  Translate text

* `/eval`
  Evaluate a arithmetic expression

### Arguments

* `[PROMPT]`
  Prompt sent to the model

### Options

* `-v, --verbose`
  Enable verbose output

* `-h, --help`
  Print help

* `-V, --version`
  Print version

## Examples

### Prompts with and without quotes

```sh
netero Explain the difference between hard links and symbolic links
netero "Explain the difference between hard links and symbolic links"
netero -v Explain the Rust ownership model
netero -v "Explain the Rust ownership model"
```

### Using stdin for longer prompts

```sh
cat README.md | netero Summarize the project README
cat README.md | netero "Summarize the project README"
```

### Generate a Git commit message

```sh
netero commit | git commit -F - --edit
```

### Generate shell completion

```sh
netero completion bash
```

### Using a custom provider

```sh
export NETERO_URL="https://api.example.com/v1/chat/completions"
export NETERO_MODEL="my-model"
export NETERO_API_KEY="your-api-key"
netero Explain how systemd manages services
```

### Verbose output

```sh
netero -v Explain the Rust ownership model
netero -v "Explain the Rust ownership model"
```

### Run inline commands inside chat

```sh
netero chat
# then type:
Summarize this directory: $(ls -la)
```

### Chat commands

```sh
netero chat
# then type:
/help
/eval 2 + 2 * 5
/trans This is a test
/clean
```

### Process a man page

```sh
man tmux | netero How can I split a tmux window?
```

### Analyze command output

```sh
ps aux | netero Which processes are consuming the most resources?
```

### Pipe output to another command
```sh
ss -tulpen | netero Summarize active listening sockets | glow -p
```

## License

BSD 2-Clause
