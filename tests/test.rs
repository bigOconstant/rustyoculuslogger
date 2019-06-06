use rustoculuslogger;

// #[test]
// fn test_print() {
//     let logobject = rustoculuslogger::LogObject::new()
//                             .msg("Error message".to_string())
//                             .op("Location of error message".to_string())
//                             .lvl("error".to_string())
//                             .optional_type("Mccarthy".to_string(),"was stillhere".to_string())
//                             .optionaldata("Optional data".to_string());

                            
    
    
//     logobject.print();

//     //logobject.map_test();
//     assert_eq!(1, 1);// Just testing a print out
// }

#[test]
fn test_print_info() {
    let logobject = rustoculuslogger::LogObject::new()
                           .msg("Committing offsets".to_string())
                            .op("ConsumerContext".to_string())
                            .lvl("INFO".to_string())
                            //.optionaldata(result.to_string())
                            .print();

                            
    
    
    //logobject.print();

    //logobject.map_test();
    assert_eq!(1, 1);// Just testing a print out
}