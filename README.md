# Althread

Althread is an open source Promela alternative for modeling and verifying multi-threaded systems.

## Key Features
- Simple modeling of concurrent systems.
- Automatic detection of race conditions and deadblocks.
- Advanced debugging tools.
- Easy-to-learn syntax for beginners.

## Installation
1. Clone the repository: 
   ```
   git clone https://github.com/romainbourdain/althread.git
   ```

2. Navigate to the directory and compile :
    ```
    cd althread/interpreter
    cargo build
    ```

3. Run an example
    ```
    cargo run test.alt
    ```

## Quick Start
Here is a minimal example of modeling a multi-thread system :
```
process A() {
    print("Hello world from A process");
}

main {
    run A();
    print("Hello world from main");
}
```

## Full Documentation
- Check out the [full documentation](https://romainbourdain.github.io/althread/) for more examples, guides and a reference of symbols.

## Sources
- https://www.rust-lang.org/fr
- https://pest.rs/
- https://docs.rs/clap/latest/clap/
- https://craftinginterpreters.com/contents.html
- https://github.com/tdp2110/crafting-interpreters-rs?tab=readme-ov-file

## License
MIT License. Contributions are welcome !