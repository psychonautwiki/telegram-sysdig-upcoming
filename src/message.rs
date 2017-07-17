#[derive(Serialize, Deserialize, Debug)]
struct MessageAlert {
    description: String,

    #[serde(rename = "editUrl")]
    edit_url: Option<String>,

    id: Option<String>,
    name: String,
    scope: Option<String>,
    severity: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageEntityMetricValue {
    metric: String,
    aggregation: String,

    #[serde(rename = "groupAggregation")]
    group_aggregation: String,

    value: u64
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageEntityPolicyAction {
    #[serde(rename = "type")]
    type_name: String,

    #[serde(rename = "beforeEventNs")]
    before_event_ns: u64,

    #[serde(rename = "afterEventNs")]
    after_event_ns: u64,

    #[serde(rename = "isLimitedToContainer")]
    is_limited_to_container: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageEntityPolicyFalcoDetails {
    #[serde(rename = "ruleNameRegEx")]
    rule_name_reg_ex: String
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageEntityPolicy {
    id: u64,
    version: u64,

    #[serde(rename = "createdOn")]
    created_on: u64,

    #[serde(rename = "modifiedOn")]
    modified_on: u64,

    name: String,
    description: String,
    severity: u64,
    status: String,
    enabled: bool,

    #[serde(rename = "type")]
    type_name: String,

    #[serde(rename = "notificationChannelIds")]
    notification_channel_ids: Vec<u64>,

    actions: Vec<MessageEntityPolicyAction>,

    #[serde(rename = "policyEventsCount")]
    policy_events_count: u64,

    #[serde(rename = "falcoDetails")]
    falco_details: Option<MessageEntityPolicyFalcoDetails>
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageEntityPolicyEventActionResult {
    #[serde(rename = "type")]
    type_name: String,
    successful: bool,
    token: String,

    #[serde(rename = "sysdigCaptureId")]
    sysdig_capture_id: u64
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageEntityPolicyEvent {
    id: String,
    version: u64,
    severity: u64,
    metrics: Vec<Option<String>>,

    #[serde(rename = "policyId")]
    policy_id: u64,

    #[serde(rename = "actionResults")]
    action_results: Vec<MessageEntityPolicyEventActionResult>,

    timestamp: u64,
    output: String,

    #[serde(rename = "hostMac")]
    host_mac: String,

    #[serde(rename = "isAggregated")]
    is_aggregated: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageEntity {
    entity: String,

    #[serde(rename = "metricValues")]
    metric_values: Vec<MessageEntityMetricValue>,

    #[serde(rename = "additionalInfo")]
    additional_info: Option<String>, /* UNSURE */

    policies: Option<Vec<MessageEntityPolicy>>,

    #[serde(rename = "policyEvents")]
    policy_events: Option<Vec<Option<MessageEntityPolicyEvent>>>
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageEvent {
    id: Option<String>,
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageEndEntity {
    entity: Option<String>,

    #[serde(rename = "metricValues")]
    metric_values: Vec<MessageEntityMetricValue>,

    #[serde(rename = "additionalInfo")]
    additional_info: Option<String> // is it really a string?
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    alert: MessageAlert,
    condition: Option<String>,
    entities: Vec<MessageEntity>,
    event: MessageEvent,

    state: String,
    source: String,
    resolved: bool,

    timespan: u64,
    timestamp: u64,

    #[serde(rename = "endTimestamp")]
    end_timestamp: Option<u64>,

    #[serde(rename = "endEntities")]
    end_entities: Option<Vec<MessageEntity>>
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use message::Message;

    #[test]
    fn message_1() {
let data = r#"{
    "timestamp": 1500122580000000,
    "timespan": 60000000,
    "alert": {
        "description": "REDACTED",
        "editUrl": null,
        "id": null,
        "name": "REDACTED",
        "scope": null,
        "severity": 3
    },
    "event": {
        "id": null,
        "url": "https://REDACTED.sysdig.com/#/events/f:1500122520,t:1500122580"
    },
    "state": "ACTIVE",
    "resolved": false,
    "entities": [{
        "entity": "",
        "metricValues": [{
            "metric": "policyEvent",
            "aggregation": "count",
            "groupAggregation": "none",
            "value": 1
        }],
        "additionalInfo": null,
        "policies": [{
            "id": 234,
            "version": 4,
            "createdOn": 1498147489000,
            "modifiedOn": 1500122491000,
            "name": "REDACTED",
            "description": "REDACTED",
            "severity": 3,
            "status": "CUSTOM",
            "enabled": true,
            "type": "REDACTED",
            "notificationChannelIds": [10227, 5917],
            "actions": [{
                "type": "REDACTED",
                "beforeEventNs": 5000000000,
                "afterEventNs": 10000000000,
                "isLimitedToContainer": true
            }],
            "policyEventsCount": 4,
            "falcoDetails": {
                "ruleNameRegEx": "REDACTED"
            }
        }],
        "policyEvents": [{
            "id": "REDACTED",
            "version": 1,
            "severity": 3,
            "metrics": ["REDACTED", null],
            "policyId": 234,
            "actionResults": [{
                "type": "REDACTED",
                "successful": true,
                "token": "REDACTED",
                "sysdigCaptureId": 110355
            }],
            "timestamp": 1500122565774072,
            "output": "REDACTED",
            "hostMac": "REDACTED",
            "isAggregated": false
        }]
    }],
    "condition": "REDACTED",
    "source": "REDACTED"
}"#;
        let _: Message = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn message_2() {
let data = r#"{
    "timestamp": 1500122580000000,
    "timespan": 60000000,
    "alert": {
        "severity": 3,
        "editUrl": null,
        "scope": null,
        "name": "REDACTED",
        "description": "REDACTED",
        "id": null
    },
    "event": {
        "id": null,
        "url": "https://REDACTED.sysdig.com/#/events/f:1500122520,t:1500122580"
    },
    "state": "OK",
    "resolved": false,
    "entities": [{
        "entity": "",
        "metricValues": [{
            "metric": "policyEvent",
            "aggregation": "count",
            "groupAggregation": "none",
            "value": 1
        }],
        "additionalInfo": null,
        "policies": [{
            "id": 234,
            "version": 4,
            "createdOn": 1498147489000,
            "modifiedOn": 1500122491000,
            "name": "REDACTED",
            "description": "REDACTED",
            "severity": 3,
            "status": "CUSTOM",
            "enabled": true,
            "type": "REDACTED",
            "notificationChannelIds": [10227, 5917],
            "actions": [{
                "type": "REDACTED",
                "beforeEventNs": 5000000000,
                "afterEventNs": 10000000000,
                "isLimitedToContainer": true
            }],
            "policyEventsCount": 4,
            "falcoDetails": {
                "ruleNameRegEx": "REDACTED"
            }
        }],
        "policyEvents": [{
            "id": "REDACTED",
            "version": 1,
            "severity": 3,
            "metrics": ["REDACTED", null],
            "policyId": 234,
            "actionResults": [{
                "type": "REDACTED",
                "successful": true,
                "token": "REDACTED",
                "sysdigCaptureId": 110355,
                "type": "REDACTED"
            }],
            "timestamp": 1500122565774072,
            "output": "REDACTED",
            "hostMac": "REDACTED",
            "isAggregated": false
        }]
    }],
    "condition": "REDACTED",
    "source": "REDACTED",
    "endEntities": [{
        "entity": "",
        "metricValues": [{
            "metric": "policyEvent",
            "aggregation": "count",
            "groupAggregation": "none",
            "value": 0
        }],
        "additionalInfo": null
    }]
}"#;
        let _: Message = serde_json::from_str(data).unwrap();
    }
}
