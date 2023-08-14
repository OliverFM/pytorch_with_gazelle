generate_build_files:
	bazel run //:requirements.update
	bazel run //:gazelle_python_manifest.update
	bazel run //:gazelle
