# chest

distributed data store

## semantics

* writes can go to any node
* writes get sync'd with peers
* reads are eventually consistent
* hardcoded host discovery (for now)
