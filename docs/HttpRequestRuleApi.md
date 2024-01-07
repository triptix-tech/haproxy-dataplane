# \HttpRequestRuleApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_http_request_rule**](HttpRequestRuleApi.md#create_http_request_rule) | **POST** /services/haproxy/configuration/http_request_rules | Add a new HTTP Request Rule
[**delete_http_request_rule**](HttpRequestRuleApi.md#delete_http_request_rule) | **DELETE** /services/haproxy/configuration/http_request_rules/{index} | Delete a HTTP Request Rule
[**get_http_request_rule**](HttpRequestRuleApi.md#get_http_request_rule) | **GET** /services/haproxy/configuration/http_request_rules/{index} | Return one HTTP Request Rule
[**get_http_request_rules**](HttpRequestRuleApi.md#get_http_request_rules) | **GET** /services/haproxy/configuration/http_request_rules | Return an array of all HTTP Request Rules
[**replace_http_request_rule**](HttpRequestRuleApi.md#replace_http_request_rule) | **PUT** /services/haproxy/configuration/http_request_rules/{index} | Replace a HTTP Request Rule



## create_http_request_rule

> crate::models::HttpRequestRule create_http_request_rule(parent_name, parent_type, http_request_rule, transaction_id, version, force_reload)
Add a new HTTP Request Rule

Adds a new HTTP Request Rule of the specified type in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**http_request_rule** | [**HttpRequestRule**](HttpRequestRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::HttpRequestRule**](http_request_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_http_request_rule

> delete_http_request_rule(index, parent_name, parent_type, transaction_id, version, force_reload)
Delete a HTTP Request Rule

Deletes a HTTP Request Rule configuration by it's index from the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP Request Rule Index | [required] |
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


## get_http_request_rule

> crate::models::GetHttpRequestRule200Response get_http_request_rule(index, parent_name, parent_type, transaction_id)
Return one HTTP Request Rule

Returns one HTTP Request Rule configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP Request Rule Index | [required] |
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetHttpRequestRule200Response**](getHTTPRequestRule_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_http_request_rules

> crate::models::GetHttpRequestRules200Response get_http_request_rules(parent_name, parent_type, transaction_id)
Return an array of all HTTP Request Rules

Returns all HTTP Request Rules that are configured in specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetHttpRequestRules200Response**](getHTTPRequestRules_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_http_request_rule

> crate::models::HttpRequestRule replace_http_request_rule(index, parent_name, parent_type, http_request_rule, transaction_id, version, force_reload)
Replace a HTTP Request Rule

Replaces a HTTP Request Rule configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP Request Rule Index | [required] |
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**http_request_rule** | [**HttpRequestRule**](HttpRequestRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::HttpRequestRule**](http_request_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

