# scip-sys
[![][img_crates]][crates] [![][img_doc]][doc]

[img_crates]: https://img.shields.io/crates/v/scip-sys.svg
[crates]: https://crates.io/crates/scip-sys
[img_doc]: https://img.shields.io/badge/rust-documentation-blue.svg
[doc]: https://docs.rs/scip-sys/

Raw rust bindings to [SCIP](https://scipopt.org/)'s C-API. The bindings are automatically generated using [bindgen](https://github.com/rust-lang/rust-bindgen). 
Meant to provide full control over SCIP's API, for a more restricted memory-safe API see [russcip](https://github.com/scipopt/russcip).

## Dependencies 
This crate depends on SCIP at runtime, as of version 0.1.6 we provide the `bundled` feature that tries to download a precompiled binary for your OS and architecture
to enable it add the following to your `Cargo.toml` file
```toml
[dependencies]
scip-sys = { version = "0.1.6", features = ["bundled"] }
```

If the `bundled` feature is not enabled, will look for a scip installation in the current conda environment, if it is not found it will look for the `SCIPOPTDIR` environment variable.
to install SCIP using conda run the following command
```bash
conda install --channel conda-forge scip
```

## License
This repo is distributed under the open-source Apache 2.0 [license](https://www.apache.org/licenses/LICENSE-2.0). Although, to simplify the building process the C-headers for the SCIPOptSuite and its dependent software are included.
These dependencies include *Bliss* that is distributed under the GNU Lesser General Public [license](http://www.gnu.org/licenses/). 
