# \MailersApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_mailers_section**](MailersApi.md#create_mailers_section) | **POST** /services/haproxy/configuration/mailers_section | Add a new Mailers section
[**delete_mailers_section**](MailersApi.md#delete_mailers_section) | **DELETE** /services/haproxy/configuration/mailers_section/{name} | Delete a Mailers section
[**edit_mailers_section**](MailersApi.md#edit_mailers_section) | **PUT** /services/haproxy/configuration/mailers_section/{name} | Modify a Mailers section
[**get_mailers_section**](MailersApi.md#get_mailers_section) | **GET** /services/haproxy/configuration/mailers_section/{name} | Return a Mailers section
[**get_mailers_sections**](MailersApi.md#get_mailers_sections) | **GET** /services/haproxy/configuration/mailers_section | Return an array of mailers sections



## create_mailers_section

> crate::models::MailersSection create_mailers_section(mailers_section, transaction_id, version, force_reload)
Add a new Mailers section

Creates a new empty Mailers section

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mailers_section** | [**MailersSection**](MailersSection.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::MailersSection**](mailers_section.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_mailers_section

> delete_mailers_section(name, transaction_id, version, force_reload)
Delete a Mailers section

Deletes a mailers from the configuration by it's name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Mailers name | [required] |
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


## edit_mailers_section

> crate::models::MailersSection edit_mailers_section(name, mailers_section, transaction_id, version, force_reload)
Modify a Mailers section

Modifies a mailers configuration by it's name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Mailers name | [required] |
**mailers_section** | [**MailersSection**](MailersSection.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::MailersSection**](mailers_section.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mailers_section

> crate::models::GetMailersSection200Response get_mailers_section(name, transaction_id)
Return a Mailers section

Returns one mailers configuration by it's name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Mailers name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetMailersSection200Response**](getMailersSection_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mailers_sections

> crate::models::GetMailersSections200Response get_mailers_sections(transaction_id)
Return an array of mailers sections

Returns an array of all the configured mailers in HAProxy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetMailersSections200Response**](getMailersSections_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

