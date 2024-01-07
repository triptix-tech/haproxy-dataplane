# \HttpAfterResponseRuleApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_http_after_response_rule**](HttpAfterResponseRuleApi.md#create_http_after_response_rule) | **POST** /services/haproxy/configuration/http_after_response_rules | Add a new HTTP After Response Rule
[**delete_http_after_response_rule**](HttpAfterResponseRuleApi.md#delete_http_after_response_rule) | **DELETE** /services/haproxy/configuration/http_after_response_rules/{index} | Delete a HTTP After Response Rule
[**get_http_after_response_rule**](HttpAfterResponseRuleApi.md#get_http_after_response_rule) | **GET** /services/haproxy/configuration/http_after_response_rules/{index} | Return one HTTP After Response Rule
[**get_http_after_response_rules**](HttpAfterResponseRuleApi.md#get_http_after_response_rules) | **GET** /services/haproxy/configuration/http_after_response_rules | Return an array of all HTTP After Response Rules
[**replace_http_after_response_rule**](HttpAfterResponseRuleApi.md#replace_http_after_response_rule) | **PUT** /services/haproxy/configuration/http_after_response_rules/{index} | Replace a HTTP After Response Rule



## create_http_after_response_rule

> crate::models::HttpAfterResponseRule create_http_after_response_rule(parent_name, parent_type, http_after_response_rule, transaction_id, version, force_reload)
Add a new HTTP After Response Rule

Adds a new HTTP After Response Rule of the specified type in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**http_after_response_rule** | [**HttpAfterResponseRule**](HttpAfterResponseRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::HttpAfterResponseRule**](http_after_response_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_http_after_response_rule

> delete_http_after_response_rule(index, parent_name, parent_type, transaction_id, version, force_reload)
Delete a HTTP After Response Rule

Deletes a HTTP After Response Rule configuration by it's index from the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP After Response Rule Index | [required] |
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


## get_http_after_response_rule

> crate::models::GetHttpAfterResponseRule200Response get_http_after_response_rule(index, parent_name, parent_type, transaction_id)
Return one HTTP After Response Rule

Returns one HTTP After Response Rule configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP After Response Rule Index | [required] |
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetHttpAfterResponseRule200Response**](getHTTPAfterResponseRule_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_http_after_response_rules

> crate::models::GetHttpAfterResponseRules200Response get_http_after_response_rules(parent_name, parent_type, transaction_id)
Return an array of all HTTP After Response Rules

Returns all HTTP After Response Rules that are configured in specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetHttpAfterResponseRules200Response**](getHTTPAfterResponseRules_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_http_after_response_rule

> crate::models::HttpAfterResponseRule replace_http_after_response_rule(index, parent_name, parent_type, http_after_response_rule, transaction_id, version, force_reload)
Replace a HTTP After Response Rule

Replaces a HTTP After Response Rule configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP After Response Rule Index | [required] |
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**http_after_response_rule** | [**HttpAfterResponseRule**](HttpAfterResponseRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::HttpAfterResponseRule**](http_after_response_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

