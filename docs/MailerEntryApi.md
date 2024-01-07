# \MailerEntryApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_mailer_entry**](MailerEntryApi.md#create_mailer_entry) | **POST** /services/haproxy/configuration/mailer_entries | Add a new mailer_entry
[**delete_mailer_entry**](MailerEntryApi.md#delete_mailer_entry) | **DELETE** /services/haproxy/configuration/mailer_entries/{name} | Delete a mailer_entry
[**get_mailer_entries**](MailerEntryApi.md#get_mailer_entries) | **GET** /services/haproxy/configuration/mailer_entries | Return an array of mailer_entries
[**get_mailer_entry**](MailerEntryApi.md#get_mailer_entry) | **GET** /services/haproxy/configuration/mailer_entries/{name} | Return one mailer_entry
[**replace_mailer_entry**](MailerEntryApi.md#replace_mailer_entry) | **PUT** /services/haproxy/configuration/mailer_entries/{name} | Replace a mailer_entry



## create_mailer_entry

> crate::models::MailerEntry create_mailer_entry(mailers_section, mailer_entry, transaction_id, version, force_reload)
Add a new mailer_entry

Adds a new mailer entry to the specified mailers section in the configuration file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mailers_section** | **String** | Parent mailers section name | [required] |
**mailer_entry** | [**MailerEntry**](MailerEntry.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::MailerEntry**](mailer_entry.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_mailer_entry

> delete_mailer_entry(name, mailers_section, transaction_id, version, force_reload)
Delete a mailer_entry

Deletes a mailer entry configuration by it's name in the specified mailers section.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | MailerEntry name | [required] |
**mailers_section** | **String** | Parent mailers section name | [required] |
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


## get_mailer_entries

> crate::models::GetMailerEntries200Response get_mailer_entries(mailers_section, transaction_id)
Return an array of mailer_entries

Returns an array of all the mailer_entries configured in the specified mailers section.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mailers_section** | **String** | Parent mailers section name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetMailerEntries200Response**](getMailerEntries_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mailer_entry

> crate::models::GetMailerEntry200Response get_mailer_entry(name, mailers_section, transaction_id)
Return one mailer_entry

Returns one mailer_entry configuration by it's name in the specified mailers section.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | MailerEntry name | [required] |
**mailers_section** | **String** | Parent mailers name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetMailerEntry200Response**](getMailerEntry_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_mailer_entry

> crate::models::MailerEntry replace_mailer_entry(name, mailers_section, mailer_entry, transaction_id, version, force_reload)
Replace a mailer_entry

Replaces a mailer entry configuration by it's name in the specified mailers section.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | MailerEntry name | [required] |
**mailers_section** | **String** | Parent mailers section name | [required] |
**mailer_entry** | [**MailerEntry**](MailerEntry.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::MailerEntry**](mailer_entry.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

