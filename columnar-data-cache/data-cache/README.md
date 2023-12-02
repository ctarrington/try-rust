# data-cache

## Todo
* use constructor for cache X
    * and methods to add columns X
    * fill in default values if existing rows? X
    * if default value is not in allowed values then add it X


* return the guid in the add row method X 
* allow replacement of row by guid X

-----------------


* Improve error handling
    * different error types for different errors
        * parse error X
        * invalid guid X
        * implement fmt::Display for error type and From for error types X
        * store source in From and expose in Error and Display X
        * duplicate column name X 
        * index error in column storage should be part of cache error X
        * store guid in Error and expose in Error and Display X
        * prevent duplicate names for columns X

* expose information from the column - name, display name, default value X
* expose information about the cached data - number of rows, number of columns  X
* expoose column storage details - allowed values, default value, format, type
        * call it metadata? and have one per column storage type? readonly summary enum?


* add a snapshot - select columns snap at time
    * add a snapshot method to cache
    * should store time of snapshot
    * allow user to select columns to snapshot
    * add filtering criteria to snapshot creation method
    * add the ability to get a vector of values from a snapshot's column?
    * get csv vector by index and length given a sort order
    * get csv vector by guid and radius given a sort order


* use criterion crate for benchmarks  https://crates.io/crates/criterion#features


* allow updates of cell value by guid, column name  ??
