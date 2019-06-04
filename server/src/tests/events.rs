use std::collections::HashSet;

use opcua_types::{
    Variant, ExtensionObject,
    operand::{Operand, ContentFilterBuilder},
    service_types::{FilterOperator, ContentFilterElement},
};

use crate::{
    tests::*,
    address_space::AddressSpace,
    events::operator,
    events::event_filter,
};
use opcua_types::node_ids::DataTypeId::Boolean;

fn make_operands(operands: &[Operand]) -> Vec<ExtensionObject> {
    operands.iter().map(|v| v.into()).collect::<Vec<ExtensionObject>>()
}

fn do_operator_test<T>(f: T)
    where T: FnOnce(&AddressSpace, &mut HashSet<u32>, &Vec<ContentFilterElement>)
{
    opcua_console_logging::init();
    let mut used_elements = HashSet::new();
    let elements = vec![];
    let address_space = AddressSpace::new();

    f(&address_space, &mut used_elements, &elements);
}

#[test]
fn test_eq() {
    do_operator_test(|address_space, used_elements, elements| {
        // Simple test, compare two values of the same kind
        let operands = make_operands(&[Operand::literal(10), Operand::literal(10)]);
        let result = operator::eq(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(9), Operand::literal(10)]);
        let result = operator::eq(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = make_operands(&[Operand::literal(10), Operand::literal(11)]);
        let result = operator::eq(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));
    });
}

#[test]
fn test_lt() {
    do_operator_test(|address_space, used_elements, elements| {
        // Simple test, compare two values of the same kind
        let operands = make_operands(&[Operand::literal(9), Operand::literal(10)]);
        let result = operator::lt(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(10), Operand::literal(10)]);
        let result = operator::lt(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = make_operands(&[Operand::literal(11), Operand::literal(10)]);
        let result = operator::lt(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));
    });
}

#[test]
fn test_lte() {
    do_operator_test(|address_space, used_elements, elements| {
        // Simple test, compare two values of the same kind
        let operands = make_operands(&[Operand::literal(9), Operand::literal(10)]);
        let result = operator::lte(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(10), Operand::literal(10)]);
        let result = operator::lte(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(11), Operand::literal(10)]);
        let result = operator::lte(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));
    });
}

#[test]
fn test_gt() {
    do_operator_test(|address_space, used_elements, elements| {
        // Simple test, compare two values of the same kind
        let operands = make_operands(&[Operand::literal(11), Operand::literal(10)]);
        let result = operator::gt(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(10), Operand::literal(10)]);
        let result = operator::gt(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = make_operands(&[Operand::literal(9), Operand::literal(10)]);
        let result = operator::gt(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));
    });
}

#[test]
fn test_gte() {
    do_operator_test(|address_space, used_elements, elements| {
        // Simple test, compare two values of the same kind
        let operands = make_operands(&[Operand::literal(11), Operand::literal(10)]);
        let result = operator::gte(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(10), Operand::literal(10)]);
        let result = operator::gte(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(9), Operand::literal(10)]);
        let result = operator::gte(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));
    });
}

#[test]
fn test_not() {
    do_operator_test(|address_space, used_elements, elements| {
        let operands = make_operands(&[Operand::literal(false)]);
        let result = operator::not(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(true)]);
        let result = operator::not(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        // String
        let operands = make_operands(&[Operand::literal("0")]);
        let result = operator::not(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        // String(2)
        let operands = make_operands(&[Operand::literal("true")]);
        let result = operator::not(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        // Invalid - Double
        let operands = make_operands(&[Operand::literal(99.9)]);
        let result = operator::not(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Empty);

        // Invalid - Int32
        let operands = make_operands(&[Operand::literal(1)]);
        let result = operator::not(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Empty);
    });
}

#[test]
fn test_between() {
    do_operator_test(|address_space, used_elements, elements| {
        // Test operator with some ranges and mix of types with implicit conversion
        let operands = make_operands(&[Operand::literal(12), Operand::literal(12), Operand::literal(13)]);
        let result = operator::between(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(13), Operand::literal(12), Operand::literal(13)]);
        let result = operator::between(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(12.3), Operand::literal(12.0), Operand::literal(12.4)]);
        let result = operator::between(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(11.99), Operand::literal(12.0), Operand::literal(13.0)]);
        let result = operator::between(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = make_operands(&[Operand::literal(13.0001), Operand::literal(12.0), Operand::literal(13.0)]);
        let result = operator::between(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

//        let operands = make_operands(&[Operand::literal("12.5"), Operand::literal(12), Operand::literal(13)]);
//        let result = operator::between(&operands[..], used_elements, elements, address_space).unwrap();
//        assert_eq!(result, Variant::Boolean(true));
    })
}

#[test]
fn test_and() {
    do_operator_test(|address_space, used_elements, elements| {
        let operands = make_operands(&[Operand::literal(true), Operand::literal(true)]);
        let result = operator::and(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(false), Operand::literal(true)]);
        let result = operator::and(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = make_operands(&[Operand::literal(true), Operand::literal(false)]);
        let result = operator::and(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = make_operands(&[Operand::literal(false), Operand::literal(false)]);
        let result = operator::and(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = make_operands(&[Operand::literal(true), Operand::literal(())]);
        let result = operator::and(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Empty);

        let operands = make_operands(&[Operand::literal(()), Operand::literal(true)]);
        let result = operator::and(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Empty);

        let operands = make_operands(&[Operand::literal(false), Operand::literal(())]);
        let result = operator::and(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = make_operands(&[Operand::literal(()), Operand::literal(false)]);
        let result = operator::and(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = make_operands(&[Operand::literal(()), Operand::literal(())]);
        let result = operator::and(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Empty);
    })
}

#[test]
fn test_or() {
    do_operator_test(|address_space, used_elements, elements| {
        let operands = make_operands(&[Operand::literal(true), Operand::literal(true)]);
        let result = operator::or(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(true), Operand::literal(false)]);
        let result = operator::or(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(false), Operand::literal(true)]);
        let result = operator::or(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(false), Operand::literal(false)]);
        let result = operator::or(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = make_operands(&[Operand::literal(true), Operand::literal(())]);
        let result = operator::or(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(()), Operand::literal(true)]);
        let result = operator::or(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(true));

        let operands = make_operands(&[Operand::literal(false), Operand::literal(())]);
        let result = operator::or(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Empty);

        let operands = make_operands(&[Operand::literal(()), Operand::literal(false)]);
        let result = operator::or(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Empty);
    })
}


#[test]
fn test_in_list() {
    do_operator_test(|address_space, used_elements, elements| {
        let operands = make_operands(&[Operand::literal(10), Operand::literal(false)]);
        let result = operator::in_list(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));

        let operands = make_operands(&[Operand::literal(true), Operand::literal(false)]);
        let result = operator::in_list(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::Boolean(false));
        /*
                let operands = make_operands(&[Operand::literal("true"), Operand::literal(true)]);
                let result = operator::in_list(&operands[..], used_elements, elements, address_space).unwrap();
                assert_eq!(result, Variant::Boolean(true));

                let operands = make_operands(&[Operand::literal(99), Operand::literal(11), Operand::literal(()), Operand::literal(99.0)]);
                let result = operator::in_list(&operands[..], used_elements, elements, address_space).unwrap();
                assert_eq!(result, Variant::Boolean(true));

                let operands = make_operands(&[Operand::literal(()), Operand::literal(11), Operand::literal(()), Operand::literal(99.0)]);
                let result = operator::in_list(&operands[..], used_elements, elements, address_space).unwrap();
                assert_eq!(result, Variant::Boolean(true));

                let operands = make_operands(&[Operand::literal(33), Operand::literal(11), Operand::literal(()), Operand::literal(99.0)]);
                let result = operator::in_list(&operands[..], used_elements, elements, address_space).unwrap();
                assert_eq!(result, Variant::Boolean(false));
                */
    })
}

#[test]
fn test_bitwise_or() {
    do_operator_test(|address_space, used_elements, elements| {
        let operands = make_operands(&[Operand::literal(0xff00u16), Operand::literal(0x00ffu16)]);
        let result = operator::bitwise_or(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::UInt16(0xffff));
    })
}

#[test]
fn test_bitwise_and() {
    do_operator_test(|address_space, used_elements, elements| {
        let operands = make_operands(&[Operand::literal(0xf00fu16), Operand::literal(0x00ffu16)]);
        let result = operator::bitwise_and(&operands[..], used_elements, elements, address_space).unwrap();
        assert_eq!(result, Variant::UInt16(0x000f));
    })
}

#[test]
fn test_where_clause() {
    let address_space = AddressSpace::new();

    // IsNull(NULL)
    let f = ContentFilterBuilder::new()
        .is_null(Operand::literal(()))
        .build();
    let result = event_filter::evaluate_where_clause(&f, &address_space);
    assert_eq!(result.unwrap(), true.into());

    // (550 == "550") && (10.5 == "10.5")
    let f = ContentFilterBuilder::new()
        .and(Operand::element(1), Operand::element(2))
        .equals(Operand::literal(550), Operand::literal("550"))
        .equals(Operand::literal(10.5), Operand::literal("10.5"))
        .build();
    let result = event_filter::evaluate_where_clause(&f, &address_space);
    assert_eq!(result.unwrap(), true.into());

    // Like operator
    let f = ContentFilterBuilder::new()
        .like(Operand::literal("Hello world"), Operand::literal("[Hh]ello w%"))
        .build();
    let result = event_filter::evaluate_where_clause(&f, &address_space);
    assert_eq!(result.unwrap(), true.into());
}