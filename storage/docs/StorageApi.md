# \StorageApi

All URIs are relative to *https://api.azion.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**storage_api_buckets_create**](StorageApi.md#storage_api_buckets_create) | **POST** /v4/storage/buckets | Create a new bucket
[**storage_api_buckets_destroy**](StorageApi.md#storage_api_buckets_destroy) | **DELETE** /v4/storage/buckets/{name} | Delete a bucket
[**storage_api_buckets_list**](StorageApi.md#storage_api_buckets_list) | **GET** /v4/storage/buckets | List buckets
[**storage_api_buckets_objects_create**](StorageApi.md#storage_api_buckets_objects_create) | **POST** /v4/storage/buckets/{bucket_name}/objects/{object_key} | Create new object key
[**storage_api_buckets_objects_destroy**](StorageApi.md#storage_api_buckets_objects_destroy) | **DELETE** /v4/storage/buckets/{bucket_name}/objects/{object_key} | Delete object key
[**storage_api_buckets_objects_list**](StorageApi.md#storage_api_buckets_objects_list) | **GET** /v4/storage/buckets/{bucket_name}/objects | List buckets objects
[**storage_api_buckets_objects_retrieve**](StorageApi.md#storage_api_buckets_objects_retrieve) | **GET** /v4/storage/buckets/{bucket_name}/objects/{object_key} | Download object
[**storage_api_buckets_objects_update**](StorageApi.md#storage_api_buckets_objects_update) | **PUT** /v4/storage/buckets/{bucket_name}/objects/{object_key} | Update the object key
[**storage_api_buckets_partial_update**](StorageApi.md#storage_api_buckets_partial_update) | **PATCH** /v4/storage/buckets/{name} | Update bucket info
[**storage_api_s3_credentials_by_access_key**](StorageApi.md#storage_api_s3_credentials_by_access_key) | **GET** /v4/storage/s3-credentials/{s3_credential_access_key} | get by s3 credentials by access key
[**storage_api_s3_credentials_create**](StorageApi.md#storage_api_s3_credentials_create) | **POST** /v4/storage/s3-credentials | create s3 credentials
[**storage_api_s3_credentials_delete**](StorageApi.md#storage_api_s3_credentials_delete) | **DELETE** /v4/storage/s3-credentials/{s3_credential_access_key} | delete by s3 credentials
[**storage_api_s3_credentials_list**](StorageApi.md#storage_api_s3_credentials_list) | **GET** /v4/storage/s3-credentials | List s3 credentials



## storage_api_buckets_create

> models::ResponseBucket storage_api_buckets_create(bucket_create)
Create a new bucket



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_create** | [**BucketCreate**](BucketCreate.md) |  | [required] |

### Return type

[**models::ResponseBucket**](ResponseBucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_api_buckets_destroy

> models::SuccessBucketOperation storage_api_buckets_destroy(name)
Delete a bucket



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::SuccessBucketOperation**](SuccessBucketOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_api_buckets_list

> models::PaginatedBucketList storage_api_buckets_list(page, page_size)
List buckets



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |

### Return type

[**models::PaginatedBucketList**](PaginatedBucketList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_api_buckets_objects_create

> models::SuccessObjectOperation storage_api_buckets_objects_create(bucket_name, object_key, content_type, body)
Create new object key

Create a new object key in the bucket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_name** | **String** |  | [required] |
**object_key** | **String** |  | [required] |
**content_type** | Option<**String**> | The content type of the file (Example: text/plain). |  |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::SuccessObjectOperation**](SuccessObjectOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_api_buckets_objects_destroy

> models::SuccessObjectOperation storage_api_buckets_objects_destroy(bucket_name, object_key)
Delete object key

Delete an object key from bucket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_name** | **String** |  | [required] |
**object_key** | **String** |  | [required] |

### Return type

[**models::SuccessObjectOperation**](SuccessObjectOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_api_buckets_objects_list

> models::PaginatedBucketObjectList storage_api_buckets_objects_list(bucket_name, continuation_token, max_object_count)
List buckets objects



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_name** | **String** |  | [required] |
**continuation_token** | Option<**String**> | Token for next page. |  |
**max_object_count** | Option<**i32**> | Number of results to return per page. |  |

### Return type

[**models::PaginatedBucketObjectList**](PaginatedBucketObjectList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_api_buckets_objects_retrieve

> storage_api_buckets_objects_retrieve(bucket_name, object_key)
Download object

Download the object key from bucket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_name** | **String** |  | [required] |
**object_key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html, application/json, application/xml, text/plain, image/jpeg, image/png, image/gif, video/mp4, audio/mpeg, application/pdf, application/javascript, text/css, application/octet-stream, multipart/form-data, application/x-www-form-urlencoded

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_api_buckets_objects_update

> models::SuccessObjectOperation storage_api_buckets_objects_update(bucket_name, object_key, content_type, body)
Update the object key

Update the object key from bucket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_name** | **String** |  | [required] |
**object_key** | **String** |  | [required] |
**content_type** | Option<**String**> | The content type of the file (Example: text/plain). |  |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::SuccessObjectOperation**](SuccessObjectOperation.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_api_buckets_partial_update

> models::ResponseBucket storage_api_buckets_partial_update(name, bucket_update)
Update bucket info



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**bucket_update** | Option<[**BucketUpdate**](BucketUpdate.md)> |  |  |

### Return type

[**models::ResponseBucket**](ResponseBucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_api_s3_credentials_by_access_key

> models::ResponseS3Credential storage_api_s3_credentials_by_access_key(s3_credential_access_key)
get by s3 credentials by access key



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**s3_credential_access_key** | **String** |  | [required] |

### Return type

[**models::ResponseS3Credential**](ResponseS3Credential.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_api_s3_credentials_create

> models::ResponseS3Credential storage_api_s3_credentials_create(s3_credential_create)
create s3 credentials



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**s3_credential_create** | [**S3CredentialCreate**](S3CredentialCreate.md) |  | [required] |

### Return type

[**models::ResponseS3Credential**](ResponseS3Credential.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_api_s3_credentials_delete

> models::ResponseS3Credential storage_api_s3_credentials_delete(s3_credential_access_key)
delete by s3 credentials



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**s3_credential_access_key** | **String** |  | [required] |

### Return type

[**models::ResponseS3Credential**](ResponseS3Credential.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_api_s3_credentials_list

> models::PaginatedS3CredentialList storage_api_s3_credentials_list(key, last_modified, size, continuation_token)
List s3 credentials



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | Option<**String**> | Object key. Used to identify the object for requests. Sent in POST requests as a path variable. |  |
**last_modified** | Option<**String**> | Timestamp of the last modification to the object. |  |
**size** | Option<**i32**> | Size of file in bytes. |  |
**continuation_token** | Option<**String**> | Hash that can be added to the continuation_token query to skip list to the next page. |  |

### Return type

[**models::PaginatedS3CredentialList**](PaginatedS3CredentialList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

