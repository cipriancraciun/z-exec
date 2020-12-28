



macro_rules! log_define {
	( $_name : ident, $_slug : literal, $_level : expr ) => {
		
		#[ macro_export ]
		macro_rules! $_name {
			( $_code : expr, $_message : expr ) => {
				$crate::tools::log ($_slug, $_level, $_code, ::std::format_args! ($_message))
			};
			( $_code : expr, $_format : expr, $_argument_1 : expr ) => {
				$crate::tools::log ($_slug, $_level, $_code, ::std::format_args! ($_format, $_argument_1))
			};
			( $_code : expr, $_format : expr, $_argument_1 : expr, $_argument_2 : expr ) => {
				$crate::tools::log ($_slug, $_level, $_code, ::std::format_args! ($_format, $_argument_1, $_argument_2))
			};
			( $_code : expr, $_format : expr, $_argument_1 : expr, $_argument_2 : expr, $_argument_3 : expr ) => {
				$crate::tools::log ($_slug, $_level, $_code, ::std::format_args! ($_format, $_argument_1, $_argument_2, $_argument_3))
			};
			( $_code : expr, $_format : expr, $_argument_1 : expr, $_argument_2 : expr, $_argument_3 : expr, $_argument_4 : expr ) => {
				$crate::tools::log ($_slug, $_level, $_code, ::std::format_args! ($_format, $_argument_1, $_argument_2, $_argument_3, $_argument_4))
			};
			( $_code : expr, $_format : expr, $_argument_1 : expr, $_argument_2 : expr, $_argument_3 : expr, $_argument_4 : expr, $_argument_5 : expr ) => {
				$crate::tools::log ($_slug, $_level, $_code, ::std::format_args! ($_format, $_argument_1, $_argument_2, $_argument_3, $_argument_4, $_argument_5))
			};
		}
	}
}

log_define! (log_panic, "[!!]", crate::settings::LOG_LEVEL_PANIC);
log_define! (log_error, "[ee]", crate::settings::LOG_LEVEL_ERROR);
log_define! (log_warning, "[ww]", crate::settings::LOG_LEVEL_WARNING);
log_define! (log_notice, "[ii]", crate::settings::LOG_LEVEL_NOTICE);
log_define! (log_information, "[ii]", crate::settings::LOG_LEVEL_INFORMATION);
log_define! (log_debug, "[dd]", crate::settings::LOG_LEVEL_DEBUG);
log_define! (log_trace, "[dd]", crate::settings::LOG_LEVEL_TRACE);
log_define! (log_dump, "[dd]", 0);

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
	( $_code : expr, $_message : expr ) => {
		return ::std::result::Result::Err ($crate::tools::error_with_message ($_code, ::std::format_args! ($_message)))
	};
	( $_code : expr, $_format : expr, $( $_arguments : expr ),* ) => {
		return ::std::result::Result::Err ($crate::tools::error_with_message ($_code, ::std::format_args! ($_format, $( $_arguments ),* )))
	};
}

macro_rules! fail_wrap {
	( $_code : expr, $_error : expr ) => {
		return ::std::result::Result::Err ($crate::tools::error_wrap ($_code, $_error))
	};
	( $_code : expr, $_message : expr, $_error : expr ) => {
		return ::std::result::Result::Err ($crate::tools::error_wrap_with_message ($_code, ::std::format_args! ($_message), $_error))
	};
	( $_code : expr, $_format : expr, $( $_arguments : expr, )* $_error : expr ) => {
		return ::std::result::Result::Err ($crate::tools::error_wrap_with_message ($_code, ::std::format_args! ($_format, $( $_arguments ),* ), $_error))
	};
}


macro_rules! fail_unimplemented {
	( $_code : expr ) => {
		return ::std::result::Result::Err ($crate::tools::error_with_message ($_code, ::std::format_args! ("not implemented!")))
	};
}

macro_rules! fail_unreachable {
	( $_code : expr ) => {
		return ::std::Err ($crate::tools::error_with_message ($_code, ::std::format_args! ("unreachable assertion encountered!")))
	};
}

macro_rules! fail_assertion {
	( $_code : expr ) => {
		return ::std::result::Result::Err ($crate::tools::error_with_message ($_code, ::std::format_args! ("unexpected assertion encountered!")))
	};
}




macro_rules! panic_wrap {
	( $_code : expr, $_error : expr ) => { {
		$crate::log_panic! ($_code, "unexpected error encountered!");
		log_panic! (0, "{}", $_error);
		log_panic! (0x806fbb39, "aborting!");
		if true {
			::std::process::exit (2);
		}
		::std::panic! ($crate::tools::error_with_message ($_code, ::std::format_args! ("unexpected error encountered!")));
	} };
}


macro_rules! panic_unreachable {
	( $_code : expr ) => { {
		log_panic! ($_code, "unreachable assertion encountered!");
		log_panic! (0xd7448a6b, "aborting!");
		if true {
			::std::process::exit (2);
		}
		::std::panic! ($crate::tools::error_with_message ($_code, ::std::format_args! ("unreachable assertion encountered!")));
	} };
}

macro_rules! panic_assertion {
	( $_code : expr ) => { {
		log_panic! ($_code, "unexpected assertion encountered!");
		log_panic! (0x4d28335c, "aborting!");
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
		impl $crate::serialization::SerializableJson for $name {}
		impl $crate::serialization::DeserializableJson for $name {}
		impl $crate::serialization::SerializableRon for $name {}
		impl $crate::serialization::DeserializableRon for $name {}
	};
	
	( $name : ident : SerializableOnly ) => {
		impl $crate::serialization::SerializableOnly for $name {}
		impl $crate::serialization::SerializableJson for $name {}
		impl $crate::serialization::SerializableRon for $name {}
	};
	
	( $name : ident : DeserializableOnly ) => {
		impl $crate::serialization::DeserializableOnly for $name {}
		impl $crate::serialization::DeserializableJson for $name {}
		impl $crate::serialization::DeserializableRon for $name {}
	};
	
	( $name : ident <$type : ident> : Serializable ) => {
		impl <$type : $crate::serialization::Serializable> $crate::serialization::Serializable for $name <$type> {}
		impl <$type : $crate::serialization::SerializableJson> $crate::serialization::SerializableJson for $name <$type> {}
		impl <$type : $crate::serialization::DeserializableJson> $crate::serialization::DeserializableJson for $name <$type> {}
		impl <$type : $crate::serialization::SerializableRon> $crate::serialization::SerializableRon for $name <$type> {}
		impl <$type : $crate::serialization::DeserializableRon> $crate::serialization::DeserializableRon for $name <$type> {}
	};
	
	( $name : ident <$type : ident> : SerializableOnly ) => {
		impl <$type : $crate::serialization::SerializableOnly> $crate::serialization::SerializableOnly for $name <$type> {}
		impl <$type : $crate::serialization::SerializableJson> $crate::serialization::SerializableJson for $name <$type> {}
		impl <$type : $crate::serialization::SerializableRon> $crate::serialization::SerializableRon for $name <$type> {}
	};
	
	( $name : ident <$type : ident> : DeserializableOnly ) => {
		impl <$type : $crate::serialization::DeserializableOnly> $crate::serialization::DeserializableOnly for $name <$type> {}
		impl <$type : $crate::serialization::DeserializableJson> $crate::serialization::DeserializableJson for $name <$type> {}
		impl <$type : $crate::serialization::DeserializableRon> $crate::serialization::DeserializableRon for $name <$type> {}
	};
}

