Comparing the execution speed between Web Assemby and Javascript. Built a prime number checker app with wasm and js for that.

To run, ensure you have rust, node and npm installed. After cloning,

1. Install wasm-pack if you haven't by running 
```
cargo install wasm-pack
```

2. Inside the root of the project, to compile rust to web assembly, run
```
npm run build-wasm
```

3. To start up the application, run
```
npm run dev
```
