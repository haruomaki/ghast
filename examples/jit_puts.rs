use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::AddressSpace;

fn make_puts_module<'ctx>(context: &'ctx Context, funname: &str) -> Module<'ctx> {
    let module = context.create_module("hello_world");
    let builder = context.create_builder();

    let i8_type = context.i8_type();
    let i32_type = context.i32_type();
    let i8ptr_type = i8_type.ptr_type(AddressSpace::default());

    // Define the string constant
    let str_constant = context.const_string(b"Hello World!", true);
    let global_str = module.add_global(
        str_constant.get_type(),
        Some(AddressSpace::default()),
        "str",
    );
    global_str.set_initializer(&str_constant);
    global_str.set_unnamed_addr(true);
    global_str.set_constant(true);
    global_str.set_linkage(Linkage::Private);

    // Declare the puts function
    let puts_type = i32_type.fn_type(&[i8ptr_type.into()], false);
    let puts = module.add_function("puts", puts_type, None);

    // Define the main function
    let main_type = i32_type.fn_type(&[], false);
    let main_func = module.add_function(funname, main_type, None);
    let entry_bb = context.append_basic_block(main_func, "");

    builder.position_at_end(entry_bb);

    // Call puts function with the string pointer
    // let str_ptr = builder
    //     .build_pointer_cast(
    //         global_str.as_pointer_value(),
    //         i8_type.ptr_type(AddressSpace::from(0)),
    //         "str_ptr",
    //     )
    //     .unwrap();
    // builder
    //     .build_call(puts, &[str_ptr.into()], "putscall")
    //     .unwrap();
    builder
        .build_call(puts, &[global_str.as_pointer_value().into()], "putscall")
        .unwrap();

    // Return 0
    builder
        .build_return(Some(&i32_type.const_int(0, false)))
        .unwrap();

    module
}

fn main() {
    let context = Context::create();

    let module = make_puts_module(&context, "main1");
    let module2 = make_puts_module(&context, "main2");

    module.link_in_module(module2).unwrap();

    // Print LLVM IR to stdout
    let ir = module.print_to_string().to_string();
    print!("{}", ir);
}
