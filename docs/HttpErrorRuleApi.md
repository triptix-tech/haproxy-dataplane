# \HttpErrorRuleApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_http_error_rule**](HttpErrorRuleApi.md#create_http_error_rule) | **POST** /services/haproxy/configuration/http_error_rules | Add a new HTTP Error Rule
[**delete_http_error_rule**](HttpErrorRuleApi.md#delete_http_error_rule) | **DELETE** /services/haproxy/configuration/http_error_rules/{index} | Delete a HTTP Error Rule
[**get_http_error_rule**](HttpErrorRuleApi.md#get_http_error_rule) | **GET** /services/haproxy/configuration/http_error_rules/{index} | Return one HTTP Error Rule
[**get_http_error_rules**](HttpErrorRuleApi.md#get_http_error_rules) | **GET** /services/haproxy/configuration/http_error_rules | Return an array of all HTTP Error Rules
[**replace_http_error_rule**](HttpErrorRuleApi.md#replace_http_error_rule) | **PUT** /services/haproxy/configuration/http_error_rules/{index} | Replace a HTTP Error Rule



## create_http_error_rule

> crate::models::HttpErrorRule create_http_error_rule(parent_type, http_error_rule, parent_name, transaction_id, version, force_reload)
Add a new HTTP Error Rule

Adds a new HTTP Error Rule of the specified type in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_type** | **String** | Parent type | [required] |
**http_error_rule** | [**HttpErrorRule**](HttpErrorRule.md) |  | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::HttpErrorRule**](http_error_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_http_error_rule

> delete_http_error_rule(index, parent_type, parent_name, transaction_id, version, force_reload)
Delete a HTTP Error Rule

Deletes a HTTP Error Rule configuration by its index from the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP Error Rule Index | [required] |
**parent_type** | **String** | Parent type | [required] |
**parent_name** | Option<**String**> | Parent name |  |
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


## get_http_error_rule

> crate::models::GetHttpErrorRule200Response get_http_error_rule(index, parent_type, parent_name, transaction_id)
Return one HTTP Error Rule

Returns one HTTP Error Rule configuration by its index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP Error Rule Index | [required] |
**parent_type** | **String** | Parent type | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetHttpErrorRule200Response**](getHTTPErrorRule_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_http_error_rules

> crate::models::GetHttpErrorRules200Response get_http_error_rules(parent_type, parent_name, transaction_id)
Return an array of all HTTP Error Rules

Returns all HTTP Error Rules that are configured in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_type** | **String** | Parent type | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetHttpErrorRules200Response**](getHTTPErrorRules_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_http_error_rule

> crate::models::HttpErrorRule replace_http_error_rule(index, parent_type, http_error_rule, parent_name, transaction_id, version, force_reload)
Replace a HTTP Error Rule

Replaces a HTTP Error Rule configuration by its index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP Error Rule Index | [required] |
**parent_type** | **String** | Parent type | [required] |
**http_error_rule** | [**HttpErrorRule**](HttpErrorRule.md) |  | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::HttpErrorRule**](http_error_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

