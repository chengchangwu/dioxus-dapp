# Dioxus-dapp

Scaffolding for a dapp build on Dioxus

```
npm install
npx swc -o components.js js/*
npx webpack --mode=production
npx tailwindcss -i ./input.css -o ./output.css --watch
trunk serve
```