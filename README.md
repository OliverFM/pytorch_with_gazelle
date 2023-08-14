To update requirements and generate BUILD.bazel files run:

```
bazel run //:requirements.update
bazel run //:gazelle_python_manifest.update
bazel run //:gazelle
```

or: `make generate_build_files`

Then go forth and run your targets to your heart's content. 
E.g. ` bazel run //test_project:test_project_bin`
