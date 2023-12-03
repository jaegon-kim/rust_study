use clap::Parser;

mod test;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}


fn main() {

    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    println!("Hello World");
    //test::primitives::test_primitive();
    //test::print::test_print();
    //test::tuples::test_tuples();
    //test::array_slice::test_array_slice();
    //test::structures::test_structures();
    //test::enums::test_enum();
    //test::list_functional::test_list_functional();
    //test::box_heap::test_box();
    //test::methods::test_methods();
    //test::constants::test_constants();
    //test::variable_bindings::test_variable_findings();
    //test::mutability::test_mutability();
    //test::scope_shadowing::test_scope_shadowing();
    //test::declare_first::test_declare_first();
    //test::freezing::test_freezing();
    //test::casting::test_casting();
    //test::literal::test_literal();
    //test::inference::test_inference();
    //test::alias::test_alias();
    //test::from_into::test_from_into();
    //test::tryfrom_tryinto::test_tryfrom_tryinto();
    //test::to_from_string::test_to_from_string();
    //test::expression::test_expression();
    //test::flow_ctrl::test_flow_ctrl();
    //test::functions::test_functions();
    //test::closures::test_closures();
    //test::higher_order_functions::test_higher_order_functions();
    //test::diverging_functions::test_diverging_functions();
    test::modules::test_modules();
}