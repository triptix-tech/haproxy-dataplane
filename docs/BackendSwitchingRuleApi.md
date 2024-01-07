# \BackendSwitchingRuleApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_backend_switching_rule**](BackendSwitchingRuleApi.md#create_backend_switching_rule) | **POST** /services/haproxy/configuration/backend_switching_rules | Add a new Backend Switching Rule
[**delete_backend_switching_rule**](BackendSwitchingRuleApi.md#delete_backend_switching_rule) | **DELETE** /services/haproxy/configuration/backend_switching_rules/{index} | Delete a Backend Switching Rule
[**get_backend_switching_rule**](BackendSwitchingRuleApi.md#get_backend_switching_rule) | **GET** /services/haproxy/configuration/backend_switching_rules/{index} | Return one Backend Switching Rule
[**get_backend_switching_rules**](BackendSwitchingRuleApi.md#get_backend_switching_rules) | **GET** /services/haproxy/configuration/backend_switching_rules | Return an array of all Backend Switching Rules
[**replace_backend_switching_rule**](BackendSwitchingRuleApi.md#replace_backend_switching_rule) | **PUT** /services/haproxy/configuration/backend_switching_rules/{index} | Replace a Backend Switching Rule



## create_backend_switching_rule

> crate::models::BackendSwitchingRule create_backend_switching_rule(frontend, backend_switching_rule, transaction_id, version, force_reload)
Add a new Backend Switching Rule

Adds a new Backend Switching Rule of the specified type in the specified frontend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**frontend** | **String** | Frontend name | [required] |
**backend_switching_rule** | [**BackendSwitchingRule**](BackendSwitchingRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::BackendSwitchingRule**](backend_switching_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_backend_switching_rule

> delete_backend_switching_rule(index, frontend, transaction_id, version, force_reload)
Delete a Backend Switching Rule

Deletes a Backend Switching Rule configuration by it's index from the specified frontend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Switching Rule Index | [required] |
**frontend** | **String** | Frontend name | [required] |
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


## get_backend_switching_rule

> crate::models::GetBackendSwitchingRule200Response get_backend_switching_rule(index, frontend, transaction_id)
Return one Backend Switching Rule

Returns one Backend Switching Rule configuration by it's index in the specified frontend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Switching Rule Index | [required] |
**frontend** | **String** | Frontend name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetBackendSwitchingRule200Response**](getBackendSwitchingRule_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_backend_switching_rules

> crate::models::GetBackendSwitchingRules200Response get_backend_switching_rules(frontend, transaction_id)
Return an array of all Backend Switching Rules

Returns all Backend Switching Rules that are configured in specified frontend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**frontend** | **String** | Frontend name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetBackendSwitchingRules200Response**](getBackendSwitchingRules_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_backend_switching_rule

> crate::models::BackendSwitchingRule replace_backend_switching_rule(index, frontend, backend_switching_rule, transaction_id, version, force_reload)
Replace a Backend Switching Rule

Replaces a Backend Switching Rule configuration by it's index in the specified frontend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Switching Rule Index | [required] |
**frontend** | **String** | Frontend name | [required] |
**backend_switching_rule** | [**BackendSwitchingRule**](BackendSwitchingRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::BackendSwitchingRule**](backend_switching_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

