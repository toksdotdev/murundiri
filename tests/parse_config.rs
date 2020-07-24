use murundiri::{config::*, hashmap_populate as hashmap};
use serde_json::json;
use std::{fs, str::from_utf8};

#[test]
fn test_that_config_is_parsed() {
    let book_services = Service::default().add_rule(
        "^/api/v1/books",
        Rule {
            timeframe: 2,
            action: RuleAction::default(),
            fields: RuleFields {
                body: Some(["trx_id".to_string(), "apple_id".to_string()].to_vec()),
                query: None,
                header: None,
            },
        },
    );

    let billing_service = Service::default().add_rule(
        "^/api/v1/recipient",
        Rule {
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
        },
    );

    let shipping_service = Service::default().add_rule(
        "\\w+/api/ships",
        Rule {
            timeframe: 3,
            action: RuleAction::Redirect {
                url: "https://google.com:9000/v2".to_string(),
            },
            fields: RuleFields {
                body: None,
                query: Some(["ship_id".to_string(), "reference".to_string()].to_vec()),
                header: Some(["Authorization".to_string(), "Content-Type".to_string()].to_vec()),
            },
        },
    );

    let expected_config = Config::new(
        "8080".to_string(),
        hashmap![
        "shipping_service".to_string() => shipping_service,
        "book_service".to_string() => book_services,
        "billing_service".to_string() => billing_service
        ],
    );

    let content = fs::read("tests/data/config.yml").unwrap();
    let parsed_config = Config::parse(from_utf8(content.as_slice()).unwrap()).unwrap();

    assert_eq!(parsed_config, expected_config);
}
