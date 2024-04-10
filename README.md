# markdown-lsp

## Features
- [ ] Autocompletion for references
- [ ] Format document
- [ ] Generate table of content
- TBD

## Local development
Use `cargo watch` to trigger a build everytime a source file or Cargo.toml file changes:
```shell
cargo watch -w markdown-lsp-core/src/ -w markdown-lsp-core/Cargo.toml -w server/src -w server/Cargo.toml -x build
```

Register the debug binary in neovim:
```lua
        local local_markdown_lsp_client = vim.lsp.start_client {
            name = "markdown-lsp",
            cmd = { "<PATH_TO_THE_REPO_DIR>/markdown-lsp/target/debug/server" },
        }

        vim.api.nvim_create_autocmd("FileType", {
            pattern = "markdown",
            callback = function()
                if not local_markdown_lsp_client then
                    vim.notify "client not properly setup for markdown-lsp, maybe it hasn't been compiled ?"
                else
                    vim.lsp.buf_attach_client(0, local_markdown_lsp_client)
                    vim.notify "attached markdown-lsp to this buffer"
                end
            end
        })
    end
```

Logs will be written to a file located at `$HOME/markdown-lsp-logs/markdown-lsp-logs.log`.
