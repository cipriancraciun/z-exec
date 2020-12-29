



#[ macro_export ]
macro_rules! log_panic {
	( $_code : expr, $_format : expr $( , $_arguments : expr )* ) => {
		$crate::tools::log ("[!!]", $crate::settings::LOG_LEVEL_PANIC, $_code, ::std::format_args! ( $_format $( , $_arguments )* ))
	};
}

#[ macro_export ]
macro_rules! log_error {
	( $_code : expr, $_format : expr $( , $_arguments : expr )* ) => {
		$crate::tools::log ("[ee]", $crate::settings::LOG_LEVEL_ERROR, $_code, ::std::format_args! ( $_format $( , $_arguments )* ))
	};
}

#[ macro_export ]
macro_rules! log_warning {
	( $_code : expr, $_format : expr $( , $_arguments : expr )* ) => {
		$crate::tools::log ("[ww]", $crate::settings::LOG_LEVEL_WARNING, $_code, ::std::format_args! ( $_format $( , $_arguments )* ))
	};
}

#[ macro_export ]
macro_rules! log_notice {
	( $_code : expr, $_format : expr $( , $_arguments : expr )* ) => {
		$crate::tools::log ("[ii]", $crate::settings::LOG_LEVEL_NOTICE, $_code, ::std::format_args! ( $_format $( , $_arguments )* ))
	};
}

#[ macro_export ]
macro_rules! log_information {
	( $_code : expr, $_format : expr $( , $_arguments : expr )* ) => {
		$crate::tools::log ("[ii]", $crate::settings::LOG_LEVEL_INFORMATION, $_code, ::std::format_args! ( $_format $( , $_arguments )* ))
	};
}

#[ macro_export ]
macro_rules! log_debug {
	( $_code : expr, $_format : expr $( , $_arguments : expr )* ) => {
		$crate::tools::log ("[dd]", $crate::settings::LOG_LEVEL_DEBUG, $_code, ::std::format_args! ( $_format $( , $_arguments )* ))
	};
}

#[ macro_export ]
macro_rules! log_trace {
	( $_code : expr, $_format : expr $( , $_arguments : expr )* ) => {
		$crate::tools::log ("[dd]", $crate::settings::LOG_LEVEL_TRACE, $_code, ::std::format_args! ( $_format $( , $_arguments )* ))
	};
}

#[ macro_export ]
macro_rules! log_dump {
	( $_code : expr, $_format : expr $( , $_arguments : expr )* ) => {
		$crate::tools::log ("[dd]", $crate::settings::LOG_LEVEL_DUMP, $_code, ::std::format_args! ( $_format $( , $_arguments )* ))
	};
}

#[ macro_export ]
macro_rules! log_cut {
	() => {
		$crate::tools::log_cut (false)
	};
}

#[ allow (unused_macros) ]
macro_rules! log_dump_cut {
	() => {
		$crate::tools::log_cut (true)
	};
}




macro_rules! fail {
	( $_code : expr ) => {
		return ::std::result::Result::Err ($crate::tools::error ($_code))
	};
	( $_code : expr, $_format : expr $( , $_arguments : expr )* ) => {
		return ::std::result::Result::Err ($crate::tools::error_with_message ($_code, ::std::format_args! ( $_format $( , $_arguments )* )))
	};
}

macro_rules! fail_wrap {
	( $_code : expr, $_error : expr ) => {
		return ::std::result::Result::Err ($crate::tools::error_wrap ($_code, $_error))
	};
	( $_code : expr, $_format : expr $( , $_arguments : expr )* ; $_error : expr ) => {
		return ::std::result::Result::Err ($crate::tools::error_wrap_with_message ($_code, ::std::format_args! ( $_format $( , $_arguments )* ), $_error))
	};
}


macro_rules! fail_unimplemented {
	( $_code : expr ) => {
		return ::std::result::Result::Err ($crate::tools::error_with_message ($_code, ::std::format_args! ("not implemented!")))
	};
}

macro_rules! fail_unreachable {
	( $_code : expr ) => {
		return ::std::result::Result::Err ($crate::tools::error_with_message ($_code, ::std::format_args! ("unreachable assertion encountered!")))
	};
}

macro_rules! fail_assertion {
	( $_code : expr ) => {
		return ::std::result::Result::Err ($crate::tools::error_with_message ($_code, ::std::format_args! ("unexpected assertion encountered!")))
	};
}




macro_rules! panic_with_message {
	( $_code : expr, $_format : expr $( , $_arguments : expr )* ) => { {
		$crate::log_panic! ($_code, $_format $( , $_arguments )* );
		$crate::log_panic! ($_code, "aborting!");
		if true {
			::std::process::exit (2);
		}
		::std::panic! ($crate::tools::error_with_message ($_code, ::std::format_args! ("unexpected error encountered!")));
	} };
}


macro_rules! panic_wrap {
	( $_code : expr, $_error : expr ) => { {
		$crate::log_panic! ($_code, "unexpected error encountered!");
		log_panic! (0, "{}", $_error);
		log_panic! ($_code, "aborting!");
		if true {
			::std::process::exit (2);
		}
		::std::panic! ($crate::tools::error_with_message ($_code, ::std::format_args! ("unexpected error encountered!")));
	} };
}


macro_rules! panic_unreachable {
	( $_code : expr ) => { {
		$crate::log_panic! ($_code, "unreachable assertion encountered!");
		$crate::log_panic! ($_code, "aborting!");
		if true {
			::std::process::exit (2);
		}
		::std::panic! ($crate::tools::error_with_message ($_code, ::std::format_args! ("unreachable assertion encountered!")));
	} };
}

macro_rules! panic_assertion {
	( $_code : expr ) => { {
		$crate::log_panic! ($_code, "unexpected assertion encountered!");
		$crate::log_panic! ($_code, "aborting!");
		if true {
			::std::process::exit (2);
		}
		::std::panic! ($crate::tools::error_with_message ($_code, ::std::format_args! ("unexpected assertion encountered!")));
	} };
}




macro_rules! panic {
	( ($_any : tt,)* ) => {
		::std::compile_error! ("panic macro not supported")
	}
}

macro_rules! unreachable {
	( ($_any : tt,)* ) => {
		::std::compile_error! ("unreachable macro not supported")
	}
}

macro_rules! unimplemented {
	( ($_any : tt,)* ) => {
		::std::compile_error! ("unimplemented macro not supported")
	}
}




#[ macro_export ]
macro_rules! serializable {
	
	
	( $name : ident : Serializable ) => {
		impl $crate::serialization::Serializable for $name {}
		serializable! ($name : SerializableErased);
		serializable! ($name : SerializableOnly);
		serializable! ($name : DeserializableOnly);
	};
	
	( $name : ident : SerializableErased ) => {
		impl $crate::serialization::SerializableErased for $name {}
	};
	
	( $name : ident : SerializableOnly ) => {
		impl $crate::serialization::SerializableOnly for $name {}
		impl $crate::serialization::SerializableJson for $name {}
		impl $crate::serialization::SerializableRon for $name {}
		impl $crate::serialization::SerializableRaw for $name {}
	};
	
	( $name : ident : DeserializableOnly ) => {
		impl $crate::serialization::DeserializableOnly for $name {}
		impl $crate::serialization::DeserializableJson for $name {}
		impl $crate::serialization::DeserializableRon for $name {}
		impl $crate::serialization::DeserializableRaw for $name {}
	};
	
	
	( $name : ident <$type : ident> : Serializable ) => {
		impl <$type : $crate::serialization::Serializable> $crate::serialization::Serializable for $name <$type> {}
		serializable! ($name<$type> : SerializableErased);
		serializable! ($name<$type> : SerializableOnly);
		serializable! ($name<$type> : DeserializableOnly);
	};
	
	( $name : ident <$type : ident> : SerializableErased ) => {
		impl <$type : ::erased_serde::Serialize> $crate::serialization::SerializableErased for $name <$type> {}
	};
	
	( $name : ident <$type : ident> : SerializableOnly ) => {
		impl <$type : ::serde::Serialize> $crate::serialization::SerializableOnly for $name <$type> {}
		impl <$type : ::serde::Serialize> $crate::serialization::SerializableJson for $name <$type> {}
		impl <$type : ::serde::Serialize> $crate::serialization::SerializableRon for $name <$type> {}
		impl <$type : ::serde::Serialize> $crate::serialization::SerializableRaw for $name <$type> {}
	};
	
	( $name : ident <$type : ident> : DeserializableOnly ) => {
		impl <$type : ::serde::de::DeserializeOwned> $crate::serialization::DeserializableOnly for $name <$type> {}
		impl <$type : ::serde::de::DeserializeOwned> $crate::serialization::DeserializableJson for $name <$type> {}
		impl <$type : ::serde::de::DeserializeOwned> $crate::serialization::DeserializableRon for $name <$type> {}
		impl <$type : ::serde::de::DeserializeOwned> $crate::serialization::DeserializableRaw for $name <$type> {}
	};
	
	
	( $name : ident <$type : ident : $type_constraint : path> : Serializable ) => {
		impl <$type : ::serde::Serialize + ::serde::de::DeserializeOwned + $type_constraint> $crate::serialization::Serializable for $name <$type> {}
		serializable! ($name<$type : $type_constraint> : SerializableErased);
		serializable! ($name<$type : $type_constraint> : SerializableOnly);
		serializable! ($name<$type : $type_constraint> : DeserializableOnly);
	};
	
	( $name : ident <$type : ident : $type_constraint : path> : SerializableErased ) => {
		impl <$type : ::erased_serde::Serialize + $type_constraint> $crate::serialization::SerializableErased for $name <$type> {}
	};
	
	( $name : ident <$type : ident : $type_constraint : path> : SerializableOnly ) => {
		impl <$type : ::serde::Serialize + $type_constraint> $crate::serialization::SerializableOnly for $name <$type> {}
		impl <$type : ::serde::Serialize + $type_constraint> $crate::serialization::SerializableJson for $name <$type> {}
		impl <$type : ::serde::Serialize + $type_constraint> $crate::serialization::SerializableRon for $name <$type> {}
		impl <$type : ::serde::Serialize + $type_constraint> $crate::serialization::SerializableRaw for $name <$type> {}
	};
	
	( $name : ident <$type : ident : $type_constraint : path> : DeserializableOnly ) => {
		impl <$type : ::serde::de::DeserializeOwned + $type_constraint> $crate::serialization::DeserializableOnly for $name <$type> {}
		impl <$type : ::serde::de::DeserializeOwned + $type_constraint> $crate::serialization::DeserializableJson for $name <$type> {}
		impl <$type : ::serde::de::DeserializeOwned + $type_constraint> $crate::serialization::DeserializableRon for $name <$type> {}
		impl <$type : ::serde::de::DeserializeOwned + $type_constraint> $crate::serialization::DeserializableRaw for $name <$type> {}
	};
}

