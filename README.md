# genv

## Overview
`genv` is a command-line tool to generate and/or update a `.env.example` file from an existing `.env` file.

## Installation
To use `genv`, ensure you have Rust installed, then build the project:

```sh
cargo build --release
```

Or just download the .exe and add the folder where it's located to your $PATH environment variable.

## Usage
Run `genv` with the available options:

```sh
genv [OPTIONS]
```

### Options:

| Option                 | Description                                              | Default Value      |
|------------------------|----------------------------------------------------------|--------------------|
| `-i`, `--input`       | Path to the `.env` file                                  | `.env`            |
| `-o`, `--output`      | Path to the `.env.example` file                          | `.env.example`    |
| `-c`, `--omit-comments` | Omit comments from `.env` in the `.env.example` file   | `false`           |
| `-w`, `--overwrite`   | Whether to overwrite the `.env.example` file or update it | `false`           |

### Example Usage:

#### Generate a `.env.example` from `.env`:
```sh
genv
```


#### Specify a custom input and output file:
```sh
genv -i my_env -o my_env_example
```

#### Fancy gif
![Demo of genv](gifs/basic-use.gif)

#### Omit comments from the output file:
```sh
genv -c
```

#### Overwrite the existing `.env.example` file:
```sh
genv -w
```

## TODOs
- Improve handling of key updates on the same line in `.env` files.

## License
This project is licensed under the MIT License.

