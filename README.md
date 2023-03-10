# scip-sys
Raw rust bindings to [SCIP](https://scipopt.org/)'s C-API. The bindings are automatically generated using [bindgen](https://github.com/rust-lang/rust-bindgen). 
Meant to provide full control over SCIP's API, for a more restricted memory-safe API see [russcip](https://github.com/scipopt/russcip).

## Dependencies 
This crate depends on SCIP at runtime, the easiest way to install it is to install a precompiled package from [here](https://scipopt.org/index.php#download) or through conda by running
```bash
conda install --channel conda-forge scip
```
After which `scip-sys` would be able to find the installation in the current Conda environment. Alternatively, you can specify the installation directory through the `SCIPOPTDIR` environment variable. 


## License
This repo is distributed under the open-source Apache 2.0 [license](https://www.apache.org/licenses/LICENSE-2.0). Although, to simplify the building process the C-headers for the SCIPOptSuite and its dependent software are included.
These dependencies include *Bliss* that is distributed under the GNU Lesser General Public [license](http://www.gnu.org/licenses/). 
