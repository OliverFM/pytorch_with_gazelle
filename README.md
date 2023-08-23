To update requirements and generate BUILD.bazel files run:

```
bazel run //:requirements.update
bazel run //:gazelle_python_manifest.update
bazel run //:gazelle
```

or: `make generate_build_files`

Then go forth and run your targets to your heart's content. 
E.g. ` bazel run //test_project:test_project_bin`

## PyO3

The PyO3 Support in this Repo is there, but it is pretty sketchy.
The Crux of the matter: one needs to build one's own versions of `pyo3`
with specific compiler flags for one's python version. It is particularly
important to compile `pyo3-ffi` and `pyo3-macros` as, without compiling them
in bazel, I was not able to get the right proc macros I needed. Most notably
`#[pyfunction], #[pymodule] & #[pyfn]`.

Lastly, one needs to make sure that the `crate_name` of the `rust_shared_library`
is the same as the name of the `pymodule` defined in the rust code. Else it does
not import.

Time permitting I may convert this into a `rules_pyo3` repo that can more easily be reused.
