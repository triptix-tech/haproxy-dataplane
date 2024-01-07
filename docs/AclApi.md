# \AclApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_acl**](AclApi.md#create_acl) | **POST** /services/haproxy/configuration/acls | Add a new ACL line
[**delete_acl**](AclApi.md#delete_acl) | **DELETE** /services/haproxy/configuration/acls/{index} | Delete a ACL line
[**get_acl**](AclApi.md#get_acl) | **GET** /services/haproxy/configuration/acls/{index} | Return one ACL line
[**get_acls**](AclApi.md#get_acls) | **GET** /services/haproxy/configuration/acls | Return an array of all ACL lines
[**replace_acl**](AclApi.md#replace_acl) | **PUT** /services/haproxy/configuration/acls/{index} | Replace a ACL line



## create_acl

> crate::models::Acl create_acl(parent_name, parent_type, acl, transaction_id, version, force_reload)
Add a new ACL line

Adds a new ACL line of the specified type in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**acl** | [**Acl**](Acl.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Acl**](acl.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_acl

> delete_acl(index, parent_name, parent_type, transaction_id, version, force_reload)
Delete a ACL line

Deletes a ACL line configuration by it's index from the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | ACL line Index | [required] |
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


## get_acl

> crate::models::GetAcl200Response get_acl(index, parent_name, parent_type, transaction_id)
Return one ACL line

Returns one ACL line configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | ACL line Index | [required] |
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetAcl200Response**](getAcl_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_acls

> crate::models::GetAcls200Response get_acls(parent_name, parent_type, acl_name, transaction_id)
Return an array of all ACL lines

Returns all ACL lines that are configured in specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**acl_name** | Option<**String**> | ACL name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetAcls200Response**](getAcls_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_acl

> crate::models::Acl replace_acl(index, parent_name, parent_type, acl, transaction_id, version, force_reload)
Replace a ACL line

Replaces a ACL line configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | ACL line Index | [required] |
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**acl** | [**Acl**](Acl.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Acl**](acl.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

