# \ServerSwitchingRuleApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_server_switching_rule**](ServerSwitchingRuleApi.md#create_server_switching_rule) | **POST** /services/haproxy/configuration/server_switching_rules | Add a new Server Switching Rule
[**delete_server_switching_rule**](ServerSwitchingRuleApi.md#delete_server_switching_rule) | **DELETE** /services/haproxy/configuration/server_switching_rules/{index} | Delete a Server Switching Rule
[**get_server_switching_rule**](ServerSwitchingRuleApi.md#get_server_switching_rule) | **GET** /services/haproxy/configuration/server_switching_rules/{index} | Return one Server Switching Rule
[**get_server_switching_rules**](ServerSwitchingRuleApi.md#get_server_switching_rules) | **GET** /services/haproxy/configuration/server_switching_rules | Return an array of all Server Switching Rules
[**replace_server_switching_rule**](ServerSwitchingRuleApi.md#replace_server_switching_rule) | **PUT** /services/haproxy/configuration/server_switching_rules/{index} | Replace a Server Switching Rule



## create_server_switching_rule

> crate::models::ServerSwitchingRule create_server_switching_rule(backend, server_switching_rule, transaction_id, version, force_reload)
Add a new Server Switching Rule

Adds a new Server Switching Rule of the specified type in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backend** | **String** | Backend name | [required] |
**server_switching_rule** | [**ServerSwitchingRule**](ServerSwitchingRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::ServerSwitchingRule**](server_switching_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_server_switching_rule

> delete_server_switching_rule(index, backend, transaction_id, version, force_reload)
Delete a Server Switching Rule

Deletes a Server Switching Rule configuration by it's index from the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Switching Rule Index | [required] |
**backend** | **String** | Backend name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_switching_rule

> crate::models::GetServerSwitchingRule200Response get_server_switching_rule(index, backend, transaction_id)
Return one Server Switching Rule

Returns one Server Switching Rule configuration by it's index in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Switching Rule Index | [required] |
**backend** | **String** | Backend name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetServerSwitchingRule200Response**](getServerSwitchingRule_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_switching_rules

> crate::models::GetServerSwitchingRules200Response get_server_switching_rules(backend, transaction_id)
Return an array of all Server Switching Rules

Returns all Backend Switching Rules that are configured in specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backend** | **String** | Backend name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetServerSwitchingRules200Response**](getServerSwitchingRules_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_server_switching_rule

> crate::models::ServerSwitchingRule replace_server_switching_rule(index, backend, server_switching_rule, transaction_id, version, force_reload)
Replace a Server Switching Rule

Replaces a Server Switching Rule configuration by it's index in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Switching Rule Index | [required] |
**backend** | **String** | Backend name | [required] |
**server_switching_rule** | [**ServerSwitchingRule**](ServerSwitchingRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::ServerSwitchingRule**](server_switching_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

