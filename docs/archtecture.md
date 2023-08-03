# Understanding the Architecture of AppFlowy

The presented PlantUML diagrams outline the architecture and workflow of a collaborative
document editor, referred to as the AppFlowy application. It highlights the roles of
different components and how they interact to create, open, edit, and synchronize documents.

### Architecture of AppFlowy

The AppFlowy application comprises of three core components: flowy-folder, flowy-database,
and flowy-document. The 'Collab' and 'CollabPlugins' are additional components involved in
data management and synchronization, connecting the core components of AppFlowy to various
data storage and synchronization plugins.

AppFlowy is designed to interact with different databases through the CollabPlugins. 
Currently, it supports RocksDB and Supabase. These databases are chosen for their superior
performance, high availability, and capability to handle substantial data volumes.

The core components of the AppFlowy application interact with their corresponding elements
in the 'Collab' component. The 'Collab' component further connects to the 'CollabPlugins' 
to ensure the data is stored and updated appropriately across various databases.

![](./collab_object.png)
### Creating a Document

When a user creates a document in the AppFlowy application, the request is first processed 
by the flowy-folder. The request then propagates to the flowy-document, creating a corresponding
'collab' document.

Once the document is created, the 'Collab' component pushes the updates to all connected plugins
via the 'CollabPlugins' component. This ensures that the document creation is reflected in both
RocksDB and Supabase databases, preserving the state of the document and ensuring its availability.

![](./collab_object-Create_Document.png)

### Opening a Document

When a user opens a document, the flowy-document component retrieves the document data. 
The collab_document component then invokes the 'did_init' method on all plugins to initialize
any required operations. This is followed by sending an init sync request to the server 
through the SupabaseDBPlugin.

Once the server has processed the init sync request, it sends a response back to the plugins,
ensuring that the most recent version of the document is loaded.

![](./collab_object-Open_Document.png)
### Editing a Document

During the editing phase, any user input is first processed by the flowy-document component. 
The input updates are then sent to the collab_document component, which pushes these updates to the respective plugins.

The RocksDB plugin saves the updates to the disk, ensuring data persistence. Concurrently, 
the SupabaseDBPlugin pushes the updates to the send queue, from which they are sent to the Supabase Cloud server. This ensures that the document is consistently updated in real-time across multiple databases.

![](./collab_object-Edit_Document.png)

### Document Synchronization

AppFlowy's architecture supports multi-user collaboration with real-time document synchronization.
When a user makes an update, the changes are sent to the server through the SupabaseDBPlugin.
Once the server acknowledges the update, it broadcasts the changes to all other connected users using
the Realtime service.

When other users receive these updates, they apply the changes to their local collab component, 
which updates their UI. This feature ensures that all users are always working on the most recent 
version of the document, promoting effective collaboration.

![](./collab_object-Sync_Document.png)
