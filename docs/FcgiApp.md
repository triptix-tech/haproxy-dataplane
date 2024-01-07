# FcgiApp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**docroot** | **String** | Defines the document root on the remote host. The parameter serves to build the default value of FastCGI parameters SCRIPT_FILENAME and PATH_TRANSLATED. It is a mandatory setting. | 
**get_values** | Option<**String**> | Enables or disables the retrieval of variables related to connection management. | [optional]
**index** | Option<**String**> | Defines the script name to append after a URI that ends with a slash (\"/\") to set the default value for the FastCGI parameter SCRIPT_NAME. It is an optional setting. | [optional]
**keep_conn** | Option<**String**> | Tells the FastCGI application whether or not to keep the connection open after it sends a response. If disabled, the FastCGI application closes the connection after responding to this request. | [optional]
**log_stderrs** | Option<[**Vec<crate::models::FcgiLogStderr>**](fcgiLogStderr.md)> |  | [optional]
**max_reqs** | Option<**i32**> | Defines the maximum number of concurrent requests this application can accept. If the FastCGI application retrieves the variable FCGI_MAX_REQS during connection establishment, it can override this option. Furthermore, if the application does not do multiplexing, it will ignore this option. | [optional][default to 1]
**mpxs_conns** | Option<**String**> | Enables or disables the support of connection multiplexing. If the FastCGI application retrieves the variable FCGI_MPXS_CONNS during connection establishment, it can override this option. | [optional]
**name** | **String** | Declares a FastCGI application | 
**pass_headers** | Option<[**Vec<crate::models::FcgiPassHeader>**](fcgiPassHeader.md)> |  | [optional]
**path_info** | Option<**String**> | Defines a regular expression to extract the script-name and the path-info from the URI. Thus, <regex> must have two captures: the first to capture the script name, and the second to capture the path- info. If not defined, it does not perform matching on the URI, and does not fill the FastCGI parameters PATH_INFO and PATH_TRANSLATED. | [optional]
**set_params** | Option<[**Vec<crate::models::FcgiSetParam>**](fcgiSetParam.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


