# neo-papyrus

End the era of paper.

## Setup

### install tailwindcss cli

```bash
cd ~/.local/bin && curl -L https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.6/tailwindcss-linux-x64 -o tw
```

1. You can find all resources for tailwindcss cli here:
https://github.com/tailwindlabs/tailwindcss/releases/tag/v3.4.6

2. Make sure that `~/.local/bin` in your env path

3. Do init:

```bash
tw init
```

This will generate a tailwindcss configuration file called `tailwind.config.js` in current project root dir.
But we've already initialized it.
You don't need to do this work anymore.

## Run

```bash
trunk serve
```

## FAQ
