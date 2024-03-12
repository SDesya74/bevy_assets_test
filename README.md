This example shows that Bevy does not track changes to files outside of assets when the project in a workspace.

Run with `cargo run -p assets_test_inner -F bevy/file_watcher -F bevy/embedded_watcher`.

By the way, with this run and a missing `assets` folder **inside assets_test_inner** Bevy panics with this error:

```
Failed to create file watcher: Error { kind: Generic("Input watch path is neither a file nor a directory."), paths: [] }
```
