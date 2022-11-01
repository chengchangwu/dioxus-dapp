# Dioxus-dapp

Scaffolding for a dapp build on Dioxus

```
npm install
npx swc -o components.js ts/*
# npx webpack --mode=production
npx webpack --mode=development
npx tailwindcss -i ./input.css -o ./output.css --watch
trunk serve
```