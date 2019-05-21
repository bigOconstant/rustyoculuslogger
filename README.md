# Rusty Oculus Logger

RustOculusLogger is a Rust Crate for logging error's according to oculus standards. [See Oculus Docs](https://pages.github.ibm.com/nettools/platform-documentation/Platform-Internal-Logging-Convention.html)

## Installation

add the following line to your dependencies in your cargo.toml
```
rustoculuslogger = { git = "ssh://git@github.ibm.com/Caleb-Andrew-McCarthy/rustoculuslogger", branch = "master" }
```


## Usage

```
extern crate rustoculuslogger;


rustoculuslogger::LogObject::new()
                            .msg("Error message".to_string())
                            .op("Location of error message".to_string())
                            .lvl("error".to_string())
                            .optionaldata("Optional data appends to msg".to_string())
                            .print();
```

will print out the following (with the date being more current)
```
{"@timestamp":"2019-05-21T18:56:02Z","lvl":"error","op":"Location of error message","msg":"Error message:Optional data appends to msg"}
```

the lvl can be one of 4 enums passed in as a string

1. Error
2. Warn
3. Warning
4. Info
5. Debug

You can also print a formatted json string with 
```
.print_pretty()
```
Custom string types are supported with the optionaltype function example

```
rustoculuslogger::LogObject::new()
                        .msg("Error message".to_string())
                        .op("Location of error message".to_string())
                        .lvl("error".to_string())
                        .optional_type("optionalType".to_string(),"Optional type value".to_string())
                        .optionaldata("Optional data".to_string()).print()
```


## License
[MIT](https://choosealicense.com/licenses/mit/)
