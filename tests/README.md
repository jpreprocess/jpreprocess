# Tests

This directory contains E2E tests, benchmarks, and dictionary data for unit tests for the project.

## Test data

```
tests/
├── data/
│   ├── dict/                # Test data for E2E tests (not managed within the repository)
│   ├── mecab-naist-jdic/     # OpenJTalk dictionary (not managed within the repository)
│   ├── min-dict/            # Test data for unit tests (managed within the repository)
│   │  ├── min-dict.csv     # Minimized dictionary source word data
│   │  └── ...              # Minimized dictionary data (generated from min-dict-src/)
│   ├── min-dict-src/        # Minimized dictionary data source (generated from min-dict.csv and naist-jdic)
│   ├── naist-jdic/          # NAIST Japanese dictionary (git submodule, not managed within the repository)
│   └── openjtalk_bin        # OpenJTalk executable (Modified, not managed within the repository)
```

### Submodule

The test dictionary is stored in a git submodule. To initialize it, run the following command:

```bash
git submodule update --init --recursive
```

### Minimized dictionary

The data is stored in `./data/min-dict` and are used by unit tests in the `src/` directory.

To rebuild the minimized dictionary (e.g. after updating lindera), run the following command in the root of the project:

```bash
cargo run -p tests --bin generate_min_dict_src
cargo run -p jpreprocess-dictionary --bin dict_tools --features=binary -- build lindera ./tests/data/min-dict-src/ ./tests/data/min-dict/
```

### Full dictionary

To run most of the E2E tests, you need to build dictionary from `naist-jdic`. Follow one of the steps below to build a full dictionary.

#### JPreprocess dictionary

`jpreprocess`-style dictionary can be built using the following command. You can do it by running the following command **in the root of the project**:

```bash
cargo run -p jpreprocess-dictionary --bin dict_tools --features=binary -- build jpreprocess ./tests/data/naist-jdic/ ./tests/data/dict/
```

#### Lindera dictionary

If you prefer lindera dictionary, where the fields (part of speech, pronunciation, etc.) are encoded as strings, you can build it using the following command although it is a little hacky:

```bash
mkdir -p ./tests/data/dict/jpreprocess-tmp/
cargo run -p jpreprocess-dictionary --bin dict_tools --features=binary -- build jpreprocess ./tests/data/naist-jdic/ ./tests/data/dict/jpreprocess-tmp/
cargo run -p jpreprocess-dictionary --bin dict_tools --features=binary -- build lindera ./tests/data/naist-jdic/ ./tests/data/dict/
cmp ./tests/data/dict/jpreprocess-tmp/dict.da ./tests/data/dict/dict.da && echo "Dictionaries are identical" || echo "Dictionaries differ"
cp ./tests/data/dict/jpreprocess-tmp/dict.vals ./tests/data/dict/dict.vals
rm -rf ./tests/data/dict/jpreprocess-tmp/
```

The `dict.vals` file is reversed in the `jpreprocess`-style dictionary for backward compatibility (Double array is expected to be identical).

### OpenJTalk dictionary

To run `open_jtalk.rs` test:

1. Download OpenJTalk dictionary from [jpreprocess/jpreprocess v0.0.1-rc0 release](https://github.com/jpreprocess/jpreprocess/releases/download/v0.0.1-rc0/mecab-naist-jdic.tar.xz) and extract it to `./tests/data/mecab-naist-jdic/` (from repo root).
2. Download OpenJTalk from [jpreprocess/open_jtalk v0.0.5 release](https://github.com/jpreprocess/open_jtalk/releases/download/v0.0.5/openjtalk_bin) and extract it as `./tests/data/openjtalk_bin` (from repo root).
