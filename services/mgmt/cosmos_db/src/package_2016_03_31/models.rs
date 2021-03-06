#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccountsListResult {
    #[serde(skip_serializing)]
    pub value: Vec<DatabaseAccount>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDatabaseListResult {
    #[serde(skip_serializing)]
    pub value: Vec<SqlDatabase>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlContainerListResult {
    #[serde(skip_serializing)]
    pub value: Vec<SqlContainer>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbDatabaseListResult {
    #[serde(skip_serializing)]
    pub value: Vec<MongoDbDatabase>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbCollectionListResult {
    #[serde(skip_serializing)]
    pub value: Vec<MongoDbCollection>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Table>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraKeyspaceListResult {
    #[serde(skip_serializing)]
    pub value: Vec<CassandraKeyspace>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraTableListResult {
    #[serde(skip_serializing)]
    pub value: Vec<CassandraTable>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinDatabaseListResult {
    #[serde(skip_serializing)]
    pub value: Vec<GremlinDatabase>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinGraphListResult {
    #[serde(skip_serializing)]
    pub value: Vec<GremlinGraph>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailoverPolicies {
    #[serde(rename = "failoverPolicies")]
    pub failover_policies: Vec<FailoverPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailoverPolicy {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "locationName", skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    #[serde(rename = "failoverPriority", skip_serializing_if = "Option::is_none")]
    pub failover_priority: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionForOnlineOffline {
    pub region: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "locationName", skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    #[serde(rename = "documentEndpoint", skip_serializing)]
    pub document_endpoint: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "failoverPriority", skip_serializing_if = "Option::is_none")]
    pub failover_priority: Option<i32>,
    #[serde(rename = "isZoneRedundant", skip_serializing_if = "Option::is_none")]
    pub is_zone_redundant: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<database_account::Kind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseAccountProperties>,
}
pub mod database_account {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "GlobalDocumentDB")]
        GlobalDocumentDb,
        #[serde(rename = "MongoDB")]
        MongoDb,
        Parse,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedResourceProperties {
    #[serde(rename = "_rid", skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,
    #[serde(rename = "_ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Value>,
    #[serde(rename = "_etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Throughput {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ThroughputProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThroughputProperties {
    #[serde(flatten)]
    pub throughput_resource: ThroughputResource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDatabase {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlDatabaseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDatabaseProperties {
    #[serde(flatten)]
    pub sql_database_resource: SqlDatabaseResource,
    #[serde(flatten)]
    pub extended_resource_properties: ExtendedResourceProperties,
    #[serde(rename = "_colls", skip_serializing_if = "Option::is_none")]
    pub colls: Option<String>,
    #[serde(rename = "_users", skip_serializing_if = "Option::is_none")]
    pub users: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlContainer {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlContainerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlContainerProperties {
    #[serde(flatten)]
    pub sql_container_resource: SqlContainerResource,
    #[serde(flatten)]
    pub extended_resource_properties: ExtendedResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbDatabase {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MongoDbDatabaseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbDatabaseProperties {
    #[serde(flatten)]
    pub mongo_db_database_resource: MongoDbDatabaseResource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbCollection {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MongoDbCollectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbCollectionProperties {
    #[serde(flatten)]
    pub mongo_db_collection_resource: MongoDbCollectionResource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Table {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableProperties {
    #[serde(flatten)]
    pub table_resource: TableResource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraKeyspace {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CassandraKeyspaceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraKeyspaceProperties {
    #[serde(flatten)]
    pub cassandra_keyspace_resource: CassandraKeyspaceResource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraTable {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CassandraTableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraTableProperties {
    #[serde(flatten)]
    pub cassandra_table_resource: CassandraTableResource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinDatabase {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GremlinDatabaseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinDatabaseProperties {
    #[serde(flatten)]
    pub gremlin_database_resource: GremlinDatabaseResource,
    #[serde(rename = "_rid", skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,
    #[serde(rename = "_ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Value>,
    #[serde(rename = "_etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinGraph {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GremlinGraphProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinGraphProperties {
    #[serde(flatten)]
    pub gremlin_graph_resource: GremlinGraphResource,
    #[serde(flatten)]
    pub extended_resource_properties: ExtendedResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsistencyPolicy {
    #[serde(rename = "defaultConsistencyLevel")]
    pub default_consistency_level: consistency_policy::DefaultConsistencyLevel,
    #[serde(rename = "maxStalenessPrefix", skip_serializing_if = "Option::is_none")]
    pub max_staleness_prefix: Option<i64>,
    #[serde(rename = "maxIntervalInSeconds", skip_serializing_if = "Option::is_none")]
    pub max_interval_in_seconds: Option<i32>,
}
pub mod consistency_policy {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DefaultConsistencyLevel {
        Eventual,
        Session,
        BoundedStaleness,
        Strong,
        ConsistentPrefix,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccountProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "documentEndpoint", skip_serializing)]
    pub document_endpoint: Option<String>,
    #[serde(rename = "databaseAccountOfferType", skip_serializing_if = "Option::is_none")]
    pub database_account_offer_type: Option<DatabaseAccountOfferType>,
    #[serde(rename = "ipRangeFilter", skip_serializing_if = "Option::is_none")]
    pub ip_range_filter: Option<IpRangeFilter>,
    #[serde(rename = "isVirtualNetworkFilterEnabled", skip_serializing_if = "Option::is_none")]
    pub is_virtual_network_filter_enabled: Option<bool>,
    #[serde(rename = "enableAutomaticFailover", skip_serializing_if = "Option::is_none")]
    pub enable_automatic_failover: Option<bool>,
    #[serde(rename = "consistencyPolicy", skip_serializing_if = "Option::is_none")]
    pub consistency_policy: Option<ConsistencyPolicy>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<Capability>,
    #[serde(rename = "writeLocations", skip_serializing)]
    pub write_locations: Vec<Location>,
    #[serde(rename = "readLocations", skip_serializing)]
    pub read_locations: Vec<Location>,
    #[serde(rename = "failoverPolicies", skip_serializing)]
    pub failover_policies: Vec<FailoverPolicy>,
    #[serde(rename = "virtualNetworkRules", skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
    #[serde(rename = "enableMultipleWriteLocations", skip_serializing_if = "Option::is_none")]
    pub enable_multiple_write_locations: Option<bool>,
    #[serde(rename = "enableCassandraConnector", skip_serializing_if = "Option::is_none")]
    pub enable_cassandra_connector: Option<bool>,
    #[serde(rename = "connectorOffer", skip_serializing_if = "Option::is_none")]
    pub connector_offer: Option<ConnectorOffer>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccountPatchProperties {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<Capability>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccountCreateUpdateProperties {
    #[serde(rename = "consistencyPolicy", skip_serializing_if = "Option::is_none")]
    pub consistency_policy: Option<ConsistencyPolicy>,
    pub locations: Vec<Location>,
    #[serde(rename = "databaseAccountOfferType")]
    pub database_account_offer_type: DatabaseAccountOfferType,
    #[serde(rename = "ipRangeFilter", skip_serializing_if = "Option::is_none")]
    pub ip_range_filter: Option<IpRangeFilter>,
    #[serde(rename = "isVirtualNetworkFilterEnabled", skip_serializing_if = "Option::is_none")]
    pub is_virtual_network_filter_enabled: Option<bool>,
    #[serde(rename = "enableAutomaticFailover", skip_serializing_if = "Option::is_none")]
    pub enable_automatic_failover: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<Capability>,
    #[serde(rename = "virtualNetworkRules", skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
    #[serde(rename = "enableMultipleWriteLocations", skip_serializing_if = "Option::is_none")]
    pub enable_multiple_write_locations: Option<bool>,
    #[serde(rename = "enableCassandraConnector", skip_serializing_if = "Option::is_none")]
    pub enable_cassandra_connector: Option<bool>,
    #[serde(rename = "connectorOffer", skip_serializing_if = "Option::is_none")]
    pub connector_offer: Option<ConnectorOffer>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccountCreateUpdateParameters {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<database_account_create_update_parameters::Kind>,
    pub properties: DatabaseAccountCreateUpdateProperties,
}
pub mod database_account_create_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "GlobalDocumentDB")]
        GlobalDocumentDb,
        #[serde(rename = "MongoDB")]
        MongoDb,
        Parse,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccountPatchParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseAccountPatchProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccountListReadOnlyKeysResult {
    #[serde(rename = "primaryReadonlyMasterKey", skip_serializing)]
    pub primary_readonly_master_key: Option<String>,
    #[serde(rename = "secondaryReadonlyMasterKey", skip_serializing)]
    pub secondary_readonly_master_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccountListKeysResult {
    #[serde(flatten)]
    pub database_account_list_read_only_keys_result: DatabaseAccountListReadOnlyKeysResult,
    #[serde(rename = "primaryMasterKey", skip_serializing)]
    pub primary_master_key: Option<String>,
    #[serde(rename = "secondaryMasterKey", skip_serializing)]
    pub secondary_master_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccountConnectionString {
    #[serde(rename = "connectionString", skip_serializing)]
    pub connection_string: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccountListConnectionStringsResult {
    #[serde(rename = "connectionStrings", skip_serializing_if = "Vec::is_empty")]
    pub connection_strings: Vec<DatabaseAccountConnectionString>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccountRegenerateKeyParameters {
    #[serde(rename = "keyKind")]
    pub key_kind: database_account_regenerate_key_parameters::KeyKind,
}
pub mod database_account_regenerate_key_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyKind {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "secondary")]
        Secondary,
        #[serde(rename = "primaryReadonly")]
        PrimaryReadonly,
        #[serde(rename = "secondaryReadonly")]
        SecondaryReadonly,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DatabaseAccountOfferType {
    Standard,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThroughputUpdateParameters {
    pub properties: ThroughputUpdateProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThroughputUpdateProperties {
    pub resource: ThroughputResource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDatabaseCreateUpdateParameters {
    pub properties: SqlDatabaseCreateUpdateProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDatabaseCreateUpdateProperties {
    pub resource: SqlDatabaseResource,
    pub options: CreateUpdateOptions,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlContainerCreateUpdateParameters {
    pub properties: SqlContainerCreateUpdateProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlContainerCreateUpdateProperties {
    pub resource: SqlContainerResource,
    pub options: CreateUpdateOptions,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbDatabaseCreateUpdateParameters {
    pub properties: MongoDbDatabaseCreateUpdateProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbDatabaseCreateUpdateProperties {
    pub resource: MongoDbDatabaseResource,
    pub options: CreateUpdateOptions,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbCollectionCreateUpdateParameters {
    pub properties: MongoDbCollectionCreateUpdateProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbCollectionCreateUpdateProperties {
    pub resource: MongoDbCollectionResource,
    pub options: CreateUpdateOptions,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableCreateUpdateParameters {
    pub properties: TableCreateUpdateProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableCreateUpdateProperties {
    pub resource: TableResource,
    pub options: CreateUpdateOptions,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraKeyspaceCreateUpdateParameters {
    pub properties: CassandraKeyspaceCreateUpdateProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraKeyspaceCreateUpdateProperties {
    pub resource: CassandraKeyspaceResource,
    pub options: CreateUpdateOptions,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraTableCreateUpdateParameters {
    pub properties: CassandraTableCreateUpdateProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraTableCreateUpdateProperties {
    pub resource: CassandraTableResource,
    pub options: CreateUpdateOptions,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinDatabaseCreateUpdateParameters {
    pub properties: GremlinDatabaseCreateUpdateProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinDatabaseCreateUpdateProperties {
    pub resource: GremlinDatabaseResource,
    pub options: CreateUpdateOptions,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinGraphCreateUpdateParameters {
    pub properties: GremlinGraphCreateUpdateProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinGraphCreateUpdateProperties {
    pub resource: GremlinGraphResource,
    pub options: CreateUpdateOptions,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThroughputResource {
    pub throughput: i64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDatabaseResource {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlContainerResource {
    pub id: String,
    #[serde(rename = "indexingPolicy", skip_serializing_if = "Option::is_none")]
    pub indexing_policy: Option<IndexingPolicy>,
    #[serde(rename = "partitionKey", skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<ContainerPartitionKey>,
    #[serde(rename = "defaultTtl", skip_serializing_if = "Option::is_none")]
    pub default_ttl: Option<i64>,
    #[serde(rename = "uniqueKeyPolicy", skip_serializing_if = "Option::is_none")]
    pub unique_key_policy: Option<UniqueKeyPolicy>,
    #[serde(rename = "conflictResolutionPolicy", skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_policy: Option<ConflictResolutionPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinGraphResource {
    pub id: String,
    #[serde(rename = "indexingPolicy", skip_serializing_if = "Option::is_none")]
    pub indexing_policy: Option<IndexingPolicy>,
    #[serde(rename = "partitionKey", skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<ContainerPartitionKey>,
    #[serde(rename = "defaultTtl", skip_serializing_if = "Option::is_none")]
    pub default_ttl: Option<i64>,
    #[serde(rename = "uniqueKeyPolicy", skip_serializing_if = "Option::is_none")]
    pub unique_key_policy: Option<UniqueKeyPolicy>,
    #[serde(rename = "conflictResolutionPolicy", skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_policy: Option<ConflictResolutionPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexingPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic: Option<bool>,
    #[serde(rename = "indexingMode", skip_serializing_if = "Option::is_none")]
    pub indexing_mode: Option<indexing_policy::IndexingMode>,
    #[serde(rename = "includedPaths", skip_serializing_if = "Vec::is_empty")]
    pub included_paths: Vec<IncludedPath>,
    #[serde(rename = "excludedPaths", skip_serializing_if = "Vec::is_empty")]
    pub excluded_paths: Vec<ExcludedPath>,
}
pub mod indexing_policy {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IndexingMode {
        Consistent,
        Lazy,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExcludedPath {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncludedPath {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub indexes: Vec<Indexes>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Indexes {
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<indexes::DataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<indexes::Kind>,
}
pub mod indexes {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataType {
        String,
        Number,
        Point,
        Polygon,
        LineString,
        MultiPolygon,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Hash,
        Range,
        Spatial,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerPartitionKey {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<container_partition_key::Kind>,
}
pub mod container_partition_key {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Hash,
        Range,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Path {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UniqueKeyPolicy {
    #[serde(rename = "uniqueKeys", skip_serializing_if = "Vec::is_empty")]
    pub unique_keys: Vec<UniqueKey>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UniqueKey {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<Path>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConflictResolutionPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<conflict_resolution_policy::Mode>,
    #[serde(rename = "conflictResolutionPath", skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_path: Option<String>,
    #[serde(rename = "conflictResolutionProcedure", skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_procedure: Option<String>,
}
pub mod conflict_resolution_policy {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        LastWriterWins,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbDatabaseResource {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbCollectionResource {
    pub id: String,
    #[serde(rename = "shardKey", skip_serializing_if = "Option::is_none")]
    pub shard_key: Option<ShardKeys>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub indexes: Vec<MongoIndex>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShardKeys {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoIndex {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<MongoIndexKeys>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<MongoIndexOptions>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoIndexKeys {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<Key>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Key {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoIndexOptions {
    #[serde(rename = "expireAfterSeconds", skip_serializing_if = "Option::is_none")]
    pub expire_after_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableResource {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraKeyspaceResource {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraTableResource {
    pub id: String,
    #[serde(rename = "defaultTtl", skip_serializing_if = "Option::is_none")]
    pub default_ttl: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<CassandraSchema>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraSchema {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<Column>,
    #[serde(rename = "partitionKeys", skip_serializing_if = "Vec::is_empty")]
    pub partition_keys: Vec<CassandraPartitionKey>,
    #[serde(rename = "clusterKeys", skip_serializing_if = "Vec::is_empty")]
    pub cluster_keys: Vec<ClusterKey>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraPartitionKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinDatabaseResource {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateUpdateOptions {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Capability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tags {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisioningState {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRangeFilter {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ignoreMissingVNetServiceEndpoint", skip_serializing_if = "Option::is_none")]
    pub ignore_missing_v_net_service_endpoint: Option<bool>,
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
        #[serde(rename = "Provider", skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(rename = "Resource", skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(rename = "Operation", skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsagesResult {
    #[serde(skip_serializing)]
    pub value: Vec<Usage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[serde(skip_serializing)]
    pub unit: Option<UnitType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[serde(rename = "quotaPeriod", skip_serializing)]
    pub quota_period: Option<String>,
    #[serde(skip_serializing)]
    pub limit: Option<i64>,
    #[serde(rename = "currentValue", skip_serializing)]
    pub current_value: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionUsagesResult {
    #[serde(skip_serializing)]
    pub value: Vec<PartitionUsage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionUsage {
    #[serde(flatten)]
    pub usage: Usage,
    #[serde(rename = "partitionId", skip_serializing)]
    pub partition_id: Option<String>,
    #[serde(rename = "partitionKeyRangeId", skip_serializing)]
    pub partition_key_range_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinitionsListResult {
    #[serde(skip_serializing)]
    pub value: Vec<MetricDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinition {
    #[serde(rename = "metricAvailabilities", skip_serializing)]
    pub metric_availabilities: Vec<MetricAvailability>,
    #[serde(rename = "primaryAggregationType", skip_serializing)]
    pub primary_aggregation_type: Option<metric_definition::PrimaryAggregationType>,
    #[serde(skip_serializing)]
    pub unit: Option<UnitType>,
    #[serde(rename = "resourceUri", skip_serializing)]
    pub resource_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
}
pub mod metric_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrimaryAggregationType {
        None,
        Average,
        Total,
        Minimum,
        Maximum,
        Last,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAvailability {
    #[serde(rename = "timeGrain", skip_serializing)]
    pub time_grain: Option<String>,
    #[serde(skip_serializing)]
    pub retention: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Metric>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    #[serde(rename = "startTime", skip_serializing)]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", skip_serializing)]
    pub end_time: Option<String>,
    #[serde(rename = "timeGrain", skip_serializing)]
    pub time_grain: Option<String>,
    #[serde(skip_serializing)]
    pub unit: Option<UnitType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[serde(rename = "metricValues", skip_serializing)]
    pub metric_values: Vec<MetricValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricName {
    #[serde(skip_serializing)]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", skip_serializing)]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricValue {
    #[serde(rename = "_count", skip_serializing)]
    pub count: Option<f64>,
    #[serde(skip_serializing)]
    pub average: Option<f64>,
    #[serde(skip_serializing)]
    pub maximum: Option<f64>,
    #[serde(skip_serializing)]
    pub minimum: Option<f64>,
    #[serde(skip_serializing)]
    pub timestamp: Option<String>,
    #[serde(skip_serializing)]
    pub total: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PercentileMetricListResult {
    #[serde(skip_serializing)]
    pub value: Vec<PercentileMetric>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PercentileMetric {
    #[serde(rename = "startTime", skip_serializing)]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", skip_serializing)]
    pub end_time: Option<String>,
    #[serde(rename = "timeGrain", skip_serializing)]
    pub time_grain: Option<String>,
    #[serde(skip_serializing)]
    pub unit: Option<UnitType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[serde(rename = "metricValues", skip_serializing)]
    pub metric_values: Vec<PercentileMetricValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PercentileMetricValue {
    #[serde(flatten)]
    pub metric_value: MetricValue,
    #[serde(rename = "P10", skip_serializing)]
    pub p10: Option<f64>,
    #[serde(rename = "P25", skip_serializing)]
    pub p25: Option<f64>,
    #[serde(rename = "P50", skip_serializing)]
    pub p50: Option<f64>,
    #[serde(rename = "P75", skip_serializing)]
    pub p75: Option<f64>,
    #[serde(rename = "P90", skip_serializing)]
    pub p90: Option<f64>,
    #[serde(rename = "P95", skip_serializing)]
    pub p95: Option<f64>,
    #[serde(rename = "P99", skip_serializing)]
    pub p99: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionMetricListResult {
    #[serde(skip_serializing)]
    pub value: Vec<PartitionMetric>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionMetric {
    #[serde(flatten)]
    pub metric: Metric,
    #[serde(rename = "partitionId", skip_serializing)]
    pub partition_id: Option<String>,
    #[serde(rename = "partitionKeyRangeId", skip_serializing)]
    pub partition_key_range_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UnitType {
    Count,
    Bytes,
    Seconds,
    Percent,
    CountPerSecond,
    BytesPerSecond,
    Milliseconds,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConnectorOffer {
    Small,
}
