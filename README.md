# Rusty Oculus Logger

RustOculusLogger is a Rust Crate for logging error's according to oculus standards. See 

## Installation

add the following line to your dependencies in your cargo.toml
```
rustoculuslogger = { git = "ssh://git@github.ibm.com/Caleb-Andrew-McCarthy/rustoculuslogger", branch = "master" }
```


## Usage

```extern crate rustoculuslogger;


rustoculuslogger::LogObject::new()
                            .msg("Error message".to_string())
                            .op("Location of error message".to_string())
                            .lvl("error".to_string())
                            .optionaldata("Optional data".to_string())
                            .print();
```
the lvl can be one of 4 enums passed in as a string

1.Error
2.Warn
3.Warning
4.Info
5.Debug

You can also print a formatted json string with 
```
.print_pretty()
```


## License
[MIT](https://choosealicense.com/licenses/mit/)
