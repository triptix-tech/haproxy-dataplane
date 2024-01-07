# \AclRuntimeApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_entry_to_acl**](AclRuntimeApi.md#add_entry_to_acl) | **POST** /services/haproxy/runtime/acl_file_entries | Add entry to an ACL file
[**add_payload_runtime_acl**](AclRuntimeApi.md#add_payload_runtime_acl) | **PUT** /services/haproxy/runtime/acl_file_entries | Add a new ACL payload
[**delete_entry_from_acl**](AclRuntimeApi.md#delete_entry_from_acl) | **DELETE** /services/haproxy/runtime/acl_file_entries/{id} | Delete an ACL entry
[**get_acl_file**](AclRuntimeApi.md#get_acl_file) | **GET** /services/haproxy/runtime/acls/{id} | Return an ACL file
[**get_acl_runtime_setting**](AclRuntimeApi.md#get_acl_runtime_setting) | **GET** /services/haproxy/runtime/acl_file_entries | Return an ACL entries
[**get_all_acl_files**](AclRuntimeApi.md#get_all_acl_files) | **GET** /services/haproxy/runtime/acls | Return an array of all ACL files
[**get_entry_from_acl**](AclRuntimeApi.md#get_entry_from_acl) | **GET** /services/haproxy/runtime/acl_file_entries/{id} | Return an ACL entry



## add_entry_to_acl

> crate::models::AclFileEntry add_entry_to_acl(acl_id, acl_file_entry)
Add entry to an ACL file

Adds an entry into the ACL file using the runtime socket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acl_id** | **String** | ACL ID | [required] |
**acl_file_entry** | [**AclFileEntry**](AclFileEntry.md) |  | [required] |

### Return type

[**crate::models::AclFileEntry**](acl_file_entry.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_payload_runtime_acl

> Vec<crate::models::AclFileEntry> add_payload_runtime_acl(acl_id, acl_file_entry)
Add a new ACL payload

Adds a new ACL payload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acl_id** | **String** | ACL ID | [required] |
**acl_file_entry** | [**Vec<crate::models::AclFileEntry>**](acl_file_entry.md) |  | [required] |

### Return type

[**Vec<crate::models::AclFileEntry>**](acl_file_entry.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_entry_from_acl

> delete_entry_from_acl(acl_id, id)
Delete an ACL entry

Deletes the entry from the ACL by its value using the runtime socket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acl_id** | **String** | ACL ID | [required] |
**id** | **String** | File entry ID | [required] |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_acl_file

> crate::models::AclFile get_acl_file(id)
Return an ACL file

Returns an ACL file by id using the runtime socket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ACL file entry ID | [required] |

### Return type

[**crate::models::AclFile**](acl_file.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_acl_runtime_setting

> Vec<crate::models::AclFileEntry> get_acl_runtime_setting(acl_id)
Return an ACL entries

Returns an ACL runtime setting using the runtime socket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acl_id** | **String** | ACL ID | [required] |

### Return type

[**Vec<crate::models::AclFileEntry>**](acl_file_entry.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_acl_files

> Vec<crate::models::AclFile> get_all_acl_files()
Return an array of all ACL files

Returns all ACL files using the runtime socket.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::AclFile>**](acl_file.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_entry_from_acl

> crate::models::AclFileEntry get_entry_from_acl(acl_id, id)
Return an ACL entry

Returns the ACL entry by its ID using the runtime socket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acl_id** | **String** | ACL ID | [required] |
**id** | **String** | File entry ID | [required] |

### Return type

[**crate::models::AclFileEntry**](acl_file_entry.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

