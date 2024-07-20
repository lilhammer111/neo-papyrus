# neo-papyrus

End the era of paper!

## Setup

Rust, system dependencies and tauri is required.

In addition, these environments and tools may also be used:

### tailwindcss

install tailwindcss cli:

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

### tauri-cli

```bash
cargo install tauri-cli
```

## Run

### browser end

```bash
trunk serve
```

### desktop end

```bash
cargo tauri dev
```

## Issue

1. white window

You can try:

```bash
who
```

It may output `1` or `0`.

And then, you set the corresponding env var:
Assuming the output is `1`.

```bash
export DISPLAY=:1
```

2. KMS: DRM_IOCTL_MODE_CREATE_DUMB failed: Permission denied Permission denied

```bash
export WEBKIT_DISABLE_DMABUF_RENDERER=1
```
