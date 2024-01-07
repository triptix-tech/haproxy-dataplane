# \PeerEntryApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_peer_entry**](PeerEntryApi.md#create_peer_entry) | **POST** /services/haproxy/configuration/peer_entries | Add a new peer_entry
[**delete_peer_entry**](PeerEntryApi.md#delete_peer_entry) | **DELETE** /services/haproxy/configuration/peer_entries/{name} | Delete a peer_entry
[**get_peer_entries**](PeerEntryApi.md#get_peer_entries) | **GET** /services/haproxy/configuration/peer_entries | Return an array of peer_entries
[**get_peer_entry**](PeerEntryApi.md#get_peer_entry) | **GET** /services/haproxy/configuration/peer_entries/{name} | Return one peer_entry
[**replace_peer_entry**](PeerEntryApi.md#replace_peer_entry) | **PUT** /services/haproxy/configuration/peer_entries/{name} | Replace a peer_entry



## create_peer_entry

> crate::models::PeerEntry create_peer_entry(peer_section, peer_entry, transaction_id, version, force_reload)
Add a new peer_entry

Adds a new peer entry in the specified peer section in the configuration file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**peer_section** | **String** | Parent peer section name | [required] |
**peer_entry** | [**PeerEntry**](PeerEntry.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::PeerEntry**](peer_entry.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_peer_entry

> delete_peer_entry(name, peer_section, transaction_id, version, force_reload)
Delete a peer_entry

Deletes a peer entry configuration by it's name in the specified peer section.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | PeerEntry name | [required] |
**peer_section** | **String** | Parent peers name | [required] |
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


## get_peer_entries

> crate::models::GetPeerEntries200Response get_peer_entries(peer_section, transaction_id)
Return an array of peer_entries

Returns an array of all peer_entries that are configured in specified peer section.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**peer_section** | **String** | Parent peer section name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetPeerEntries200Response**](getPeerEntries_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_peer_entry

> crate::models::GetPeerEntry200Response get_peer_entry(name, peer_section, transaction_id)
Return one peer_entry

Returns one peer_entry configuration by it's name in the specified peer section.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | PeerEntry name | [required] |
**peer_section** | **String** | Parent peers name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetPeerEntry200Response**](getPeerEntry_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_peer_entry

> crate::models::PeerEntry replace_peer_entry(name, peer_section, peer_entry, transaction_id, version, force_reload)
Replace a peer_entry

Replaces a peer entry configuration by it's name in the specified peer section.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | PeerEntry name | [required] |
**peer_section** | **String** | Parent peers name | [required] |
**peer_entry** | [**PeerEntry**](PeerEntry.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::PeerEntry**](peer_entry.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

