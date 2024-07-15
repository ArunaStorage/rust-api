# Aruna - Rust API
This repo contains the Rust API builds for Aruna. It is derived from the language agnostic Protocol Buffers [API Definitions](https://github.com/ArunaStorage/api).


## Structure

The API contains three main sections:

- [Storage section](#storage): This is the main section for external use. It contains a basic set of services and models that describe the interfaces with the storage system.

- [Notification section](#notifications): This section contains a set of services and models that describe the interfaces with the notification system.

- [Hooks](#hooks): This section contains the service that can be used to extend Aruna with external functionality or automate internal processes.


### Storage

The storage section is divided in two sub-sections:

- [Models](https://github.com/ArunaStorage/api/tree/main/aruna/api/storage/models/v2): This section contains the models that are used by the storage system. 

- [Storage services](https://github.com/ArunaStorage/api/tree/main/aruna/api/storage/services/v2/): This section contains all services that are used to interact with the storage system. Services are defined as RPCs and are grouped by object type.

### Notifications

The Notification section provides a set of RPCs that are used to interact with the notification system. The notification system uses [nats.io](https://nats.io/) as its underlying service. The service definition can be found [here](https://github.com/ArunaStorage/api/tree/main/aruna/api/notification/services/v2/notification_service.proto).


### Hooks

Hooks are the way to automate internal processes in Aruna and/or to integrate external services to extend functionality. Once created, they're available globally in Aruna, and Projects must be associated with them to be included in their trigger cycle. The action that triggers the specific hook is defined by its trigger type.



## License

The API is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option. Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions. 
