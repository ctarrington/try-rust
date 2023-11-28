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
        * duplicate column name 


* prevent duplicate names for columns

* allow updates of cell value by guid  ??


* add a snapshot - specify filters, select columns snap at time
    * add filtering
    * add the ability to get a vector of values from a snapshot's column?
    * add sorting


* use criterion crate for benchmarks

