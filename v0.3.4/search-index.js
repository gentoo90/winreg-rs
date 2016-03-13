var searchIndex = {};
searchIndex['winreg'] = {"items":[[3,"RegKeyMetadata","winreg","Metadata returned by `RegKey::query_info`",null,null],[12,"sub_keys","","",0,null],[12,"max_sub_key_len","","",0,null],[12,"max_class_len","","",0,null],[12,"values","","",0,null],[12,"max_value_name_len","","",0,null],[12,"max_value_len","","",0,null],[3,"RegValue","","Raw registry value",null,null],[12,"bytes","","",1,null],[12,"vtype","","",1,null],[3,"RegKey","","Handle of opened registry key",null,null],[3,"EnumKeys","","Iterator over subkeys names",null,null],[3,"EnumValues","","Iterator over values",null,null],[0,"enums","","`use winreg::enums::*;` to import all needed enumerations and constants",null,null],[17,"HKEY_CLASSES_ROOT","winreg::enums","",null,null],[17,"HKEY_CURRENT_USER","","",null,null],[17,"HKEY_LOCAL_MACHINE","","",null,null],[17,"HKEY_USERS","","",null,null],[17,"HKEY_PERFORMANCE_DATA","","",null,null],[17,"HKEY_PERFORMANCE_TEXT","","",null,null],[17,"HKEY_PERFORMANCE_NLSTEXT","","",null,null],[17,"HKEY_CURRENT_CONFIG","","",null,null],[17,"HKEY_DYN_DATA","","",null,null],[17,"HKEY_CURRENT_USER_LOCAL_SETTINGS","","",null,null],[17,"KEY_QUERY_VALUE","","",null,null],[17,"KEY_SET_VALUE","","",null,null],[17,"KEY_CREATE_SUB_KEY","","",null,null],[17,"KEY_ENUMERATE_SUB_KEYS","","",null,null],[17,"KEY_NOTIFY","","",null,null],[17,"KEY_CREATE_LINK","","",null,null],[17,"KEY_WOW64_32KEY","","",null,null],[17,"KEY_WOW64_64KEY","","",null,null],[17,"KEY_WOW64_RES","","",null,null],[17,"KEY_READ","","",null,null],[17,"KEY_WRITE","","",null,null],[17,"KEY_EXECUTE","","",null,null],[17,"KEY_ALL_ACCESS","","",null,null],[4,"RegType","","Enumeration of possible registry value types",null,null],[13,"REG_NONE","","",2,null],[13,"REG_SZ","","",2,null],[13,"REG_EXPAND_SZ","","",2,null],[13,"REG_BINARY","","",2,null],[13,"REG_DWORD","","",2,null],[13,"REG_DWORD_BIG_ENDIAN","","",2,null],[13,"REG_LINK","","",2,null],[13,"REG_MULTI_SZ","","",2,null],[13,"REG_RESOURCE_LIST","","",2,null],[13,"REG_FULL_RESOURCE_DESCRIPTOR","","",2,null],[13,"REG_RESOURCE_REQUIREMENTS_LIST","","",2,null],[13,"REG_QWORD","","",2,null],[11,"eq","","",2,{"inputs":[{"name":"regtype"},{"name":"regtype"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"regtype"},{"name":"regtype"}],"output":{"name":"bool"}}],[11,"clone","","",2,{"inputs":[{"name":"regtype"}],"output":{"name":"regtype"}}],[11,"fmt","","",2,{"inputs":[{"name":"regtype"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"types","winreg","Traits for loading/saving Registry values",null,null],[8,"FromRegValue","winreg::types","A trait for types that can be loaded from registry values.",null,null],[10,"from_reg_value","","",3,{"inputs":[{"name":"fromregvalue"},{"name":"regvalue"}],"output":{"name":"result"}}],[8,"ToRegValue","","A trait for types that can be written into registry values.",null,null],[10,"to_reg_value","","",4,{"inputs":[{"name":"toregvalue"}],"output":{"name":"regvalue"}}],[11,"from_reg_value","collections::string","",5,{"inputs":[{"name":"string"},{"name":"regvalue"}],"output":{"name":"result"}}],[11,"to_reg_value","","",5,{"inputs":[{"name":"string"}],"output":{"name":"regvalue"}}],[0,"serialization","winreg","Registry keys parsing and serialization",null,null],[3,"Encoder","winreg::serialization","",null,null],[3,"Decoder","","",null,null],[4,"EncoderError","","",null,null],[13,"EncodeNotImplemented","","",6,null],[13,"IoError","","",6,null],[13,"NoFieldName","","",6,null],[4,"DecoderError","","",null,null],[13,"DecodeNotImplemented","","",7,null],[13,"IoError","","",7,null],[13,"ParseError","","",7,null],[13,"NoFieldName","","",7,null],[6,"EncodeResult","","",null,null],[6,"DecodeResult","","",null,null],[11,"fmt","","",6,{"inputs":[{"name":"encodererror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",6,{"inputs":[{"name":"encodererror"},{"name":"error"}],"output":{"name":"encodererror"}}],[11,"fmt","","",8,{"inputs":[{"name":"encoder"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_key","","",8,{"inputs":[{"name":"encoder"},{"name":"regkey"}],"output":{"name":"encoderesult"}}],[11,"commit","","",8,{"inputs":[{"name":"encoder"}],"output":{"name":"encoderesult"}}],[11,"emit_nil","","",8,{"inputs":[{"name":"encoder"}],"output":{"name":"encoderesult"}}],[11,"emit_usize","","",8,{"inputs":[{"name":"encoder"},{"name":"usize"}],"output":{"name":"encoderesult"}}],[11,"emit_u64","","",8,{"inputs":[{"name":"encoder"},{"name":"u64"}],"output":{"name":"encoderesult"}}],[11,"emit_u32","","",8,{"inputs":[{"name":"encoder"},{"name":"u32"}],"output":{"name":"encoderesult"}}],[11,"emit_u16","","",8,{"inputs":[{"name":"encoder"},{"name":"u16"}],"output":{"name":"encoderesult"}}],[11,"emit_u8","","",8,{"inputs":[{"name":"encoder"},{"name":"u8"}],"output":{"name":"encoderesult"}}],[11,"emit_isize","","",8,{"inputs":[{"name":"encoder"},{"name":"isize"}],"output":{"name":"encoderesult"}}],[11,"emit_i64","","",8,{"inputs":[{"name":"encoder"},{"name":"i64"}],"output":{"name":"encoderesult"}}],[11,"emit_i32","","",8,{"inputs":[{"name":"encoder"},{"name":"i32"}],"output":{"name":"encoderesult"}}],[11,"emit_i16","","",8,{"inputs":[{"name":"encoder"},{"name":"i16"}],"output":{"name":"encoderesult"}}],[11,"emit_i8","","",8,{"inputs":[{"name":"encoder"},{"name":"i8"}],"output":{"name":"encoderesult"}}],[11,"emit_bool","","",8,{"inputs":[{"name":"encoder"},{"name":"bool"}],"output":{"name":"encoderesult"}}],[11,"emit_f64","","",8,{"inputs":[{"name":"encoder"},{"name":"f64"}],"output":{"name":"encoderesult"}}],[11,"emit_f32","","",8,{"inputs":[{"name":"encoder"},{"name":"f32"}],"output":{"name":"encoderesult"}}],[11,"emit_char","","",8,{"inputs":[{"name":"encoder"},{"name":"char"}],"output":{"name":"encoderesult"}}],[11,"emit_str","","",8,{"inputs":[{"name":"encoder"},{"name":"str"}],"output":{"name":"encoderesult"}}],[11,"emit_enum","","",8,{"inputs":[{"name":"encoder"},{"name":"str"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_enum_variant","","",8,{"inputs":[{"name":"encoder"},{"name":"str"},{"name":"usize"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_enum_variant_arg","","",8,{"inputs":[{"name":"encoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_enum_struct_variant","","",8,{"inputs":[{"name":"encoder"},{"name":"str"},{"name":"usize"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_enum_struct_variant_field","","",8,{"inputs":[{"name":"encoder"},{"name":"str"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_struct","","",8,{"inputs":[{"name":"encoder"},{"name":"str"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_struct_field","","",8,{"inputs":[{"name":"encoder"},{"name":"str"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_tuple","","",8,{"inputs":[{"name":"encoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_tuple_arg","","",8,{"inputs":[{"name":"encoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_tuple_struct","","",8,{"inputs":[{"name":"encoder"},{"name":"str"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_tuple_struct_arg","","",8,{"inputs":[{"name":"encoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_option","","",8,{"inputs":[{"name":"encoder"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_option_none","","",8,{"inputs":[{"name":"encoder"}],"output":{"name":"encoderesult"}}],[11,"emit_option_some","","",8,{"inputs":[{"name":"encoder"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_seq","","",8,{"inputs":[{"name":"encoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_seq_elt","","",8,{"inputs":[{"name":"encoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_map","","",8,{"inputs":[{"name":"encoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_map_elt_key","","",8,{"inputs":[{"name":"encoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"emit_map_elt_val","","",8,{"inputs":[{"name":"encoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"encoderesult"}}],[11,"fmt","","",7,{"inputs":[{"name":"decodererror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",9,{"inputs":[{"name":"decoder"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_key","","",9,{"inputs":[{"name":"decoder"},{"name":"regkey"}],"output":{"name":"decoderesult"}}],[11,"read_nil","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_usize","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_u64","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_u32","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_u16","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_u8","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_isize","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_i64","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_i32","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_i16","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_i8","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_bool","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_f64","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_f32","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_char","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_str","","",9,{"inputs":[{"name":"decoder"}],"output":{"name":"decoderesult"}}],[11,"read_enum","","",9,{"inputs":[{"name":"decoder"},{"name":"str"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"read_enum_variant","","",9,null],[11,"read_enum_variant_arg","","",9,{"inputs":[{"name":"decoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"read_enum_struct_variant","","",9,null],[11,"read_enum_struct_variant_field","","",9,{"inputs":[{"name":"decoder"},{"name":"str"},{"name":"usize"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"read_struct","","",9,{"inputs":[{"name":"decoder"},{"name":"str"},{"name":"usize"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"read_struct_field","","",9,{"inputs":[{"name":"decoder"},{"name":"str"},{"name":"usize"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"read_tuple","","",9,{"inputs":[{"name":"decoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"read_tuple_arg","","",9,{"inputs":[{"name":"decoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"read_tuple_struct","","",9,{"inputs":[{"name":"decoder"},{"name":"str"},{"name":"usize"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"read_tuple_struct_arg","","",9,{"inputs":[{"name":"decoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"read_option","","",9,{"inputs":[{"name":"decoder"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"read_seq","","",9,{"inputs":[{"name":"decoder"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"read_seq_elt","","",9,{"inputs":[{"name":"decoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"read_map","","",9,{"inputs":[{"name":"decoder"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"read_map_elt_key","","",9,{"inputs":[{"name":"decoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"read_map_elt_val","","",9,{"inputs":[{"name":"decoder"},{"name":"usize"},{"name":"f"}],"output":{"name":"decoderesult"}}],[11,"error","","",9,{"inputs":[{"name":"decoder"},{"name":"str"}],"output":{"name":"error"}}],[0,"transaction","winreg","",null,null],[3,"Transaction","winreg::transaction","",null,null],[12,"handle","","",10,null],[11,"fmt","","",10,{"inputs":[{"name":"transaction"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",10,{"inputs":[{"name":"transaction"}],"output":{"name":"result"}}],[11,"commit","","",10,{"inputs":[{"name":"transaction"}],"output":{"name":"result"}}],[11,"rollback","","",10,{"inputs":[{"name":"transaction"}],"output":{"name":"result"}}],[11,"drop","","",10,{"inputs":[{"name":"transaction"}],"output":null}],[11,"default","winreg","",0,{"inputs":[{"name":"regkeymetadata"}],"output":{"name":"regkeymetadata"}}],[11,"fmt","","",0,{"inputs":[{"name":"regkeymetadata"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",1,{"inputs":[{"name":"regvalue"},{"name":"regvalue"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"regvalue"},{"name":"regvalue"}],"output":{"name":"bool"}}],[11,"fmt","","",1,{"inputs":[{"name":"regvalue"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",11,{"inputs":[{"name":"regkey"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"predef","","Open one of predefined keys:",11,{"inputs":[{"name":"regkey"},{"name":"hkey"}],"output":{"name":"regkey"}}],[11,"open_subkey","","Open subkey with `KEY_ALL_ACCESS` permissions.\nWill open another handle to itself if `path` is an empty string.\nTo open with different permissions use `open_subkey_with_flags`.",11,{"inputs":[{"name":"regkey"},{"name":"p"}],"output":{"name":"result"}}],[11,"open_subkey_with_flags","","Open subkey with desired permissions.\nWill open another handle to itself if `path` is an empty string.",11,{"inputs":[{"name":"regkey"},{"name":"p"},{"name":"regsam"}],"output":{"name":"result"}}],[11,"open_subkey_transacted","","",11,{"inputs":[{"name":"regkey"},{"name":"p"},{"name":"transaction"}],"output":{"name":"result"}}],[11,"open_subkey_transacted_with_flags","","",11,{"inputs":[{"name":"regkey"},{"name":"p"},{"name":"transaction"},{"name":"regsam"}],"output":{"name":"result"}}],[11,"create_subkey","","Create subkey (and all missing parent keys)\nand open it with `KEY_ALL_ACCESS` permissions.\nWill just open key if it already exists.\nWill open another handle to itself if `path` is an empty string.\nTo create with different permissions use `create_subkey_with_flags`.",11,{"inputs":[{"name":"regkey"},{"name":"p"}],"output":{"name":"result"}}],[11,"create_subkey_with_flags","","",11,{"inputs":[{"name":"regkey"},{"name":"p"},{"name":"regsam"}],"output":{"name":"result"}}],[11,"create_subkey_transacted","","",11,{"inputs":[{"name":"regkey"},{"name":"p"},{"name":"transaction"}],"output":{"name":"result"}}],[11,"create_subkey_transacted_with_flags","","",11,{"inputs":[{"name":"regkey"},{"name":"p"},{"name":"transaction"},{"name":"regsam"}],"output":{"name":"result"}}],[11,"copy_tree","","Copy all the values and subkeys from `path` to `dest` key.\nWIll copy the content of `self` if `path` is an empty string.",11,{"inputs":[{"name":"regkey"},{"name":"p"},{"name":"regkey"}],"output":{"name":"result"}}],[11,"query_info","","",11,{"inputs":[{"name":"regkey"}],"output":{"name":"result"}}],[11,"enum_keys","","Return an iterator over subkeys names.",11,{"inputs":[{"name":"regkey"}],"output":{"name":"enumkeys"}}],[11,"enum_values","","Return an iterator over values.",11,{"inputs":[{"name":"regkey"}],"output":{"name":"enumvalues"}}],[11,"delete_subkey","","Delete key. Cannot delete if it has subkeys.\nWill delete itself if `path` is an empty string.\nUse `delete_subkey_all` for that.",11,{"inputs":[{"name":"regkey"},{"name":"p"}],"output":{"name":"result"}}],[11,"delete_subkey_transacted","","",11,{"inputs":[{"name":"regkey"},{"name":"p"},{"name":"transaction"}],"output":{"name":"result"}}],[11,"delete_subkey_all","","Recursively delete subkey with all its subkeys and values.\nWill delete itself if `path` is an empty string.",11,{"inputs":[{"name":"regkey"},{"name":"p"}],"output":{"name":"result"}}],[11,"get_value","","Get a value from registry and seamlessly convert it to the specified rust type\nwith `FromRegValue` implemented (currently `String`, `u32` and `u64`).\nWill get the `Default` value if `name` is an empty string.",11,{"inputs":[{"name":"regkey"},{"name":"n"}],"output":{"name":"result"}}],[11,"get_raw_value","","Get raw bytes from registry value.\nWill get the `Default` value if `name` is an empty string.",11,{"inputs":[{"name":"regkey"},{"name":"n"}],"output":{"name":"result"}}],[11,"set_value","","Seamlessly convert a value from a rust type and write it to the registry value\nwith `ToRegValue` trait implemented (currently `String`, `&str`, `u32` and `u64`).\nWill set the `Default` value if `name` is an empty string.",11,{"inputs":[{"name":"regkey"},{"name":"n"},{"name":"t"}],"output":{"name":"result"}}],[11,"set_raw_value","","Write raw bytes from `RegValue` struct to a registry value.\nWill set the `Default` value if `name` is an empty string.",11,{"inputs":[{"name":"regkey"},{"name":"n"},{"name":"regvalue"}],"output":{"name":"result"}}],[11,"delete_value","","Delete specified value from registry.\nWill delete the `Default` value if `name` is an empty string.",11,{"inputs":[{"name":"regkey"},{"name":"n"}],"output":{"name":"result"}}],[11,"encode","","Save `Encodable` type to a registry key.",11,{"inputs":[{"name":"regkey"},{"name":"t"}],"output":{"name":"encoderesult"}}],[11,"decode","","Load `Decodable` type from a registry key.",11,{"inputs":[{"name":"regkey"}],"output":{"name":"decoderesult"}}],[11,"drop","","",11,{"inputs":[{"name":"regkey"}],"output":null}],[11,"next","","",12,{"inputs":[{"name":"enumkeys"}],"output":{"name":"option"}}],[11,"next","","",13,{"inputs":[{"name":"enumvalues"}],"output":{"name":"option"}}]],"paths":[[3,"RegKeyMetadata"],[3,"RegValue"],[4,"RegType"],[8,"FromRegValue"],[8,"ToRegValue"],[3,"String"],[4,"EncoderError"],[4,"DecoderError"],[3,"Encoder"],[3,"Decoder"],[3,"Transaction"],[3,"RegKey"],[3,"EnumKeys"],[3,"EnumValues"]]};
initSearch(searchIndex);