# \StorageApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_storage_general_file**](StorageApi.md#create_storage_general_file) | **POST** /services/haproxy/storage/general | Creates a managed storage general use file with contents
[**create_storage_map_file**](StorageApi.md#create_storage_map_file) | **POST** /services/haproxy/storage/maps | Creates a managed storage map file with its entries
[**create_storage_ssl_certificate**](StorageApi.md#create_storage_ssl_certificate) | **POST** /services/haproxy/storage/ssl_certificates | Create SSL certificate
[**delete_storage_general_file**](StorageApi.md#delete_storage_general_file) | **DELETE** /services/haproxy/storage/general/{name} | Deletes a managed general use file from disk
[**delete_storage_map**](StorageApi.md#delete_storage_map) | **DELETE** /services/haproxy/storage/maps/{name} | Deletes a managed map file from disk
[**delete_storage_ssl_certificate**](StorageApi.md#delete_storage_ssl_certificate) | **DELETE** /services/haproxy/storage/ssl_certificates/{name} | Delete SSL certificate from disk
[**get_all_storage_general_files**](StorageApi.md#get_all_storage_general_files) | **GET** /services/haproxy/storage/general | Return a list of all managed general use files
[**get_all_storage_map_files**](StorageApi.md#get_all_storage_map_files) | **GET** /services/haproxy/storage/maps | Return a list of all managed map files
[**get_all_storage_ssl_certificates**](StorageApi.md#get_all_storage_ssl_certificates) | **GET** /services/haproxy/storage/ssl_certificates | Return all available SSL certificates on disk
[**get_one_storage_general_file**](StorageApi.md#get_one_storage_general_file) | **GET** /services/haproxy/storage/general/{name} | Return the contents of one managed general use file from disk
[**get_one_storage_map**](StorageApi.md#get_one_storage_map) | **GET** /services/haproxy/storage/maps/{name} | Return the contents of one managed map file from disk
[**get_one_storage_ssl_certificate**](StorageApi.md#get_one_storage_ssl_certificate) | **GET** /services/haproxy/storage/ssl_certificates/{name} | Return one SSL certificate from disk
[**replace_storage_general_file**](StorageApi.md#replace_storage_general_file) | **PUT** /services/haproxy/storage/general/{name} | Replace contents of a managed general use file on disk
[**replace_storage_map_file**](StorageApi.md#replace_storage_map_file) | **PUT** /services/haproxy/storage/maps/{name} | Replace contents of a managed map file on disk
[**replace_storage_ssl_certificate**](StorageApi.md#replace_storage_ssl_certificate) | **PUT** /services/haproxy/storage/ssl_certificates/{name} | Replace SSL certificates on disk



## create_storage_general_file

> crate::models::GeneralFile create_storage_general_file(file_upload)
Creates a managed storage general use file with contents

Creates a managed storage general use file with contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_upload** | Option<**std::path::PathBuf**> | General use file content |  |

### Return type

[**crate::models::GeneralFile**](general_file.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_storage_map_file

> crate::models::Map create_storage_map_file(file_upload)
Creates a managed storage map file with its entries

Creates a managed storage map file with its entries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_upload** | Option<**std::path::PathBuf**> | The map file contents |  |

### Return type

[**crate::models::Map**](map.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_storage_ssl_certificate

> crate::models::SslCertificate create_storage_ssl_certificate(force_reload, file_upload)
Create SSL certificate

Creates SSL certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]
**file_upload** | Option<**std::path::PathBuf**> | The SSL certificate to upload |  |

### Return type

[**crate::models::SslCertificate**](ssl_certificate.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_storage_general_file

> delete_storage_general_file(name)
Deletes a managed general use file from disk

Deletes a managed general use file from disk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | General use file storage_name | [required] |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_storage_map

> delete_storage_map(name)
Deletes a managed map file from disk

Deletes a managed map file from disk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Map file storage_name | [required] |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_storage_ssl_certificate

> delete_storage_ssl_certificate(name, skip_reload, force_reload)
Delete SSL certificate from disk

Deletes SSL certificate from disk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | SSL certificate name | [required] |
**skip_reload** | Option<**bool**> | If set, no reload will be initiated after update |  |[default to false]
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_storage_general_files

> Vec<crate::models::GeneralFile> get_all_storage_general_files()
Return a list of all managed general use files

Returns a list of all managed general use files

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::GeneralFile>**](general_file.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_storage_map_files

> Vec<crate::models::Map> get_all_storage_map_files()
Return a list of all managed map files

Returns a list of all managed map files

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Map>**](map.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_storage_ssl_certificates

> Vec<crate::models::SslCertificate> get_all_storage_ssl_certificates()
Return all available SSL certificates on disk

Returns all available SSL certificates on disk.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SslCertificate>**](ssl_certificate.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_one_storage_general_file

> std::path::PathBuf get_one_storage_general_file(name)
Return the contents of one managed general use file from disk

Returns the contents of one managed general use file from disk

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | General use file storage_name | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_one_storage_map

> std::path::PathBuf get_one_storage_map(name)
Return the contents of one managed map file from disk

Returns the contents of one managed map file from disk

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Map file storage_name | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_one_storage_ssl_certificate

> crate::models::SslCertificate get_one_storage_ssl_certificate(name)
Return one SSL certificate from disk

Returns one SSL certificate from disk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | SSL certificate name | [required] |

### Return type

[**crate::models::SslCertificate**](ssl_certificate.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_storage_general_file

> replace_storage_general_file(name, body, skip_reload, force_reload)
Replace contents of a managed general use file on disk

Replaces the contents of a managed general use file on disk

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | General use file storage_name | [required] |
**body** | **String** |  | [required] |
**skip_reload** | Option<**bool**> | If set, no reload will be initiated after update |  |[default to false]
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_storage_map_file

> replace_storage_map_file(name, body, skip_reload, force_reload)
Replace contents of a managed map file on disk

Replaces the contents of a managed map file on disk

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Map file storage_name | [required] |
**body** | **String** |  | [required] |
**skip_reload** | Option<**bool**> | If set, no reload will be initiated after update |  |[default to false]
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_storage_ssl_certificate

> crate::models::SslCertificate replace_storage_ssl_certificate(name, body, skip_reload, force_reload)
Replace SSL certificates on disk

Replaces SSL certificate on disk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | SSL certificate name | [required] |
**body** | **String** |  | [required] |
**skip_reload** | Option<**bool**> | If set, no reload will be initiated after update |  |[default to false]
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::SslCertificate**](ssl_certificate.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

