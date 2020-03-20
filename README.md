# chest

distributed data store

## semantics

* writes can go to any node
* writes get sync'd with peers
* reads are eventually consistent
* hardcoded host discovery (for now)

### network

* client forms payload with magic byte header
* client sends payload
* client ends with no more magic byte
* server ack's
* client closes connection
