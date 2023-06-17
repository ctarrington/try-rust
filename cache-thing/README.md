## Watch and run
find . -name "*.rs" | entr -r -c cargo run --release


# for a million u32 values no index
* no parallelization 71 ms
* with rayon 11 ms or 8 with unstable

# for a million f32 values no index
* with rayon 18 ms

# todo
* add index in tuple, make it like the Java version
* 