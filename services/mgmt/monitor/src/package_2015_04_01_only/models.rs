#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizableString {
    pub value: String,
    #[serde(rename = "localizedValue", skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SenderAuthorization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpRequestInfo {
    #[serde(rename = "clientRequestId", skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[serde(rename = "clientIpAddress", skip_serializing_if = "Option::is_none")]
    pub client_ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<SenderAuthorization>,
    #[serde(skip_serializing)]
    pub claims: Option<serde_json::Value>,
    #[serde(skip_serializing)]
    pub caller: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "eventDataId", skip_serializing)]
    pub event_data_id: Option<String>,
    #[serde(rename = "correlationId", skip_serializing)]
    pub correlation_id: Option<String>,
    #[serde(rename = "eventName", skip_serializing_if = "Option::is_none")]
    pub event_name: Option<LocalizableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<LocalizableString>,
    #[serde(rename = "httpRequest", skip_serializing_if = "Option::is_none")]
    pub http_request: Option<HttpRequestInfo>,
    #[serde(skip_serializing)]
    pub level: Option<event_data::Level>,
    #[serde(rename = "resourceGroupName", skip_serializing)]
    pub resource_group_name: Option<String>,
    #[serde(rename = "resourceProviderName", skip_serializing_if = "Option::is_none")]
    pub resource_provider_name: Option<LocalizableString>,
    #[serde(rename = "resourceId", skip_serializing)]
    pub resource_id: Option<String>,
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<LocalizableString>,
    #[serde(rename = "operationId", skip_serializing)]
    pub operation_id: Option<String>,
    #[serde(rename = "operationName", skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<LocalizableString>,
    #[serde(skip_serializing)]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LocalizableString>,
    #[serde(rename = "subStatus", skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<LocalizableString>,
    #[serde(rename = "eventTimestamp", skip_serializing)]
    pub event_timestamp: Option<String>,
    #[serde(rename = "submissionTimestamp", skip_serializing)]
    pub submission_timestamp: Option<String>,
    #[serde(rename = "subscriptionId", skip_serializing)]
    pub subscription_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
}
pub mod event_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Level {
        Critical,
        Error,
        Warning,
        Informational,
        Verbose,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventDataCollection {
    pub value: Vec<EventData>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleCapacity {
    pub minimum: String,
    pub maximum: String,
    pub default: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTrigger {
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[serde(rename = "metricNamespace", skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    #[serde(rename = "metricResourceUri")]
    pub metric_resource_uri: String,
    #[serde(rename = "timeGrain")]
    pub time_grain: String,
    pub statistic: metric_trigger::Statistic,
    #[serde(rename = "timeWindow")]
    pub time_window: String,
    #[serde(rename = "timeAggregation")]
    pub time_aggregation: metric_trigger::TimeAggregation,
    pub operator: metric_trigger::Operator,
    pub threshold: f64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<ScaleRuleMetricDimension>,
}
pub mod metric_trigger {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Statistic {
        Average,
        Min,
        Max,
        Sum,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TimeAggregation {
        Average,
        Minimum,
        Maximum,
        Total,
        Count,
        Last,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Equals,
        NotEquals,
        GreaterThan,
        GreaterThanOrEqual,
        LessThan,
        LessThanOrEqual,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleAction {
    pub direction: scale_action::Direction,
    #[serde(rename = "type")]
    pub type_: scale_action::Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub cooldown: String,
}
pub mod scale_action {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Direction {
        None,
        Increase,
        Decrease,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        ChangeCount,
        PercentChangeCount,
        ExactCount,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleRule {
    #[serde(rename = "metricTrigger")]
    pub metric_trigger: MetricTrigger,
    #[serde(rename = "scaleAction")]
    pub scale_action: ScaleAction,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeWindow {
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    pub start: String,
    pub end: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurrentSchedule {
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    pub days: Vec<String>,
    pub hours: Vec<i32>,
    pub minutes: Vec<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recurrence {
    pub frequency: recurrence::Frequency,
    pub schedule: RecurrentSchedule,
}
pub mod recurrence {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Frequency {
        None,
        Second,
        Minute,
        Hour,
        Day,
        Week,
        Month,
        Year,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleProfile {
    pub name: String,
    pub capacity: ScaleCapacity,
    pub rules: Vec<ScaleRule>,
    #[serde(rename = "fixedDate", skip_serializing_if = "Option::is_none")]
    pub fixed_date: Option<TimeWindow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Recurrence>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailNotification {
    #[serde(rename = "sendToSubscriptionAdministrator", skip_serializing_if = "Option::is_none")]
    pub send_to_subscription_administrator: Option<bool>,
    #[serde(rename = "sendToSubscriptionCoAdministrators", skip_serializing_if = "Option::is_none")]
    pub send_to_subscription_co_administrators: Option<bool>,
    #[serde(rename = "customEmails", skip_serializing_if = "Vec::is_empty")]
    pub custom_emails: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookNotification {
    #[serde(rename = "serviceUri", skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleNotification {
    pub operation: autoscale_notification::Operation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailNotification>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub webhooks: Vec<WebhookNotification>,
}
pub mod autoscale_notification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        Scale,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSetting {
    pub profiles: Vec<AutoscaleProfile>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notifications: Vec<AutoscaleNotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "targetResourceUri", skip_serializing_if = "Option::is_none")]
    pub target_resource_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSettingResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: AutoscaleSetting,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSettingResourcePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutoscaleSetting>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSettingResourceCollection {
    pub value: Vec<AutoscaleSettingResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleRuleMetricDimension {
    #[serde(rename = "DimensionName")]
    pub dimension_name: String,
    #[serde(rename = "Operator")]
    pub operator: scale_rule_metric_dimension::Operator,
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}
pub mod scale_rule_metric_dimension {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Equals,
        NotEquals,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventCategoryCollection {
    pub value: Vec<LocalizableString>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
}
