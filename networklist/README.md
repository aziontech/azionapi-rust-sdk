# Rust API client for networklist

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 3.0
- Package version: 3.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `networklist` and add the following to `Cargo.toml` under `[dependencies]`:

```
networklist = { path = "./networklist" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.azionapi.net*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**network_lists_get**](docs/DefaultApi.md#network_lists_get) | **GET** /network_lists | List all user Network Lists
*DefaultApi* | [**network_lists_post**](docs/DefaultApi.md#network_lists_post) | **POST** /network_lists | Create a Network Lists
*DefaultApi* | [**network_lists_uuid_get**](docs/DefaultApi.md#network_lists_uuid_get) | **GET** /network_lists/{uuid} | Retrieve a Network Lists set by uuid
*DefaultApi* | [**network_lists_uuid_put**](docs/DefaultApi.md#network_lists_uuid_put) | **PUT** /network_lists/{uuid} | Overwrite some Network Lists attributes


## Documentation For Models

 - [BadRequestResponse](docs/BadRequestResponse.md)
 - [CreateNetworkListsRequest](docs/CreateNetworkListsRequest.md)
 - [ErrorModel](docs/ErrorModel.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [Links](docs/Links.md)
 - [ListNetworkListsResponse](docs/ListNetworkListsResponse.md)
 - [NetworkLists](docs/NetworkLists.md)
 - [NetworkListsResponse](docs/NetworkListsResponse.md)
 - [UpdateNetworkListsRequest](docs/UpdateNetworkListsRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


