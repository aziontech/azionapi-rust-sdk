# Rust API client for storage

REST API OpenAPI documentation for the Object Storage


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0 (v1)
- Package version: 1.0.0 (v1)
- Generator version: 7.4.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `storage` and add the following to `Cargo.toml` under `[dependencies]`:

```
storage = { path = "./storage" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.azion.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*StorageApi* | [**storage_api_buckets_create**](docs/StorageApi.md#storage_api_buckets_create) | **POST** /v4/storage/buckets | Create a new bucket
*StorageApi* | [**storage_api_buckets_destroy**](docs/StorageApi.md#storage_api_buckets_destroy) | **DELETE** /v4/storage/buckets/{name} | Delete a bucket
*StorageApi* | [**storage_api_buckets_list**](docs/StorageApi.md#storage_api_buckets_list) | **GET** /v4/storage/buckets | List buckets
*StorageApi* | [**storage_api_buckets_objects_create**](docs/StorageApi.md#storage_api_buckets_objects_create) | **POST** /v4/storage/buckets/{bucket_name}/objects/{object_key} | Create new object key
*StorageApi* | [**storage_api_buckets_objects_destroy**](docs/StorageApi.md#storage_api_buckets_objects_destroy) | **DELETE** /v4/storage/buckets/{bucket_name}/objects/{object_key} | Delete object key
*StorageApi* | [**storage_api_buckets_objects_list**](docs/StorageApi.md#storage_api_buckets_objects_list) | **GET** /v4/storage/buckets/{bucket_name}/objects | List buckets objects
*StorageApi* | [**storage_api_buckets_objects_retrieve**](docs/StorageApi.md#storage_api_buckets_objects_retrieve) | **GET** /v4/storage/buckets/{bucket_name}/objects/{object_key} | Download object
*StorageApi* | [**storage_api_buckets_objects_update**](docs/StorageApi.md#storage_api_buckets_objects_update) | **PUT** /v4/storage/buckets/{bucket_name}/objects/{object_key} | Update the object key
*StorageApi* | [**storage_api_buckets_partial_update**](docs/StorageApi.md#storage_api_buckets_partial_update) | **PATCH** /v4/storage/buckets/{name} | Update bucket info


## Documentation For Models

 - [Bucket](docs/Bucket.md)
 - [BucketCreate](docs/BucketCreate.md)
 - [BucketObject](docs/BucketObject.md)
 - [EdgeAccessEnum](docs/EdgeAccessEnum.md)
 - [ObjectResponseData](docs/ObjectResponseData.md)
 - [PaginatedBucketList](docs/PaginatedBucketList.md)
 - [PaginatedBucketObjectList](docs/PaginatedBucketObjectList.md)
 - [ResponseBucket](docs/ResponseBucket.md)
 - [StateEnum](docs/StateEnum.md)
 - [SuccessBucketOperation](docs/SuccessBucketOperation.md)
 - [SuccessObjectOperation](docs/SuccessObjectOperation.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


