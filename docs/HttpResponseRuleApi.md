# \HttpResponseRuleApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_http_response_rule**](HttpResponseRuleApi.md#create_http_response_rule) | **POST** /services/haproxy/configuration/http_response_rules | Add a new HTTP Response Rule
[**delete_http_response_rule**](HttpResponseRuleApi.md#delete_http_response_rule) | **DELETE** /services/haproxy/configuration/http_response_rules/{index} | Delete a HTTP Response Rule
[**get_http_response_rule**](HttpResponseRuleApi.md#get_http_response_rule) | **GET** /services/haproxy/configuration/http_response_rules/{index} | Return one HTTP Response Rule
[**get_http_response_rules**](HttpResponseRuleApi.md#get_http_response_rules) | **GET** /services/haproxy/configuration/http_response_rules | Return an array of all HTTP Response Rules
[**replace_http_response_rule**](HttpResponseRuleApi.md#replace_http_response_rule) | **PUT** /services/haproxy/configuration/http_response_rules/{index} | Replace a HTTP Response Rule



## create_http_response_rule

> crate::models::HttpResponseRule create_http_response_rule(parent_name, parent_type, http_response_rule, transaction_id, version, force_reload)
Add a new HTTP Response Rule

Adds a new HTTP Response Rule of the specified type in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**http_response_rule** | [**HttpResponseRule**](HttpResponseRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::HttpResponseRule**](http_response_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_http_response_rule

> delete_http_response_rule(index, parent_name, parent_type, transaction_id, version, force_reload)
Delete a HTTP Response Rule

Deletes a HTTP Response Rule configuration by it's index from the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP Response Rule Index | [required] |
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
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


## get_http_response_rule

> crate::models::GetHttpResponseRule200Response get_http_response_rule(index, parent_name, parent_type, transaction_id)
Return one HTTP Response Rule

Returns one HTTP Response Rule configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP Response Rule Index | [required] |
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetHttpResponseRule200Response**](getHTTPResponseRule_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_http_response_rules

> crate::models::GetHttpResponseRules200Response get_http_response_rules(parent_name, parent_type, transaction_id)
Return an array of all HTTP Response Rules

Returns all HTTP Response Rules that are configured in specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetHttpResponseRules200Response**](getHTTPResponseRules_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_http_response_rule

> crate::models::HttpResponseRule replace_http_response_rule(index, parent_name, parent_type, http_response_rule, transaction_id, version, force_reload)
Replace a HTTP Response Rule

Replaces a HTTP Response Rule configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP Response Rule Index | [required] |
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**http_response_rule** | [**HttpResponseRule**](HttpResponseRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::HttpResponseRule**](http_response_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

