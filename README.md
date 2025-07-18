# Neural Networks WASM Project

## Overview
Projekt symuluje "ptaki uczące się zjadać pożywienie". Używa algorytmu genetycznego i prostych konceptów dotyczących sieci neuronowych. 
Logika ewolucji i uczenia została zaimplementowana w języku Rust i skompilowana do Web-assembly tak, aby projekt można było odpalić w przeglądarce.


## Tech Stack
- **Rust**: Core logic, neural networks, genetic algorithms, simulation
- **wasm-pack**: Builds Rust code to WASM for web consumption
- **JavaScript/TypeScript**: Frontend logic and visualization
- **Webpack**: Bundles frontend assets and WASM glue code
- **HTML5 Canvas**: Visualization of simulation

## Requirements
- [Node.js](https://nodejs.org/) (v16+ recommended)
- [npm](https://www.npmjs.com/) or [yarn](https://yarnpkg.com/)
- [Rust](https://www.rust-lang.org/tools/install) (with [wasm-pack](https://rustwasm.github.io/wasm-pack/))

## How to Run

### 1. Build the WASM package
Navigate to the simulation-wasm library and build the WASM package:
```bash
cd libs/simulation-wasm
wasm-pack build --target web
```
This will generate the WASM glue code in `libs/simulation-wasm/pkg`.

### 2. Install dependencies
Navigate to the frontend directory and install dependencies:
```bash
cd www
npm install
```

### 3. Start the development server
```bash
npm start
```
This will launch a local server (usually at http://localhost:8080/) and open the simulation in your browser.

### 4. Build for production
```bash
npm run build
```

## File Structure
- `libs/genetic-algorithm/` - Genetic algorithm logic (Rust)
- `libs/network/` - Neural network logic (Rust)
- `libs/simulation/` - Simulation logic (Rust)
- `libs/simulation-wasm/` - WASM glue code for simulation (Rust)
- `www/` - Frontend (JavaScript/TypeScript, Webpack, HTML)

## Notes
- If you want to use TypeScript, add it to the frontend (`npm install --save-dev typescript`) and configure `tsconfig.json`.
- Make sure the WASM glue code is built and available in `www/pkg` or update imports accordingly.
- For troubleshooting WASM imports, always use relative paths in browser code (e.g., `import ... from "./pkg/lib_simulation_wasm.js"`).
