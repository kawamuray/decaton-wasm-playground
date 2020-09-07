decaton-wasm-experiment
=======================

This is a playground to play with experimental WebAssembly support for [Decaton](https://github.com/line/decaton).

How to play
===========

See or hit `bash build-all.sh`.

Running Decaton with wasm processors
====================================

In `./decaton/wasmton` directory:

```sh
@ decaton/wasmton
$ ../gradlew --no-daemon run --args "$BOOTSTRAP_SERVERS $TOPIC /path/to/processor.wasm"
```

License
=======

Every original things in this repository are licensed under Apache License, Version 2.0.
Anything else in this repository belongs to its original source of distribution and inherits their license.
