# sav-serialize

Starter code for implimenting serialization/deserialization via binary encoding (`.sav`) using the `serde` and `bincode` libraries. 

## Usage

Run the program:

```bash
cargo run
```

The program will alternate outputing the following content (indicating that data persistance was successful), as well as the save file path:

```text
Person { name: "Tom", age: 32 }
```

```text
Person { name: "Jim", age: 28 }
```
