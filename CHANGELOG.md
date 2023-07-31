# Change Log

## 1.7.5 Nightly 1
- Support for SurrealDB nightly (Commit 860796b)
- Experimental support for connecting to remote HTTP endpoints
	- The remote database no longer requires an SSL certificate

## 1.7.5 - Bug fixes
- Remove UI zooming due to editor issues (#78)
	- We intend to re-add this feature in the future once [upstream issues](https://github.com/tauri-apps/tauri/issues/3310) are resolved 
- More accurately report on local database launch failures (#76)
	- The console panel can now be toggled at any time allowing you to see potential errors
- Add refresh button to inspector panel
- Respect XDG_CONFIG_HOME for locating the settings file (#67)
- Fix some panel sizing issues

## 1.7.4 - Inspector history
- Added back and forward buttons to the inspector panel
	- More easily navigate when editing multiple records
- Fixed inspector pane opening too small
- Inspector pane now remembers its size
- Fixed comment shortcut using the wrong keys (#58)
- Various UI improvements

## 1.7.3 - UI Improvements
- Added the ability to drag sessions to customize their order
- Add support for latest SurrealDB nightly builds (#64)
- Correctly handle databases with no tables (#66)
- Fixed panels sometimes breaking when resizing the window
- Redesigned settings dialog
	- Session search box is now a setting

## 1.7.2 - Improved error highlighting
- Improved the error highlighter
	- Results in significantly less false positives
	- Now accurately highlights the position of the syntax error
	- Correctly handles comments
- Fixed visualizer no longer displaying edges in newer versions of SurrealDB
- Dismissed update notifications will be hidden until a new version is available
- Added manual update check button in settings

## 1.7.1 - Live error checking
- Added live error checking to the query view
	- Underlines invalid queries with an underline
	- Displays the error message on hover
	- Can be disabled in the settings dialog
- Fixed some connection UI issues
- Updated internal version of surrealdb

## 1.7.0 - Environments
- Replaced the tab system with a new session system
	- Sessions are organized into environments
	- Environments can define default connection details which all sessions within fallback to
	- Provides a significantly better user experience when defining many sessions
	- Sessions can be pinned directly to the toolbar for quick access
	- Existing tabs are automatically migrated to sessions
- Added filtering functionality to the explorer view
	- Filter input can be toggled by pressing the filter icon on the top right of the explorer panel
	- Validates the where clause and indicates when it is invalid
- Local database starting is now decoupled from the active sessions
	- Root username, root password, and port are now configured in settings
- Overhauled the settings UI to be more scalable
- Improve table retrieval performance for explorer and designer view
- The surreal executable path can now also be manually specified in settings
- Changed the way multiple responses are displayed in query view
	- Tabs have been replaced by pagination
- Surrealist windows can be resized to a slightly smaller size than before
- Fixed database start button getting stuck in loading state

## 1.6.4 - Minor improvements
- Updated internal version of surrealdb
- Fixed surrealdb process sometimes staying alive after exit
- Fixed unexpected behavior with permission inputs in designer view (#46)

## 1.6.3 - UI/UX Improvements
- Records are now opened in the explorer when the entire row is clicked
- Passwords are no longer displayed in plaintext by default
- Fixed wrapping issues with record links
- Close explorer when table is deleted
- Improved performance of total record count calculation
- Fixed inconsistent row heights in explorer
- Alphabetically sort headers
- Impose a visual limit on the length of record links
- Fixed update notification not opening the releases page
- Fixed incorrect colors in query view table renderer
- Scope name is now displayed in the connection bar
- Further attempts to fix spaces behaving weirdly when selected

## 1.6.2 - Visualization export
- Improvements to the visualizer view
	- Graphs can be saved as png and jpg
	- Improved node layout algorithm
	- Fixed labels dissapearing close to the edge
- Add new "Export schema" toolbar button in designer view
	- Exports the database schema to a `.surql` file
	- Unlike `surreal export` this file only contains schema definitions, not data
	- Useful when you want to save an existing schema and apply it to another database
- Added new "Advanced editor" buttons to some Designer view inputs
	- Opens a dedicated dialog with a multiline syntax highted editor
	- The regular input boxes can still be used and now accept multilines
- Scope authentication fields can now be fully customized
- Remember the active tab between application restarts
- Allow selecting multiple from and to tables when creating edges
- Improved table sorting in explorer and designer view
- Fixed an issue in the table view renderer causing a crash
- Improve preview of array items within table views
- Table headers are now sticky and always visible
- Fixed various visual issues and inconsistencies
- Updated to latest SurrealDB nightly

## 1.6.1 - Schema management
- Added designer view
	- Allows graphically creating and modifying schemas
	- Supports permissions, fields, indexes, and events
	- Create new tables and edge tables
- Added visualizer view
	- Plots your tables and relationships into a graph
	- Provides a useful overview of your database design
	- Early stages, will gain more functionality in the future
- Added authentication view
	- Manage namespace & database logins
	- Manage database scopes
- Rewritten backend in Rust
	- Will allow for more intelligent features in the future
	- Dev tools are now available in release builds (Ctrl + Shift + I)
- Added the ability to search table lists
- Allow executing queries from variables pane
- Highlighting for single quotes and &lt;future&gt;
- Added "Execute selection" option to query editor context menu
- Improved matching of certain types in explorer view
- The current view is now scoped to the tab instead of global
- Edge tables now display a different icon from normal tables
- Use left and right arrow keys to cycle between the available modes
- Fixed incorrect UI state due to lack of access (anonymous & scope auth)
- Fixed crash on connecting to insecure endpoint

## 1.5.2 - Bug fixes
- Add new anonymous connection authentication mode
- Improve record link detection
- Fixed database and scope authentication not working
- Add more syntax highlighting keywords
- Fixed certain functions not being matched correctly
- Fixed crash when editing a saved query
- Fixed horizontal scrollbar in history and favorites panel

## 1.5.1 - New shortcuts
- Added new keyboard shortcuts
	- `Ctrl + number` to switch to another tab
	- `Ctrl + Q` to switch to the Query View
	- `Ctrl + E` to switch to the Explorer View
	- `Ctrl + S` to start / stop the local server
	- `Ctrl + -` to zoom out
	- `Ctrl + +` to zoom in
- Added editor zooming
	- Experimental, may not work on all systems (This cannot be fixed at the moment)
	- Can cause issues with draggable elements
- Added the ability to create new records in explorer view
- Added the ability to delete records in explorer view
- Add button to disconnect from the database when connected
- Improve automatic theme and prevent potential flashing of the wrong theme
- Fixed history not saving when all entries are cleared
- Added syntax highlighting for SurrealDB functions
- Improve WebSocket handling to result in more consistent behavior
- Fixed table view incorrectly marking certain strings as records
- Tweak some default values
- Various UI/UX improvements

## 1.5.0 - Explorer view
- Added an explorer view to view and edit tables and their data
	- You can switch between Query and Explorer view using the button in the top left
	- Edit records directly as JSON
	- View and traverse record relations
	- Columns can be sorted by clicking on the header
- Redesigned the table view
	- Improved cell rendering based on data type
	- Objects and arrays can be hovered for a preview
	- Null and undefined values are now styled differently
- Added namespace, database, and scope authentication
	- Redesigned the connection details dialog to include new fields
- Added additional keywords for highlighting
- Table completion will now be triggered in more situations
- Tabs can be dragged to reorder them
- Improved favorites dragging behavior

## 1.4.2 - Improved favorites
- Redesigned the favorites UI/UX
	- Queries are now be opened by clicking rather than hovering
	- You can now edit and rename queries in a dialog
	- Favorites can be manually sorted by dragging
	- Added the option to open a query in a new tab
- New tabs now use the connection details of the previous tab
- Hide the Surreal banner in the console
- Improved the pane dragging behavior
- Fixed text moving within the editor when selected

## 1.4.1 - Saved queries (Fixed)
- Highlight regex correctly in the query editor
- Fixed updater not working
- Fixed issues with the table viewer
- Improve database start/stop button behavior
- Changed default tab names

## 1.4.0 - Saved queries
- Added query saving functionality
- Added a button to render query results as table
- Redesigned the query history UI
- Added F9 as shortcut for "Execute query"
- F9 and Ctrl + Space now work outside the editor
- Improved the panel resizing behavior

## 1.3.0 - Console panel
- Added a console panel to view logs for the integrated database (#1)
	- Can be hidden and revealed on the fly or from the setting screen
- Disabled uneccecary auto completion for input fields (#3)
- Added configurable global query timeout setting
- Added update checker to prompt about new releases
	- Can be disabled from the settings screen
- Added comment toggle shortcut to the query editor (Ctrl + /)
- Updated the setting screen UI

## 1.2.0 - Local database
- Added the ability to start and stop a local database directly from Surrealist
	- Will use the username, password, and port entered for the current tab
	- The tab hosting the database will display an indication icon
- Added an "Automatic" theme option that uses your operating system theme
	- This is now the default
- Added auto completion support for variables

## 1.1.2 - Query history
- Query history drawer
- Result wrapping setting
- Ctrl+Space to suggest table names at any time
- Various UI fixes

## 1.1.1 - UX Improvements
- Fixed indentation issues
- Improved query result consistency
- Added auto completion for table names

## 1.1.0 - Feature complete
- Auto connect when switching tabs
- Dark theme support
- Variables pane
- Improved error handling
- Various UI fixes

## 1.0.0 - Initial release
- Multi-tab query editing
- Support for multiple queries in one request
- Provides a clean and foldable view of your query result
- SurrealQL syntax highlighting