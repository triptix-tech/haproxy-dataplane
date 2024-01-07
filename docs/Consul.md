# Consul

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** |  | 
**defaults** | Option<**String**> | Name of the defaults section to be used in backends created by this service | [optional]
**description** | Option<**String**> |  | [optional]
**enabled** | **bool** |  | 
**health_check_policy** | Option<**String**> | Defines the health check conditions required for each node to be considered valid for the service.   none: all nodes are considered valid   any: a node is considered valid if any one health check is 'passing'   all: a node is considered valid if all health checks are 'passing'   min: a node is considered valid if the number of 'passing' checks is greater or equal to the 'health_check_policy_min' value.     If the node has less health checks configured then 'health_check_policy_min' it is considered invalid. | [optional][default to None]
**health_check_policy_min** | Option<**i32**> |  | [optional]
**id** | Option<**String**> | Auto generated ID. | [optional]
**mode** | Option<**String**> |  | [optional][default to Http]
**name** | Option<**String**> |  | [optional]
**namespace** | Option<**String**> |  | [optional]
**port** | **i32** |  | 
**retry_timeout** | **i32** | Duration in seconds in-between data pulling requests to the consul server | 
**server_slots_base** | Option<**i32**> |  | [optional][default to 10]
**server_slots_growth_increment** | Option<**i32**> |  | [optional]
**server_slots_growth_type** | Option<**String**> |  | [optional][default to Exponential]
**service_blacklist** | Option<**Vec<String>**> | deprecated, use service_denylist | [optional]
**service_whitelist** | Option<**Vec<String>**> | deprecated, use service_allowlist | [optional]
**service_allowlist** | Option<**Vec<String>**> |  | [optional]
**service_denylist** | Option<**Vec<String>**> |  | [optional]
**service_name_regexp** | Option<**String**> | Regular expression used to filter services by name. | [optional]
**token** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


