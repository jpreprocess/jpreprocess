# Tests

This directory contains E2E tests and benchmarks for the project.

## Test data

```
tests/
├── data/
│   ├── dict/                # Test data for E2E tests (not managed within the repository)
│   ├── mecab-naist-jdic/     # OpenJTalk dictionary (not managed within the repository)
│   ├── min-dict/            # Test data for unit tests (managed within the repository)
│   │  ├── orig/            # Minimized dictionary source data
│   │  └── ...              # Minimized dictionary data (generated from the source data)
│   ├── naist-jdic/          # NAIST Japanese dictionary (git submodule, not managed within the repository)
│   └── openjtalk_bin        # OpenJTalk executable (Modified, not managed within the repository)
```

### Test data for unit tests

This directory also contains test data for unit tests. The unit test data are kept minimal and managed within the repository.

#### Minimized dictionary

The data is stored in `./data/min-dict` and are used by unit tests in the `src/` directory.

To rebuild the minimized dictionary (e.g. after updating the serialization format), run the following command in the root of the project:

```bash
cargo run --bin dict_tools --features=binary -- build jpreprocess ./tests/data/min-dict/orig/ ./tests/data/min-dict/
```

### Test data for E2E tests

The data for E2E tests are stored in `./data/dict` and are used by E2E tests in the `tests/` directory.

#### Submodule

The test dictionary is stored in a git submodule. To initialize it, run the following command:

```bash
git submodule update --init --recursive
```

#### JPreprocess dictionary (full)

To run most of the E2E tests, you need to build dictionary from `naist-jdic` using `jpreprocess`. You can do it by running the following command **in the root of the project**:

```bash
cargo run --bin dict_tools --features=binary -- build jpreprocess ./tests/data/naist-jdic/ ./tests/data/dict/
```

#### OpenJTalk dictionary

To run `open_jtalk.rs` test:

1. Download OpenJTalk dictionary from [jpreprocess/jpreprocess v0.0.1-rc0 release](https://github.com/jpreprocess/jpreprocess/releases/download/v0.0.1-rc0/mecab-naist-jdic.tar.xz) and extract it to `./tests/data/mecab-naist-jdic/`.
2. Download OpenJTalk from [jpreprocess/open_jtalk v0.0.5 release](https://github.com/jpreprocess/open_jtalk/releases/download/v0.0.5/openjtalk_bin) and extract it as `./tests/data/openjtalk_bin`.
