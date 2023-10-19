This tests shows various configuration of output directory for generated parsers
and actions when the generator is called from `build.rs` script.

By default, everithing is generated in Cargo `OUT_DIR` folder. But in this case we make a different configurations.

1. For grammar `out_dir.rustemo` the generator is configured to generate both
   parser and actions in the this directory.
2. For grammar `our_dir_act.rustemo` the generator is configured to generate
   only actions in this directory while the parser is still generated in
   `OUT_DIR`.
