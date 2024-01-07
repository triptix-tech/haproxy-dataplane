# GetSites200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_version** | Option<**i32**> |  | [optional]
**data** | [**Vec<crate::models::Site>**](site.md) | Sites array. Sites are considered as one service and all farms connected to that service. Farms are connected to service using use-backend and default_backend directives. Sites let you configure simple HAProxy configurations, for more advanced options use /haproxy/configuration endpoints.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


