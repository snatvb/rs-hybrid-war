# Development

```bash
$ cd www
```

```bash
$ yarn install
```

```bash
$ yarn start
```

Move to `localhost:3000`

## rust-analyzer

Need add to project settings:

```json
"rust-analyzer.checkOnSave.extraArgs": ["--target=wasm32-unknown-unknown"],
```
