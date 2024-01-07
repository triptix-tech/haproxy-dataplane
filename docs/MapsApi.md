# \MapsApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_map_entry**](MapsApi.md#add_map_entry) | **POST** /services/haproxy/runtime/maps_entries | Adds an entry into the map file
[**add_payload_runtime_map**](MapsApi.md#add_payload_runtime_map) | **PUT** /services/haproxy/runtime/maps/{name} | Add a new map payload
[**clear_runtime_map**](MapsApi.md#clear_runtime_map) | **DELETE** /services/haproxy/runtime/maps/{name} | Remove all map entries from the map file
[**delete_runtime_map_entry**](MapsApi.md#delete_runtime_map_entry) | **DELETE** /services/haproxy/runtime/maps_entries/{id} | Deletes all the map entries from the map by its id
[**get_all_runtime_map_files**](MapsApi.md#get_all_runtime_map_files) | **GET** /services/haproxy/runtime/maps | Return runtime map files
[**get_one_runtime_map**](MapsApi.md#get_one_runtime_map) | **GET** /services/haproxy/runtime/maps/{name} | Return one runtime map file
[**get_runtime_map_entry**](MapsApi.md#get_runtime_map_entry) | **GET** /services/haproxy/runtime/maps_entries/{id} | Return one map runtime setting
[**replace_runtime_map_entry**](MapsApi.md#replace_runtime_map_entry) | **PUT** /services/haproxy/runtime/maps_entries/{id} | Replace the value corresponding to each id in a map
[**show_runtime_map**](MapsApi.md#show_runtime_map) | **GET** /services/haproxy/runtime/maps_entries | Return one map runtime entries



## add_map_entry

> crate::models::MapEntry add_map_entry(map, map_entry, force_sync)
Adds an entry into the map file

Adds an entry into the map file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**map** | **String** | Mapfile attribute storage_name | [required] |
**map_entry** | [**MapEntry**](MapEntry.md) |  | [required] |
**force_sync** | Option<**bool**> | If true, immediately syncs changes to disk |  |[default to false]

### Return type

[**crate::models::MapEntry**](map_entry.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_payload_runtime_map

> Vec<crate::models::MapEntry> add_payload_runtime_map(name, map_entry, force_sync)
Add a new map payload

Adds a new map payload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Map file name | [required] |
**map_entry** | [**Vec<crate::models::MapEntry>**](map_entry.md) |  | [required] |
**force_sync** | Option<**bool**> | If true, immediately syncs changes to disk |  |[default to false]

### Return type

[**Vec<crate::models::MapEntry>**](map_entry.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_runtime_map

> clear_runtime_map(name, force_delete, force_sync)
Remove all map entries from the map file

Remove all map entries from the map file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Map file name | [required] |
**force_delete** | Option<**bool**> | If true, deletes file from disk |  |
**force_sync** | Option<**bool**> | If true, immediately syncs changes to disk |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_runtime_map_entry

> delete_runtime_map_entry(id, map, force_sync)
Deletes all the map entries from the map by its id

Delete all the map entries from the map by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Map id | [required] |
**map** | **String** | Mapfile attribute storage_name | [required] |
**force_sync** | Option<**bool**> | If true, immediately syncs changes to disk |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_runtime_map_files

> Vec<crate::models::Map> get_all_runtime_map_files(include_unmanaged)
Return runtime map files

Returns runtime map files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_unmanaged** | Option<**bool**> | If true, also show unmanaged map files loaded in haproxy |  |[default to false]

### Return type

[**Vec<crate::models::Map>**](map.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_one_runtime_map

> crate::models::Map get_one_runtime_map(name)
Return one runtime map file

Returns one runtime map file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Map file name | [required] |

### Return type

[**crate::models::Map**](map.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_runtime_map_entry

> crate::models::MapEntry get_runtime_map_entry(id, map)
Return one map runtime setting

Returns one map runtime setting by it's id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Map id | [required] |
**map** | **String** | Mapfile attribute storage_name | [required] |

### Return type

[**crate::models::MapEntry**](map_entry.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_runtime_map_entry

> crate::models::MapEntry replace_runtime_map_entry(id, map, replace_runtime_map_entry_request, force_sync)
Replace the value corresponding to each id in a map

Replaces the value corresponding to each id in a map.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Map id | [required] |
**map** | **String** | Mapfile attribute storage_name | [required] |
**replace_runtime_map_entry_request** | [**ReplaceRuntimeMapEntryRequest**](ReplaceRuntimeMapEntryRequest.md) |  | [required] |
**force_sync** | Option<**bool**> | If true, immediately syncs changes to disk |  |[default to false]

### Return type

[**crate::models::MapEntry**](map_entry.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_runtime_map

> Vec<crate::models::MapEntry> show_runtime_map(map)
Return one map runtime entries

Returns an array of all entries in a given runtime map file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**map** | **String** | Mapfile attribute storage_name | [required] |

### Return type

[**Vec<crate::models::MapEntry>**](map_entry.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

