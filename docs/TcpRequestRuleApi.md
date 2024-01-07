# \TcpRequestRuleApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tcp_request_rule**](TcpRequestRuleApi.md#create_tcp_request_rule) | **POST** /services/haproxy/configuration/tcp_request_rules | Add a new TCP Request Rule
[**delete_tcp_request_rule**](TcpRequestRuleApi.md#delete_tcp_request_rule) | **DELETE** /services/haproxy/configuration/tcp_request_rules/{index} | Delete a TCP Request Rule
[**get_tcp_request_rule**](TcpRequestRuleApi.md#get_tcp_request_rule) | **GET** /services/haproxy/configuration/tcp_request_rules/{index} | Return one TCP Request Rule
[**get_tcp_request_rules**](TcpRequestRuleApi.md#get_tcp_request_rules) | **GET** /services/haproxy/configuration/tcp_request_rules | Return an array of all TCP Request Rules
[**replace_tcp_request_rule**](TcpRequestRuleApi.md#replace_tcp_request_rule) | **PUT** /services/haproxy/configuration/tcp_request_rules/{index} | Replace a TCP Request Rule



## create_tcp_request_rule

> crate::models::TcpRequestRule create_tcp_request_rule(parent_name, parent_type, tcp_request_rule, transaction_id, version, force_reload)
Add a new TCP Request Rule

Adds a new TCP Request Rule of the specified type in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**tcp_request_rule** | [**TcpRequestRule**](TcpRequestRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::TcpRequestRule**](tcp_request_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tcp_request_rule

> delete_tcp_request_rule(index, parent_name, parent_type, transaction_id, version, force_reload)
Delete a TCP Request Rule

Deletes a TCP Request Rule configuration by it's index from the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | TCP Request Rule Index | [required] |
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


## get_tcp_request_rule

> crate::models::GetTcpRequestRule200Response get_tcp_request_rule(index, parent_name, parent_type, transaction_id)
Return one TCP Request Rule

Returns one TCP Request Rule configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | TCP Request Rule Index | [required] |
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetTcpRequestRule200Response**](getTCPRequestRule_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tcp_request_rules

> crate::models::GetTcpRequestRules200Response get_tcp_request_rules(parent_name, parent_type, transaction_id)
Return an array of all TCP Request Rules

Returns all TCP Request Rules that are configured in specified parent and parent type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetTcpRequestRules200Response**](getTCPRequestRules_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_tcp_request_rule

> crate::models::TcpRequestRule replace_tcp_request_rule(index, parent_name, parent_type, tcp_request_rule, transaction_id, version, force_reload)
Replace a TCP Request Rule

Replaces a TCP Request Rule configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | TCP Request Rule Index | [required] |
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**tcp_request_rule** | [**TcpRequestRule**](TcpRequestRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::TcpRequestRule**](tcp_request_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

