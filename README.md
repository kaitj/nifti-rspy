# nifti-rspy

This repository serves as a playground for testing out the `nifti` crate
and attempting to interface it with Python via `pyo3` and `maturin`. 

Some (very non-intensive) benchmarks are performed and compared 
against `nibabel`.

```
* nibabel
181 ms ± 6.2 ms per loop (mean ± std. dev. of 5 runs, 5 loops each)

* rust
885 ms ± 39.3 ms per loop (mean ± std. dev. of 5 runs, 5 loops each)
```

## Notes
* On its own, the rust-python interface is slower
  * This may have to do with type casting
  * May also have to uncompressing the gzipped nifti files
* Had to hack the module input a little bit to get it to work
* Would probably do all of the back end computation in
Rust and then convert the final result to be passed back
to Python
