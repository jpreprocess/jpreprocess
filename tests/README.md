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

To rebuild the minimized dictionary (e.g. after updating the serialization format), run the following command in the root of the project:

```bash
cargo run -p tests --bin generate_min_dict_src
cargo run -p jpreprocess-dictionary --bin dict_tools --features=binary -- build jpreprocess ./tests/data/min-dict-src/ ./tests/data/min-dict/
```

### JPreprocess dictionary (full)

To run most of the E2E tests, you need to build dictionary from `naist-jdic` using `jpreprocess`. You can do it by running the following command **in the root of the project**:

```bash
cargo run -p jpreprocess-dictionary --bin dict_tools --features=binary -- build jpreprocess ./tests/data/naist-jdic/ ./tests/data/dict/
```

### OpenJTalk dictionary

To run `open_jtalk.rs` test:

1. Download OpenJTalk dictionary from [jpreprocess/jpreprocess v0.0.1-rc0 release](https://github.com/jpreprocess/jpreprocess/releases/download/v0.0.1-rc0/mecab-naist-jdic.tar.xz) and extract it to `./tests/data/mecab-naist-jdic/` (from repo root).
2. Download OpenJTalk from [jpreprocess/open_jtalk v0.0.5 release](https://github.com/jpreprocess/open_jtalk/releases/download/v0.0.5/openjtalk_bin) and extract it as `./tests/data/openjtalk_bin` (from repo root).
