# PeerSection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_bind** | Option<[**crate::models::DefaultBind**](default_bind.md)> |  | [optional]
**default_server** | Option<[**crate::models::DefaultServer**](default_server.md)> |  | [optional]
**disabled** | Option<**bool**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**name** | **String** |  | 
**shards** | Option<**i32**> | In some configurations, one would like to distribute the stick-table contents to some peers in place of sending all the stick-table contents to each peer declared in the \"peers\" section. In such cases, \"shards\" specifies the number of peer involved in this stick-table contents distribution. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


