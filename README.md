# Aruna Object Storage (AOS) rust-api
This repo contains the Rust api builds for the Aruna storage. It is derived from the [API Definitions](https://github.com/ArunaStorage/ArunaAPI).


## Structure

The API contains three main sections:

- [Storage section](#storage): This is the main section for external use. It contains a basic set of services and models that describe the interfaces with the storage system.

- [Notification section](#notifications): This section contains a set of services and models that describe the interfaces with the notification system.

- [Internal section](#internal): This section is for internal use only. It contains a set of internal services and apis that are used by different internal components of the system.


### Storage

The storage section is divided in two sub-sections:

- [Models](aruna/api/storage/models/v1): This section contains the models that are used by the storage system. 

- [Storage services](aruna/api/storage/services/v1/): This section contains all services that are used to interact with the storage system. Services are defined as RPCs and are grouped by object type.

### Notifications

The Notification section provides a set of RPCs that are used to interact with the notification system. The notification system uses [nats.io](https://nats.io/) as its underlying service. The service definition can be found [here](notifications/services/v1/notification_service.proto).


### Internal

This contains definitions for internal APIs that are used by different (micro) services internally. These endpoints are not exposed to external users. Currently the main use is the interaction with the storage proxy that bundles multiple storage methods (S3, local file system etc.) in one service that provides stable pre-authenticated up- and download URLs for users.



## License

The API is licensed under the [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) license. See the [License](LICENSE.md) file for more information.