#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

//use serde_json::Error;

#[derive(Serialize, Deserialize)]
struct MessageAlert {
    description: String,

    #[serde(rename = "editUrl")]
    edit_url: Option<String>,

    id: Option<String>,
    name: String,
    scope: Option<String>,
    severity: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct MessageEntityMetricValues {
    metric: String,
    aggregation: String,

    #[serde(rename = "groupAggregation")]
    group_aggregation: String,

    value: u32
}

#[derive(Serialize, Deserialize)]
struct MessageEntityPolicyAction {
    #[serde(rename = "type")]
    type_name: String,

    #[serde(rename = "beforeEventNs")]
    before_event_ns: u32,

    #[serde(rename = "afterEventNs")]
    after_event_ns: u32,

    #[serde(rename = "isLimitedToContainer")]
    is_limited_to_container: bool
}

#[derive(Serialize, Deserialize)]
struct MessageEntityPolicyFalcoDetails {
    #[serde(rename = "ruleNameRegEx")]
    rule_name_reg_ex: String
}

#[derive(Serialize, Deserialize)]
struct MessageEntityPolicy {
    id: u32,
    version: u32,

    #[serde(rename = "createdOn")]
    created_on: u32,

    #[serde(rename = "modifiedOn")]
    modified_on: u32,

    name: String,
    description: String,
    severity: u32,
    status: String,
    enabled: bool,

    #[serde(rename = "type")]
    type_name: String,

    #[serde(rename = "notificationChannelIds")]
    notification_channel_ids: Vec<u32>,

    actions: Vec<MessageEntityPolicyAction>,

    #[serde(rename = "policyEventsCount")]
    policy_events_count: u32,

    #[serde(rename = "falcoDetails")]
    falco_details: Option<MessageEntityPolicyFalcoDetails>
}

#[derive(Serialize, Deserialize)]
struct MessageEntity {
    entity: String,

    #[serde(rename = "metricValues")]
    metric_values: Vec<MessageEntityMetricValues>,

    #[serde(rename = "additionalInfo")]
    additional_info: Option<String>, /* UNSURE */

    policies: Vec<MessageEntityPolicy>
}

#[derive(Serialize, Deserialize)]
struct Message {
    alert: MessageAlert,
    condition: Option<String>,
    entities: Vec<MessageEntity>
}

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello]).launch();
}