# \TcpResponseRuleApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tcp_response_rule**](TcpResponseRuleApi.md#create_tcp_response_rule) | **POST** /services/haproxy/configuration/tcp_response_rules | Add a new TCP Response Rule
[**delete_tcp_response_rule**](TcpResponseRuleApi.md#delete_tcp_response_rule) | **DELETE** /services/haproxy/configuration/tcp_response_rules/{index} | Delete a TCP Response Rule
[**get_tcp_response_rule**](TcpResponseRuleApi.md#get_tcp_response_rule) | **GET** /services/haproxy/configuration/tcp_response_rules/{index} | Return one TCP Response Rule
[**get_tcp_response_rules**](TcpResponseRuleApi.md#get_tcp_response_rules) | **GET** /services/haproxy/configuration/tcp_response_rules | Return an array of all TCP Response Rules
[**replace_tcp_response_rule**](TcpResponseRuleApi.md#replace_tcp_response_rule) | **PUT** /services/haproxy/configuration/tcp_response_rules/{index} | Replace a TCP Response Rule



## create_tcp_response_rule

> crate::models::TcpResponseRule create_tcp_response_rule(backend, tcp_response_rule, transaction_id, version, force_reload)
Add a new TCP Response Rule

Adds a new TCP Response Rule of the specified type in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backend** | **String** | Parent backend name | [required] |
**tcp_response_rule** | [**TcpResponseRule**](TcpResponseRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::TcpResponseRule**](tcp_response_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tcp_response_rule

> delete_tcp_response_rule(index, backend, transaction_id, version, force_reload)
Delete a TCP Response Rule

Deletes a TCP Response Rule configuration by it's index from the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | TCP Response Rule Index | [required] |
**backend** | **String** | Parent backend name | [required] |
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


## get_tcp_response_rule

> crate::models::GetTcpResponseRule200Response get_tcp_response_rule(index, backend, transaction_id)
Return one TCP Response Rule

Returns one TCP Response Rule configuration by it's index in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | TCP Response Rule Index | [required] |
**backend** | **String** | Parent backend name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetTcpResponseRule200Response**](getTCPResponseRule_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tcp_response_rules

> crate::models::GetTcpResponseRules200Response get_tcp_response_rules(backend, transaction_id)
Return an array of all TCP Response Rules

Returns all TCP Response Rules that are configured in specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backend** | **String** | Parent backend name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetTcpResponseRules200Response**](getTCPResponseRules_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_tcp_response_rule

> crate::models::TcpResponseRule replace_tcp_response_rule(index, backend, tcp_response_rule, transaction_id, version, force_reload)
Replace a TCP Response Rule

Replaces a TCP Response Rule configuration by it's Index in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | TCP Response Rule Index | [required] |
**backend** | **String** | Parent backend name | [required] |
**tcp_response_rule** | [**TcpResponseRule**](TcpResponseRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::TcpResponseRule**](tcp_response_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

