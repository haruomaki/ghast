use inkwell::{
    context::Context,
    types::{BasicMetadataTypeEnum, BasicTypeEnum, FunctionType},
};

use crate::corelang::CoreType;

#[allow(dead_code)]
#[derive(Debug)]
pub enum SemanticError {
    TypeConversionFailed(String),
    ReturnTypeMismatch(CoreType, CoreType),
    CannotApply(CoreType, CoreType),
    Misc,
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for SemanticError {}

pub fn coretype_to_llvm<'ctx>(
    context: &'ctx Context,
    core_type: CoreType,
) -> Result<BasicTypeEnum<'ctx>, SemanticError> {
    match core_type {
        CoreType::I32 => Ok(context.i32_type().into()),
        CoreType::Tuple(elems) => {
            let types: Vec<_> = elems
                .into_iter()
                .map(|elem| coretype_to_llvm(context, elem).unwrap())
                .collect();
            Ok(context.struct_type(&types, false).into())
        }
        _ => Err(SemanticError::TypeConversionFailed(format!(
            "CoreTypeからBasicTypeEnumへ変換できません: {:?}",
            core_type
        ))),
    }
}

pub fn make_fn_type<'ctx>(
    ret_type: BasicTypeEnum<'ctx>,
    param_types: &[BasicMetadataTypeEnum<'ctx>],
) -> FunctionType<'ctx> {
    match ret_type {
        BasicTypeEnum::IntType(it) => it.fn_type(param_types, false),
        BasicTypeEnum::StructType(st) => st.fn_type(param_types, false),
        _ => panic!("(´・ω・｀)"),
    }
}
