# uvup Documentation

Documentation site for uvup, built with [VitePress](https://vitepress.dev).

## Development

Install dependencies:

```bash
npm install
```

Start development server:

```bash
npm run dev
```

Build for production:

```bash
npm run build
```

Preview production build:

```bash
npm run preview
```

## Structure

```
docs/
├── .vitepress/
│   └── config.mts          # VitePress configuration
├── guide/                  # English documentation
│   ├── installation.md
│   ├── quick-start.md
│   └── core-concepts.md
├── commands/               # Command reference
├── use-cases/              # Usage examples
├── zh/                     # Chinese documentation
│   ├── guide/
│   ├── commands/
│   └── use-cases/
└── index.md                # Homepage
```

## Deployment

The documentation is automatically deployed to GitHub Pages when changes are pushed to the `main` branch.
