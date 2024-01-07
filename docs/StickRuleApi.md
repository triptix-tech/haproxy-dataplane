# \StickRuleApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_stick_rule**](StickRuleApi.md#create_stick_rule) | **POST** /services/haproxy/configuration/stick_rules | Add a new Stick Rule
[**delete_stick_rule**](StickRuleApi.md#delete_stick_rule) | **DELETE** /services/haproxy/configuration/stick_rules/{index} | Delete a Stick Rule
[**get_stick_rule**](StickRuleApi.md#get_stick_rule) | **GET** /services/haproxy/configuration/stick_rules/{index} | Return one Stick Rule
[**get_stick_rules**](StickRuleApi.md#get_stick_rules) | **GET** /services/haproxy/configuration/stick_rules | Return an array of all Stick Rules
[**replace_stick_rule**](StickRuleApi.md#replace_stick_rule) | **PUT** /services/haproxy/configuration/stick_rules/{index} | Replace a Stick Rule



## create_stick_rule

> crate::models::StickRule create_stick_rule(backend, stick_rule, transaction_id, version, force_reload)
Add a new Stick Rule

Adds a new Stick Rule of the specified type in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backend** | **String** | Backend name | [required] |
**stick_rule** | [**StickRule**](StickRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::StickRule**](stick_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_stick_rule

> delete_stick_rule(index, backend, transaction_id, version, force_reload)
Delete a Stick Rule

Deletes a Stick Rule configuration by it's index from the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Stick Rule Index | [required] |
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


## get_stick_rule

> crate::models::GetStickRule200Response get_stick_rule(index, backend, transaction_id)
Return one Stick Rule

Returns one Stick Rule configuration by it's index in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Stick Rule Index | [required] |
**backend** | **String** | Backend name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetStickRule200Response**](getStickRule_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stick_rules

> crate::models::GetStickRules200Response get_stick_rules(backend, transaction_id)
Return an array of all Stick Rules

Returns all Stick Rules that are configured in specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backend** | **String** | Backend name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetStickRules200Response**](getStickRules_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_stick_rule

> crate::models::StickRule replace_stick_rule(index, backend, stick_rule, transaction_id, version, force_reload)
Replace a Stick Rule

Replaces a Stick Rule configuration by it's index in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Stick Rule Index | [required] |
**backend** | **String** | Backend name | [required] |
**stick_rule** | [**StickRule**](StickRule.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::StickRule**](stick_rule.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

