# Dioxus-dapp

Scaffolding for a dapp build on Dioxus - ported from [dapp-scaffoid](https://github.com/solana-labs/dapp-scaffold)

```
npm install
cp node_modules/@solana/wallet-adapter-react-ui/styles.css ./wallet-styles.css
npx swc -o components.js ts/*
npx tailwindcss -i ./input.css -o ./output.css --watch

dioxus serve
```

## Screenshot

![image info](./screenshot.png)
