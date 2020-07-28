use murundiri::{config::*, hashmap_populate as hashmap};
use serde_json::json;
use std::{fs, str::from_utf8};

#[test]
fn test_that_config_is_parsed() {
    let book_rule = Rule {
        timeframe: 2,
        action: RuleAction::default(),
        fields: RuleFields {
            body: Some(["trx_id".to_string(), "apple_id".to_string()].to_vec()),
            query: None,
            header: None,
        },
    };

    let billing_rule = Rule {
        timeframe: 5,
        action: RuleAction::Respond {
            success: json!({
                "status": "pending",
                "message": "Bill is still pending."
            })
            .into(),
            failure: json!({
                "status": "unknown",
                "message": "Bill status is currently unknown."
            })
            .into(),
        },

        fields: RuleFields {
            body: Some(["card".to_string(), "pan".to_string()].to_vec()),
            query: None,
            header: None,
        },
    };

    let shipping_rule = Rule {
        timeframe: 3,
        action: RuleAction::Redirect {
            uri: "https://google.com:9000/v2".to_string(),
        },
        fields: RuleFields {
            body: None,
            query: Some(["ship_id".to_string(), "reference".to_string()].to_vec()),
            header: Some(["Authorization".to_string(), "Content-Type".to_string()].to_vec()),
        },
    };

    let expected_config = Config::new(
        "0.0.0.0".parse().unwrap(),
        8080,
        "/var/run/docker.sock".to_string(),
        hashmap![
        UriRegex::from_str("\\w+/api/ships").unwrap() => shipping_rule,
        UriRegex::from_str("^/api/v1/books").unwrap() => book_rule,
        UriRegex::from_str("^/api/v1/recipient").unwrap() => billing_rule
        ],
    );

    let content = fs::read("tests/data/config.yml").unwrap();
    let parsed_config = Config::parse(from_utf8(content.as_slice()).unwrap()).unwrap();

    assert_eq!(parsed_config, expected_config);
}
