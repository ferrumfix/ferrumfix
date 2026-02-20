use fefix::prelude::*;
use fefix::tagvalue::{Config, Decoder, Encoder};
use fefix::Dictionary;

/// This test demonstrates a bug in nested repeating group parsing where the second entry
/// in an outer repeating group fails to parse correctly when the group contains nested groups.
///
/// The test case models a NewOrderList message with the following structure:
/// - NoOrders (repeating group)
///   - Order Entry 1
///     - ClOrdID, OrderQty (from NewOrder component)
///     - NoListTriggeringInstructions (nested repeating group)
///       - ListTriggerType
///   - Order Entry 2
///     - ClOrdID, OrderQty (from NewOrder component)
///     - NoListTriggeringInstructions (nested repeating group)
///       - ListTriggerType (multiple entries)
///
/// The bug: The second order entry's fields become inaccessible after parsing.
#[test]
fn test_nested_repeating_group_parsing_bug() {
    let dict = create_test_dictionary();
    let raw_msg = create_test_message_with_nested_groups();
    let raw_msg = raw_msg.as_slice();

    // Decode the message
    let mut decoder = Decoder::new(dict.clone());
    decoder.config_mut().verify_checksum = false;
    let decoded_msg = decoder.decode(&raw_msg).unwrap();

    // Verify the outer repeating group has 2 entries
    let orders_group = decoded_msg.group(73).unwrap();
    assert_eq!(
        orders_group.len(),
        2,
        "NoOrders group should have 2 entries"
    );

    // First order entry - should work correctly
    let order1 = orders_group.get(0).unwrap();

    // Verify first order's fields
    let clordid1: &str = order1.get(11).unwrap();
    assert_eq!(clordid1, "ORDER1");

    let qty1: f64 = order1.get(38).unwrap();
    assert_eq!(qty1, 100.0);

    // Verify first order's nested group
    let triggers1 = order1.group(25010).unwrap();
    assert_eq!(triggers1.len(), 1);
    let trigger1 = triggers1.get(0).unwrap();
    let trigger1_type: &str = trigger1.get(25011).unwrap();
    assert_eq!(trigger1_type, "1");

    // Second order entry - THIS IS WHERE THE BUG OCCURS
    let order2 = orders_group.get(1).unwrap();

    // BUG: The following assertions should pass but will fail
    // because the second entry's fields are not accessible
    let clordid2_result = order2.get::<&str>(11);
    assert!(
        clordid2_result.is_ok(),
        "BUG: Second order entry should have ClOrdID, but got: {:?}",
        clordid2_result
    );

    if let Ok(clordid2) = clordid2_result {
        assert_eq!(clordid2, "ORDER2");

        // Check OrderQty
        let qty2: f64 = order2.get(38).expect("Second order should have OrderQty");
        assert_eq!(qty2, 200.0);

        // Check nested group in second order
        let triggers2 = order2
            .group(25010)
            .expect("Second order should have trigger group");
        assert_eq!(
            triggers2.len(),
            2,
            "Second order should have 2 trigger instructions"
        );
    }
}

/// Test with a simpler repeating group structure (no nesting) to verify
/// basic repeating group functionality works correctly
#[test]
fn test_simple_repeating_group_works() {
    let dict = create_simple_test_dictionary();
    let raw_msg = create_simple_test_message();

    let mut decoder = Decoder::new(dict);
    decoder.config_mut().verify_checksum = false;
    let decoded_msg = decoder.decode(&raw_msg).unwrap();

    // Verify we can access both entries in a simple repeating group
    let group = decoded_msg.group(73).unwrap();
    assert_eq!(group.len(), 2);

    // First entry
    let entry1 = group.get(0).unwrap();
    let clordid1: &str = entry1.get(11).unwrap();
    assert_eq!(clordid1, "ORDER1");

    // Second entry - should work fine with simple groups
    let entry2 = group.get(1).unwrap();
    let clordid2: &str = entry2.get(11).unwrap();
    assert_eq!(clordid2, "ORDER2");
}

/// Creates a FIX dictionary with nested repeating groups for testing
fn create_test_dictionary() -> Dictionary {
    let dict_xml = r#"
    <fix major="4" minor="4" type='FIX' servicepack='0'>
        <header>
            <field name="BeginString" required="Y" />
            <field name="BodyLength" required="Y" />
            <field name="MsgType" required="Y" />
            <field name="SenderCompID" required="Y" />
            <field name="TargetCompID" required="Y" />
            <field name="MsgSeqNum" required="Y" />
            <field name="SendingTime" required="Y" />
        </header>
        <messages>
            <message name='NewOrderList' msgcat='app' msgtype='E'>
                <field name='ClListID' required='Y'/>
                <group name='NoOrders' required='N'>
                    <component name='NewOrder' required='Y'/>
                    <component name='ListTriggeringInstruction' required='N'/>
                </group>
            </message>
        </messages>
        <trailer>
            <field name='CheckSum' required='N'/>
        </trailer>
        <components>
            <component name='NewOrder'>
                <field name='ClOrdID' required='Y'/>
                <field name='OrderQty' required='N'/>
            </component>
            <component name='ListTriggeringInstruction'>
                <group name='NoListTriggeringInstructions' required='N'>
                    <field name='ListTriggerType' required='Y'/>
                </group>
            </component>
        </components>
        <fields>
            <field number="8" name="BeginString" type="STRING" />
            <field number="9" name="BodyLength" type="LENGTH" />
            <field number='10' name='CheckSum' type='STRING'/>
            <field number="35" name="MsgType" type="STRING" />
            <field number="49" name="SenderCompID" type="STRING" />
            <field number="56" name="TargetCompID" type="STRING" />
            <field number="34" name="MsgSeqNum" type="SEQNUM" />
            <field number="52" name="SendingTime" type="UTCTIMESTAMP" />
            <field number="73" name="NoOrders" type="NUMINGROUP" />
            <field number="11" name="ClOrdID" type="STRING" />
            <field number="38" name="OrderQty" type="QTY" />
            <field number='25014' name='ClListID' type='STRING'/>
            <field number='25010' name='NoListTriggeringInstructions' type='NUMINGROUP'/>
            <field number='25011' name='ListTriggerType' type='CHAR'>
                <value enum='1' description='ACTIVATED'/>
                <value enum='2' description='PARTIALLY_FILLED'/>
                <value enum='3' description='FILLED'/>
            </field>
        </fields>
    </fix>
    "#;

    Dictionary::from_quickfix_spec(dict_xml).unwrap()
}

/// Creates a simple FIX dictionary without nested groups for comparison
fn create_simple_test_dictionary() -> Dictionary {
    let dict_xml = r#"
    <fix major="4" minor="4" type='FIX' servicepack='0'>
        <header>
            <field name="BeginString" required="Y" />
            <field name="BodyLength" required="Y" />
            <field name="MsgType" required="Y" />
            <field name="SenderCompID" required="Y" />
            <field name="TargetCompID" required="Y" />
            <field name="MsgSeqNum" required="Y" />
            <field name="SendingTime" required="Y" />
        </header>
        <messages>
            <message name="TestMessage" msgtype="T" msgcat="app">
                <group name="NoOrders" required="N">
                    <field name="ClOrdID" required="Y" />
                    <field name="OrderQty" required="N" />
                </group>
            </message>
        </messages>
        <trailer>
            <field name='CheckSum' required='N'/>
        </trailer>
        <components>
        </components>
        <fields>
            <field number="8" name="BeginString" type="STRING" />
            <field number="9" name="BodyLength" type="LENGTH" />
            <field number="35" name="MsgType" type="STRING" />
            <field number="49" name="SenderCompID" type="STRING" />
            <field number="56" name="TargetCompID" type="STRING" />
            <field number="34" name="MsgSeqNum" type="SEQNUM" />
            <field number="52" name="SendingTime" type="UTCTIMESTAMP" />
            <field number="73" name="NoOrders" type="NUMINGROUP" />
            <field number="11" name="ClOrdID" type="STRING" />
            <field number="38" name="OrderQty" type="QTY" />
            <field number="10" name="CheckSum" type="STRING" />
        </fields>
    </fix>
    "#;

    Dictionary::from_quickfix_spec(dict_xml).unwrap()
}

/// Creates a test message with nested repeating groups
fn create_test_message_with_nested_groups() -> Vec<u8> {
    let mut encoder = Encoder::new();
    let mut buffer = Vec::new();
    let mut msg = encoder.start_message(b"FIX.4.4", &mut buffer, b"E");

    // Set header fields
    msg.set(49, "SENDER");
    msg.set(56, "TARGET");
    msg.set(34, 1u32);
    msg.set(52, "20240101-00:00:00.000");

    // Set message body
    msg.set(25014, "LIST123"); // ClListID

    // Set NoOrders = 2
    msg.set(73, 2);

    // First order entry
    msg.set(11, "ORDER1"); // ClOrdID
    msg.set(38, 100.0f64); // OrderQty
    msg.set(25010, 1); // NoListTriggeringInstructions = 1
    msg.set(25011, "1"); // ListTriggerType = ACTIVATED

    // Second order entry
    msg.set(11, "ORDER2"); // ClOrdID
    msg.set(38, 200.0f64); // OrderQty
    msg.set(25010, 2); // NoListTriggeringInstructions = 2
    msg.set(25011, "1"); // First trigger type = ACTIVATED
    msg.set(25011, "2"); // Second trigger type = PARTIALLY_FILLED

    let (raw_msg, _) = msg.done();
    raw_msg.to_vec()
}

/// Creates a simple test message without nested groups
fn create_simple_test_message() -> Vec<u8> {
    let mut encoder = Encoder::new();
    let mut buffer = Vec::new();
    let mut msg = encoder.start_message(b"FIX.4.4", &mut buffer, b"T");

    // Set header fields
    msg.set(49, "SENDER");
    msg.set(56, "TARGET");
    msg.set(34, 1u32);
    msg.set(52, "20240101-00:00:00.000");

    // Set NoOrders = 2
    msg.set(73, 2);

    // First order
    msg.set(11, "ORDER1");
    msg.set(38, 100.0f64);

    // Second order
    msg.set(11, "ORDER2");
    msg.set(38, 200.0f64);

    let (raw_msg, _) = msg.done();
    raw_msg.to_vec()
}

/// Test edge case: Empty nested groups in the second entry
#[test]
fn test_nested_group_with_empty_inner_group() {
    let dict = create_test_dictionary();

    // Create message with second order having empty trigger group
    let mut encoder = Encoder::new();
    let mut buffer = Vec::new();
    let mut msg = encoder.start_message(b"FIX.4.4", &mut buffer, b"E");

    // Set header fields
    msg.set(49, "SENDER");
    msg.set(56, "TARGET");
    msg.set(34, 1u32);
    msg.set(52, "20240101-00:00:00.000");
    msg.set(25014, "LIST456"); // ClListID

    // Set NoOrders = 2
    msg.set(73, 2);

    // First order entry with triggers
    msg.set(11, "ORDER1");
    msg.set(38, 100.0f64);
    msg.set(25010, 1);
    msg.set(25011, "1");

    // Second order entry with NO triggers (NoListTriggeringInstructions = 0)
    msg.set(11, "ORDER2");
    msg.set(38, 200.0f64);
    msg.set(25010, 0); // Empty nested group

    let (raw_msg, _) = msg.done();

    // Decode the message
    let mut decoder = Decoder::new(dict);
    decoder.config_mut().verify_checksum = false;
    let decoded_msg = decoder.decode(&raw_msg).unwrap();

    let orders_group = decoded_msg.group(73).unwrap();
    assert_eq!(orders_group.len(), 2);

    // Second order with empty nested group - should still be accessible
    let order2 = orders_group.get(1).unwrap();
    let clordid2_result = order2.get::<&str>(11);
    assert!(
        clordid2_result.is_ok(),
        "Second order with empty nested group should still have accessible fields"
    );
}
