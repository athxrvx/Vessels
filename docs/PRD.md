# VESSELS
## Product Requirements Document (PRD)

**Product:** Vessels  
**Category:** Container management and development environment management  
**Primary platform:** COSMIC Desktop  
**Primary distribution:** Flatpak  
**Primary language:** Rust  
**UI framework:** libcosmic  
**Runtime targets:** Podman and Docker  
**Design philosophy:** Native, minimal, adaptive, card-driven, keyboard-accessible  
**Document status:** Product specification / initial engineering baseline

---

# 1. Product vision

## 1.1 Overview

Vessels is a native COSMIC desktop application for discovering, configuring, creating, organizing and managing containerized development environments.

It combines container lifecycle management, OCI image discovery, project management, multi-container orchestration and host editor integration into one cohesive desktop experience.

Vessels aims to make container-based development approachable without sacrificing the flexibility and control expected by experienced developers.

Rather than presenting containers, images and development environments as unrelated technical objects, Vessels connects them through a unified workflow:

1. Discover an image from a container registry.
2. Configure the image for a development workload.
3. Create and manage containers.
4. Organize containers into groups.
5. Associate containers and environments with projects.
6. Open complete multi-container environments.
7. Access terminals, logs, storage and networking from one workspace.
8. Launch locally installed editors against the development environment.

## 1.2 Product objectives

Vessels must:

- Provide a first-class COSMIC-native experience.
- Make container management visually intuitive.
- Simplify OCI image discovery across multiple registries.
- Support individual containers and manually organized container groups.
- Provide project-oriented development environment management.
- Support multiple services within one project environment.
- Offer integrated multi-terminal and multi-service management.
- Work with existing Podman and Docker installations.
- Respect Flatpak sandboxing and desktop security conventions.
- Remain lightweight, responsive and maintainable.

## 1.3 Product positioning

Vessels is not intended to replace every capability of Docker Desktop, Kubernetes management software or a complete IDE.

Its primary purpose is to provide a polished desktop experience for local container-based development.

Its defining capabilities are:

| Capability | Product value |
|---|---|
| Container management | Manage local container instances |
| Image Hub | Discover and configure OCI images |
| Container grouping | Organize related containers |
| Project management | Associate development resources with workspaces |
| Environment orchestration | Manage multiple services as one environment |
| Environment workspace | Access all services through one interface |
| Editor integration | Connect host editors to development environments |
| COSMIC integration | Deliver a cohesive native desktop experience |

## 1.4 Target users

**Primary users**

- Linux developers.
- COSMIC desktop users.
- Students and software engineering teams.
- Developers using Podman or Docker.
- Developers working with multiple services.
- Users who prefer graphical container management.

**Secondary users**

- System administrators managing local containers.
- Users experimenting with different Linux distributions.
- Developers maintaining local databases, testing services and isolated toolchains.

## 1.5 Non-goals

The initial product is not intended to:

- Replace Kubernetes orchestration platforms.
- Provide a complete cloud container management service.
- Implement a new container runtime.
- Replace Docker or Podman's image-building infrastructure.
- Become a complete IDE.
- Automatically install arbitrary host applications.
- Provide a full virtual machine management system.
- Manage production infrastructure as its primary use case.

---

# 2. Platform and technology requirements

## 2.1 COSMIC-first development

COSMIC is the primary desktop target, not simply one of several supported desktop environments.

Vessels must use the COSMIC design language and native application conventions.

The application should use:

- Rust as its primary implementation language.
- libcosmic for its interface.
- COSMIC theming and system appearance integration.
- Native COSMIC controls and widgets.
- Wayland-native windowing.
- Standard desktop integration interfaces.
- XDG conventions for configuration, cache and application data.

The interface must not resemble a generic web dashboard or a GNOME application with a modified theme.

### Design requirements

- Follow COSMIC spacing, typography and component conventions.
- Respect system light and dark themes.
- Respect system accent colour where supported.
- Support adaptive layouts.
- Use native dialogs, popovers, menus and contextual controls.
- Provide consistent keyboard navigation.
- Avoid unnecessary custom widget implementations.
- Avoid gradients, excessive shadows, glow effects and glassmorphism.
- Prefer clear hierarchy over decorative complexity.

## 2.2 Flatpak-first distribution

The Flatpak package is the primary distribution target.

The application must be designed around the restrictions and security model of Flatpak.

The Flatpak must not depend on unrestricted host filesystem access or arbitrary host command execution.

### Flatpak requirements

- Provide a reproducible Flatpak build.
- Follow Flatpak application metadata conventions.
- Use a stable application ID.
- Store application configuration and state using XDG directories.
- Use portals for user-selected files and directories where appropriate.
- Request only necessary sandbox permissions.
- Avoid unrestricted session and system D-Bus access.
- Avoid blanket host filesystem access.
- Avoid requiring root privileges for normal operations.
- Provide clear diagnostics when host runtime integration is unavailable.

### Important runtime integration constraint

A Flatpak application cannot simply assume that it can access the host's Docker or Podman socket.

Runtime connectivity must be treated as an explicit integration problem.

The application should support a configurable runtime connection layer with these approaches:

1. Rootless Podman API socket.
2. Docker API socket where explicitly configured.
3. Remote Podman or Docker-compatible endpoints.
4. An optional, separately packaged host-side integration service if necessary.

The initial implementation should investigate and validate rootless Podman socket access from a Flatpak sandbox before committing to a permanent permission strategy.

Do not silently grant unrestricted host filesystem access to make runtime integration work.

Docker socket access must be treated as highly privileged because control of the daemon can provide extensive control over the host.

### Filesystem access

Projects must be selected through a file chooser or directory picker.

Vessels should retain access through the available document portal mechanisms where supported.

The application must not assume that a project directory selected through the portal can automatically be accessed by a separately running container runtime.

Host-side bind mounts require a clearly defined host-path resolution strategy.

If a selected directory is unavailable to the runtime, Vessels must explain the problem and provide a guided resolution.

### Host editor integration

Editors installed on the host must be discoverable without assuming that the Flatpak can inspect arbitrary host executable directories.

Investigate the following integration mechanisms:

- Desktop application metadata.
- XDG desktop application discovery through an explicit integration layer.
- D-Bus application activation where supported.
- XDG OpenURI portal.
- A narrowly scoped host-side integration helper where required.

The application must distinguish between:

- Editor detected.
- Editor launchable.
- Editor supports remote development.
- Editor can access the selected project directory.
- Editor can connect to the selected container environment.

These are separate capabilities and must not be presented as interchangeable.

---

# 3. Global application architecture

## 3.1 Primary navigation

The application contains six main navigation destinations.

| Navigation | Purpose |
|---|---|
| Containers | Individual container management and manual groups |
| Overview | All-in-one application command centre |
| Hub | OCI registry discovery and image management |
| Projects | Development workspaces and environments |
| Editors | Host editor detection and integration |
| Settings | Application and runtime configuration |

**Default landing page: Containers.**

Vessels must open directly into the Containers page on startup unless the user has explicitly configured another startup destination in a future version.

## 3.2 Application shell

The application shell consists of:

- Navigation sidebar.
- Main content region.
- Global search.
- Global activity indicator.
- Notifications access.
- Application-level contextual actions.

### Sidebar

The sidebar should remain compact and readable.

Navigation items:

- Containers
- Overview
- Hub
- Projects

A subtle divider separates the primary destinations from:

- Editors
- Settings

The sidebar should support appropriate collapsed and expanded layouts if the window size permits.

### Global search

Global search should eventually index:

- Containers.
- Container groups.
- Images.
- Projects.
- Environments.
- Services.
- Registries.

Search results must be categorized and navigable.

### Global activity

Long-running operations must be visible throughout the application.

Examples:

- Pulling an image.
- Creating a container.
- Starting an environment.
- Building an image.
- Removing unused images.
- Connecting to a runtime.

Users must be able to inspect operation progress and errors without leaving their current page.

## 3.3 Visual language

The interface should be card-driven.

Cards are the primary representation of independently manageable resources.

Examples:

- Container cards.
- Project cards.
- Environment cards.
- Image cards.
- Registry cards.
- Editor cards.
- Resource summary cards.

Cards should use restrained surfaces, clear typography and consistent spacing.

### Card interaction requirements

Every interactive card should support:

- A clear primary action.
- A contextual menu.
- An obvious selected state where applicable.
- A meaningful hover state.
- Keyboard accessibility.
- Appropriate loading, error and disabled states.

Cards must not become decorative containers for information that would be better represented through simple typography.

### Layout behaviour

- Use adaptive grids.
- Avoid fixed card widths where possible.
- Support compact and comfortable density.
- Use responsive card arrangements.
- Allow appropriate horizontal space for detail panels.
- Avoid excessive nesting.
- Use dialogs for focused, short operations.
- Use dedicated detail pages for complex configuration.

## 3.4 Global interaction patterns

Common operations should behave consistently.

Examples:

- Create actions open a focused creation workflow.
- Destructive actions require confirmation.
- Long-running operations expose progress.
- Failed operations provide useful explanations.
- Context menus provide secondary operations.
- Search and filtering remain accessible.
- Keyboard shortcuts are documented and configurable where appropriate.

---

# 4. Page specification: Containers

**Navigation position:** First  
**Startup destination:** Yes  
**Primary purpose:** Manage individual containers and manually organized container groups.

## 4.1 Main page

The Containers page is the primary working interface.

It should provide immediate access to local containers without requiring users to navigate through projects or the Hub.

### Header

The page header contains:

- Page title.
- Short contextual description.
- Create Container action.
- Refresh action.
- Optional runtime selector.

### Summary area

Display compact summary cards:

1. Total containers.
2. Running containers.
3. Stopped containers.

Additional status categories can be introduced where useful.

Summary cards should be clickable filters.

### Search and filtering

Provide:

- Container name search.
- Image search.
- Runtime filter.
- Status filter.
- Project association filter.
- Group filter.
- Sorting.
- Card/list presentation toggle.

### Container cards

Each container card displays:

- Container icon.
- Container name.
- Runtime status.
- Image repository and tag.
- Port mappings where available.
- Project/environment association.
- Resource information where available.
- Contextual menu.

The primary card action opens container details.

Secondary actions should be available through a contextual menu or an appropriate hover action area.

### Container card actions

- Start.
- Stop.
- Restart.
- Pause.
- Resume.
- Open terminal.
- View logs.
- Inspect.
- Delete.

Only valid actions should be available for the current container state.

## 4.2 Manual container groups

Users must be able to organize independent containers into manual groups.

A manual group is an organizational abstraction, not an orchestration environment.

### Group creation

Users can:

- Create a group.
- Assign a name.
- Choose an optional icon.
- Add existing containers.
- Remove containers.
- Reorder containers.
- Rename the group.
- Delete the group.

Deleting a group must not automatically delete its containers.

### Group presentation

Groups should appear as expandable card sections or grouped card collections.

A group summary displays:

- Group name.
- Total members.
- Running members.
- Stopped members.
- Aggregate resource usage where available.

### Group actions

- Start all eligible members.
- Stop all running members.
- Restart members.
- Open individual members.
- Remove selected members.
- Delete the group.

Manual group lifecycle operations must not pretend to provide service dependency management.

## 4.3 Container detail

The individual container detail screen contains the following sections:

**Overview**

- Container ID.
- Name.
- Image.
- Status.
- Created time.
- Runtime.
- Restart policy.
- Entrypoint.
- Command.

**Logs**

- Live log streaming.
- Search.
- Filtering.
- Timestamp visibility.
- Copy/export.
- Pause/resume stream.

**Terminal**

- Interactive terminal.
- Shell selection where supported.
- Session management.
- Reconnect.
- Multiple terminal sessions where feasible.

**Networking**

- Network memberships.
- IP information.
- Port mappings.
- Network aliases.
- Connection information.

**Storage**

- Bind mounts.
- Named volumes.
- Mount destinations.
- Read/write modes.

**Resources**

- CPU.
- Memory.
- Network activity.
- Disk statistics where available.

**Inspect**

- Structured runtime inspection data.
- Raw JSON view.
- Copy configuration.

## 4.4 Bulk operations

Users should be able to select multiple containers.

A contextual action bar should provide valid bulk operations:

- Start.
- Stop.
- Restart.
- Delete.
- Add to group.
- Remove from group.

Bulk operations must provide individual operation results and must not fail silently when one container cannot be processed.

## 4.5 Acceptance criteria

- Vessels opens into Containers by default.
- Containers can be discovered from the selected runtime.
- Status updates are reflected without requiring application restart.
- Users can create, start, stop and delete containers.
- Users can open logs and terminals.
- Users can create and manage manual groups.
- Deleting a manual group preserves its members.
- Bulk operations report individual failures.
- The interface remains usable with hundreds of containers.

---

# 5. Page specification: Overview

**Purpose:** Provide an all-in-one management centre across the application.

Overview is not a replacement for Containers or Projects. It aggregates their most useful information into one configurable page.

## 5.1 Layout

The Overview page uses a modular card-based layout.

Recommended sections:

1. Runtime health.
2. Container statistics.
3. Environment statistics.
4. Resource monitoring.
5. Active environments.
6. Recent projects.
7. Recent activity.
8. Quick actions.

## 5.2 Runtime status card

Displays:

- Selected runtime.
- Connection status.
- Runtime version.
- Endpoint.
- Connection error where applicable.

Actions:

- Connect.
- Reconnect.
- Open runtime settings.
- View diagnostics.

## 5.3 Container summary cards

Display:

- Total containers.
- Running containers.
- Stopped containers.
- Containers with errors where detectable.

Clicking a summary opens Containers with the relevant filter.

## 5.4 Environment cards

Display project environments across the application.

Each environment card should include:

- Project name.
- Environment name.
- Service count.
- Running service count.
- Overall status.
- Start/stop controls.
- Open environment action.

## 5.5 Resource monitoring

Provide host and runtime resource summaries.

Metrics:

- CPU consumption.
- Memory consumption.
- Disk usage where available.
- Network activity.
- Per-container resource usage.

Clearly distinguish host-wide resource consumption from container-specific consumption.

Resource polling must be configurable and should not unnecessarily consume CPU.

## 5.6 Recent projects

Display recently opened projects as cards.

Each card includes:

- Project name.
- Project directory.
- Active environment.
- Last opened.
- Preferred editor.
- Quick-open action.

## 5.7 Recent activity

Activity entries include:

- Image pulls.
- Image builds.
- Container creation.
- Container deletion.
- Environment startup.
- Environment shutdown.
- Runtime connection changes.
- Failed operations.

Users should be able to inspect the complete error output of a failed operation.

## 5.8 Quick actions

- Create container.
- Pull image.
- Create project.
- Create environment.
- Import Compose file.
- Open Containers.
- Open Hub.

## 5.9 Customization

Future functionality should allow users to:

- Rearrange Overview cards.
- Hide unwanted sections.
- Adjust card density.
- Choose visible resource metrics.

Customization must not compromise the initial usability of the page.

## 5.10 Acceptance criteria

- Overview aggregates resources from all supported application modules.
- Runtime failures are visible.
- Environment status is accurate.
- Quick actions open the appropriate workflow.
- Resource monitoring does not block the interface.
- The page adapts to different window sizes.

---

# 6. Page specification: Hub

**Purpose:** Discover, inspect, pull and configure container images from OCI-compatible registries.

Hub should feel like an image discovery library rather than a registry administration console.

## 6.1 Registry support

Initial registry targets:

- Docker Hub.
- Quay.io.
- GitHub Container Registry.
- GitLab Container Registry.
- Generic OCI registries.

Registry integrations must use supported registry APIs and OCI-compatible distribution mechanisms.

Registry-specific capabilities should be isolated behind provider adapters.

## 6.2 Hub landing page

The landing page contains:

**Search**

A prominent image search field.

Search should support:

- Repository names.
- Image names.
- Publisher names.
- Registry filters.

**Registry selector**

Users can switch between:

- All registries.
- Docker Hub.
- Quay.io.
- GHCR.
- GitLab.
- Custom registries.

**Discovery sections**

- Popular images.
- Recently viewed.
- Recently pulled.
- Saved images.
- Categories.

Potential categories:

- Linux distributions.
- Programming languages.
- Databases.
- Development environments.
- Web servers.
- Infrastructure tools.
- Testing tools.

Popularity and publisher verification must only be displayed when reliable metadata is available.

## 6.3 Image cards

Image cards should show:

- Repository name.
- Publisher.
- Registry identity.
- Short description.
- Latest or recommended tag.
- Architecture availability.
- Image size when known.
- Popularity metadata when available.

Primary action:

**View image**

Secondary actions:

- Pull.
- Save.
- Copy repository reference.

## 6.4 Image detail page

The image detail page includes:

**Overview**

- Description.
- Repository.
- Publisher.
- Registry.
- Documentation links.
- Image metadata.

**Tags**

- Available tags.
- Tag selection.
- Architecture availability.
- Manifest information.

**Layers**

- Layer list.
- Layer sizes.
- Image total size.

**Local status**

- Whether the image is already downloaded.
- Locally available tags.
- Local image ID.
- Last pull information.

**Actions**

- Pull.
- Pull and create container.
- Save configuration.
- Copy image reference.

## 6.5 Image configuration workflow

Image configuration is a dedicated guided creation workflow.

It should use progressive disclosure so inexperienced users are not overwhelmed.

### Step 1: Image selection

- Select registry.
- Select repository.
- Select tag.
- Select architecture.

### Step 2: General configuration

- Container name.
- Working directory.
- Entrypoint.
- Command.
- Restart policy.

### Step 3: Environment

- Add environment variables.
- Edit variables.
- Remove variables.
- Import environment files where accessible.

Sensitive values must not be displayed in plaintext unnecessarily.

### Step 4: Storage

- Add bind mounts.
- Add named volumes.
- Configure mount destinations.
- Select read-only or read/write mode.

### Step 5: Networking

- Publish ports.
- Select network.
- Configure aliases.
- Configure host binding.

### Step 6: Security

- User and group.
- Capabilities.
- Security options.
- Privileged mode, with explicit warning.

### Step 7: Resources

- CPU limits.
- Memory limits.
- Storage limits where supported.
- GPU access where supported by the runtime and host.

### Step 8: Development configuration

- Workspace directory.
- Development user.
- Development-specific environment variables.
- Optional project association.
- Optional editor preference.

### Step 9: Review

Show a complete configuration summary.

Allow users to:

- Create container.
- Pull image and create container.
- Save configuration as a reusable definition.
- Cancel.

## 6.6 Local image library

Hub also provides access to locally available images.

Features:

- List local images.
- Search and filter.
- Inspect metadata.
- Inspect tags.
- View image layers.
- Remove images.
- Prune unused images.
- Build from a Containerfile where supported.

Image removal must warn users if the image is currently used by containers.

## 6.7 Saved image configurations

Users can save image configurations as reusable definitions.

A saved definition is not itself a container or image.

It represents a reusable set of configuration parameters.

Users can:

- Create.
- Rename.
- Edit.
- Duplicate.
- Export.
- Import.
- Create a container from a saved definition.

## 6.8 Acceptance criteria

- Users can browse supported registries.
- Users can search repositories.
- Users can inspect image tags.
- Users can pull images.
- Users can inspect local images.
- Users can configure container creation.
- Configuration can be saved and reused.
- Authentication errors are clearly explained.
- Unsupported registry functionality is reported accurately.

---

# 7. Page specification: Projects

**Purpose:** Manage development workspaces and their associated container environments.

Projects are the primary home of multi-container development workflows.

A project represents a development workspace, not an individual container.

## 7.1 Project library

The Projects landing page uses a card grid.

Each project card contains:

- Project icon.
- Project name.
- Root directory.
- Description where available.
- Number of environments.
- Total services.
- Running services.
- Environment status.
- Preferred editor.
- Last opened.

Primary actions:

- Open project.
- Open in editor.
- Create environment.
- Edit project.
- Remove project from Vessels.

Removing a project registration must not delete its source directory or containers unless the user explicitly requests those operations.

## 7.2 Project creation

Users can:

- Select an existing directory.
- Create a new directory.
- Import an existing project configuration.
- Import a Compose-based project.
- Create an empty project.

Project selection must use a portal-compatible directory selection flow.

## 7.3 Project workspace

Opening a project provides a dedicated workspace.

Recommended internal navigation:

- Overview.
- Environments.
- Services.
- Terminals.
- Logs.
- Storage.
- Networking.
- Configuration.

The project workspace should preserve a clear project identity and environment context.

## 7.4 Project overview

Display:

- Project information.
- Project directory.
- Current environment.
- Environment health.
- Total services.
- Running services.
- Recent operations.
- Preferred editor.
- Quick actions.

Quick actions:

- Open editor.
- Open terminal.
- Start environment.
- Stop environment.
- Create environment.
- Import Compose file.

## 7.5 Environments

A project may contain multiple environments.

Examples:

- Development.
- Testing.
- Staging.
- Local integration.

Each environment contains a collection of service definitions and their runtime instances.

### Environment card

Displays:

- Environment name.
- Description.
- Runtime.
- Service count.
- Running service count.
- Environment status.
- Last operation.

Actions:

- Open.
- Start.
- Stop.
- Restart.
- Edit configuration.
- Duplicate.
- Delete.

Deleting an environment must clearly distinguish between deleting its configuration and deleting its runtime resources.

## 7.6 Service management

A service represents a workload within an environment.

A service definition can specify:

- Image.
- Tag.
- Build context where supported.
- Command.
- Entrypoint.
- Environment variables.
- Ports.
- Volumes.
- Networks.
- Dependencies.
- Health checks.
- Restart policy.
- Resource limits.

Services must be individually controllable.

Users can:

- Start.
- Stop.
- Restart.
- Recreate.
- Inspect.
- Open terminal.
- View logs.
- Edit configuration.

## 7.7 Environment orchestration

Environment operations must support:

- Start all services.
- Stop all services.
- Restart the environment.
- Start selected services.
- Stop selected services.
- Recreate services.
- Resolve service dependencies.
- Display startup progress.
- Display service health.

Vessels should use established Compose-compatible definitions rather than implementing a new orchestration format.

Compose import and export are core requirements.

The application should preserve the user's existing Compose configuration whenever possible.

It must not silently overwrite user-authored configuration.

## 7.8 Environment workspace

Opening an environment launches its dedicated management workspace.

This is one of Vessels' defining features.

### Workspace layout

The workspace contains:

**Environment header**

- Project name.
- Environment name.
- Runtime state.
- Overall health.
- Start/stop/restart controls.
- More actions.

**Service cards**

Every service appears as a card.

Each card displays:

- Service name.
- Image.
- Runtime status.
- Published ports.
- Health state.
- Resource summary where available.

**Service controls**

- Start.
- Stop.
- Restart.
- Logs.
- Terminal.
- Inspect.

**Environment terminals**

The terminal interface supports multiple simultaneous sessions.

Requirements:

- One terminal session per selected service.
- Switch between services.
- Open multiple sessions.
- Rename terminal tabs.
- Close sessions.
- Reconnect.
- Display connection state.
- Copy and paste.
- Keyboard navigation.

Terminal sessions must use the runtime's supported execution interface.

Vessels must not assume that a particular shell exists inside every image.

**Logs**

- Combined environment logs.
- Individual service logs.
- Service filtering.
- Search.
- Timestamp display.
- Live streaming.
- Export.

**Networking**

- Environment networks.
- Service connectivity information.
- Port mappings.
- Aliases.
- Network inspection.

**Storage**

- Named volumes.
- Bind mounts.
- Mount destinations.
- Persistence information.

**Topology**

An optional visual service relationship view.

It should display:

- Services.
- Network relationships.
- Dependencies.
- Exposed ports.
- Health states.

The topology view must supplement the service card layout, not replace it.

## 7.9 Environment definitions

Environments must be reproducible.

Support:

- Compose-compatible definitions.
- Imported Compose projects.
- Generated Compose-compatible definitions.
- Environment-specific overrides where supported.
- Export and import.

The application should clearly identify whether a configuration is:

- Imported.
- Created by Vessels.
- Modified externally.
- Out of sync with runtime state.

## 7.10 Project configuration

Configuration includes:

- Project name.
- Description.
- Root directory.
- Default editor.
- Default environment.
- Project-specific environment variables.
- Project metadata.
- Environment definitions.
- Container workspace mappings.

## 7.11 Project persistence

Projects should be represented by a local Vessels configuration file.

Suggested format:

`vessels.toml`

The configuration should contain project metadata and references to environment definitions.

It should not contain credentials or secrets in plaintext.

Example conceptual structure:

```text
project/
├── vessels.toml
├── compose.yaml
├── compose.dev.yaml
├── .env.example
└── source/
```

Vessels should not require users to reorganize their existing source directories.

## 7.12 Acceptance criteria

- Users can create and import projects.
- Projects can contain multiple environments.
- Environments can contain multiple services.
- Users can start and stop environments.
- Services can be controlled individually.
- Users can access each service terminal.
- Combined logs are available.
- Compose files can be imported and exported.
- Project removal does not accidentally delete source code.
- Environment deletion explains the impact on runtime resources.

---

# 8. Page specification: Editors

**Purpose:** Detect installed host editors and provide development environment integration.

## 8.1 Editor discovery

Vessels should detect editors available on the host.

Potential integrations:

- Visual Studio Code.
- VSCodium.
- Zed.
- Neovim.
- Helix.
- Emacs.
- Other user-configured editors.

Editor discovery must be implemented through an explicit Flatpak-compatible host integration strategy.

Do not assume that the Flatpak can directly inspect arbitrary host executable paths.

## 8.2 Editor cards

Each editor card displays:

- Editor name.
- Application icon.
- Version if available.
- Detection status.
- Launch capability.
- Remote development capability.
- Configuration status.

Possible states:

- Detected.
- Launchable.
- Remote capable.
- Partially supported.
- Unsupported.
- Unavailable.

## 8.3 Editor detail

Configuration includes:

- Application identifier.
- Executable or activation target.
- Launch arguments.
- Version.
- Remote integration method.
- Workspace mapping.
- Default status.

## 8.4 Remote development

Vessels should distinguish local editor launch from remote container development.

Remote development may be implemented through:

- VS Code Remote - Containers or Dev Containers integration.
- VSCodium-compatible remote extensions where supported.
- Editor-specific remote mechanisms.
- SSH-based integration where appropriate.
- Terminal-based editors attached through the runtime.

Vessels should not claim that every editor supports direct container integration.

## 8.5 Project integration

Each project may have its own default editor.

Users should be able to:

- Set a global default.
- Set a project-specific default.
- Open the project locally.
- Open the project through a remote environment.
- Launch an editor against a selected service where supported.

## 8.6 Acceptance criteria

- Installed editors are detected accurately.
- Uninstalled editors are not presented as available.
- Unsupported remote integrations are clearly labelled.
- Users can configure launch preferences.
- Project-specific editor settings override global defaults.

---

# 9. Page specification: Settings

**Purpose:** Configure Vessels globally.

Settings should use COSMIC-native settings conventions and a clear category navigation.

## 9.1 Appearance

- Follow system theme.
- Light theme.
- Dark theme.
- Accent colour integration.
- Interface density.
- Card layout preferences.
- Default page.
- Optional reduced animation.

## 9.2 Runtime

- Default runtime.
- Runtime discovery.
- Connection endpoint.
- Rootless Podman configuration.
- Docker configuration.
- Remote connection configuration.
- Connection timeout.
- Runtime diagnostics.

Users must be able to inspect runtime connection status without leaving Settings.

## 9.3 Registries

- Registry accounts.
- Authentication.
- Default registry.
- Custom registries.
- Credential status.
- Registry connection testing.

Credentials should use an appropriate secure credential store where available.

Never store passwords or access tokens in ordinary project configuration files.

## 9.4 Containers

- Default container naming.
- Default image pull behaviour.
- Default restart policy.
- Default network behaviour.
- Default resource limits.
- Confirmation preferences.

## 9.5 Projects

- Default project directory.
- Project naming preferences.
- Default environment.
- Project registration behaviour.
- Default configuration filename.

## 9.6 Editors

- Default editor.
- Discovery settings.
- Launch preferences.
- Remote development preferences.
- Editor integration diagnostics.

## 9.7 Storage

- Image storage information.
- Volume information.
- Runtime storage information.
- Application cache location.
- Application data location.

Vessels should not claim to change the runtime's storage root unless the runtime explicitly supports that operation.

## 9.8 Resources

- Default CPU limits.
- Default memory limits.
- Monitoring refresh interval.
- Resource display preferences.

## 9.9 Notifications

- Operation completion.
- Operation failure.
- Runtime disconnection.
- Environment state changes.
- Background operation notifications.

## 9.10 Advanced

- Application logs.
- Runtime diagnostics.
- Debug logging.
- Configuration export.
- Configuration import.
- Reset preferences.
- Version information.

---

# 10. Shared application functionality

## 10.1 Global search

Global search should search across the main application resources.

Search categories:

- Containers.
- Groups.
- Projects.
- Environments.
- Services.
- Images.
- Registries.

Search results should provide direct navigation and relevant actions.

## 10.2 Command palette

A keyboard-accessible command palette should support common actions.

Examples:

- Create container.
- Pull image.
- Create project.
- Open project.
- Open environment.
- Start container.
- Stop container.
- Open terminal.
- Search images.

## 10.3 Activity centre

The activity centre should display long-running and recent operations.

Each activity item contains:

- Operation type.
- Resource name.
- Current state.
- Progress where available.
- Start time.
- Completion time.
- Error information.

## 10.4 Operation queue

The operation queue should handle concurrent asynchronous operations.

Examples:

- Multiple image pulls.
- Container creation.
- Environment startup.
- Image builds.
- Image pruning.

The interface must remain responsive while operations are running.

Operations should support cancellation when the underlying runtime operation allows it.

## 10.5 Notifications

Notifications should be concise and actionable.

Examples:

- Image pull completed.
- Container failed to start.
- Environment started.
- Runtime disconnected.
- Editor integration unavailable.

---

# 11. Technical architecture

## 11.1 Architectural principles

The implementation should follow these principles:

- Separate UI from runtime logic.
- Keep runtime integrations modular.
- Separate image, container, project and environment models.
- Use asynchronous operations.
- Avoid blocking the UI thread.
- Make persistent state explicit.
- Provide consistent error handling.
- Keep the core architecture independent of a single runtime.

## 11.2 Recommended technology stack

| Layer | Technology |
|---|---|
| Language | Rust |
| UI | libcosmic |
| Async runtime | Tokio |
| Serialization | Serde |
| Configuration | TOML |
| Local database | SQLite |
| Database access | SQLx or rusqlite |
| OCI integration | OCI Distribution-compatible client |
| Container runtime | Podman and Docker APIs |
| Environment orchestration | Compose-compatible integration |
| Terminal interface | PTY/runtime exec integration |
| Logging | tracing |
| Error handling | thiserror / anyhow |
| Packaging | Flatpak |
| Build tooling | Cargo |

Specific libraries should be evaluated before implementation. Avoid coupling the domain model to one library's API.

## 11.3 Core modules

```text
vessels/
├── application/
│   ├── state/
│   ├── navigation/
│   ├── commands/
│   └── events/
│
├── ui/
│   ├── shell/
│   ├── components/
│   ├── cards/
│   ├── dialogs/
│   ├── forms/
│   ├── pages/
│   │   ├── containers/
│   │   ├── overview/
│   │   ├── hub/
│   │   ├── projects/
│   │   ├── editors/
│   │   └── settings/
│   └── workspace/
│
├── core/
│   ├── models/
│   ├── services/
│   ├── events/
│   ├── operations/
│   └── errors/
│
├── runtime/
│   ├── manager/
│   ├── podman/
│   ├── docker/
│   ├── connection/
│   └── capabilities/
│
├── registry/
│   ├── providers/
│   │   ├── docker_hub/
│   │   ├── quay/
│   │   ├── ghcr/
│   │   ├── gitlab/
│   │   └── generic/
│   ├── authentication/
│   ├── search/
│   └── images/
│
├── projects/
│   ├── manager/
│   ├── configuration/
│   └── discovery/
│
├── environments/
│   ├── manager/
│   ├── compose/
│   ├── services/
│   ├── orchestration/
│   └── topology/
│
├── terminal/
│   ├── session/
│   ├── execution/
│   └── stream/
│
├── editors/
│   ├── discovery/
│   ├── providers/
│   └── remote/
│
├── integration/
│   ├── flatpak/
│   ├── portals/
│   ├── dbus/
│   ├── credentials/
│   └── host/
│
├── storage/
│   ├── database/
│   ├── configuration/
│   └── cache/
│
└── packaging/
    ├── flatpak/
    └── metadata/
```

This is a logical architecture, not a requirement to create every directory immediately.

## 11.4 Runtime abstraction

Vessels must implement a common runtime interface.

Conceptual operations:

```rust
trait ContainerRuntime {
    async fn connect(&self) -> Result<()>;

    async fn list_containers(&self) -> Result<Vec<Container>>;

    async fn inspect_container(
        &self,
        id: &str,
    ) -> Result<ContainerDetails>;

    async fn create_container(
        &self,
        config: ContainerConfig,
    ) -> Result<Container>;

    async fn start_container(&self, id: &str) -> Result<()>;

    async fn stop_container(&self, id: &str) -> Result<()>;

    async fn restart_container(&self, id: &str) -> Result<()>;

    async fn delete_container(&self, id: &str) -> Result<()>;

    async fn stream_logs(&self, id: &str) -> Result<LogStream>;

    async fn exec(&self, id: &str, command: Vec<String>)
        -> Result<ExecSession>;
}
```

The actual implementation should accommodate differences between Docker and Podman.

Unsupported operations must return explicit capability errors rather than silently failing.

## 11.5 Capability detection

Runtime features vary.

Vessels must discover capabilities rather than assuming every runtime supports every feature.

Examples:

- Compose support.
- GPU support.
- Health checks.
- Resource statistics.
- Volume inspection.
- Interactive exec.
- Build operations.

The UI should adapt to the available capabilities.

## 11.6 Environment orchestration

The environment manager should use a Compose-compatible model.

The application should distinguish:

- Environment definition.
- Service definition.
- Runtime container instance.
- Runtime network.
- Runtime volume.

Do not treat a service definition and its container instance as the same object.

The orchestration implementation should be isolated so that Podman Compose and Docker Compose workflows can be supported without rewriting the project interface.

## 11.7 Event-driven operations

Runtime operations should publish application events.

Examples:

- RuntimeConnected.
- RuntimeDisconnected.
- ContainerCreated.
- ContainerStarted.
- ContainerStopped.
- ImagePulled.
- EnvironmentStarted.
- EnvironmentFailed.
- ProjectOpened.

UI components should update based on these events.

This avoids excessive polling and unnecessary full-page refreshes.

---

# 12. Core data model

## 12.1 Image

Represents an OCI image.

Fields:

- ID.
- Registry.
- Repository.
- Tag.
- Digest.
- Architecture.
- Operating system.
- Size.
- Metadata.
- Local availability.

## 12.2 Container

Represents an individual runtime instance.

Fields:

- Runtime ID.
- Runtime provider.
- Name.
- Image reference.
- Status.
- Created timestamp.
- Ports.
- Mounts.
- Networks.
- Resource information.
- Optional project association.
- Optional environment/service association.

## 12.3 ContainerGroup

Represents a manual organizational grouping.

Fields:

- Group ID.
- Name.
- Description.
- Icon.
- Container references.
- Display order.
- Created timestamp.

A group must not automatically own the lifecycle of its containers.

## 12.4 Project

Represents a development workspace.

Fields:

- Project ID.
- Name.
- Description.
- Root directory.
- Default editor.
- Default environment.
- Configuration location.
- Created timestamp.
- Last opened timestamp.

## 12.5 Environment

Represents an orchestrated environment inside a project.

Fields:

- Environment ID.
- Project ID.
- Name.
- Description.
- Definition source.
- Compose configuration reference.
- Runtime provider.
- Service definitions.
- Network definitions.
- Volume definitions.
- Last operation.
- Current state.

## 12.6 Service

Represents a workload within an environment.

Fields:

- Service ID.
- Environment ID.
- Service name.
- Image reference.
- Build configuration.
- Command.
- Environment variables.
- Port mappings.
- Mounts.
- Networks.
- Dependencies.
- Health-check configuration.
- Resource constraints.
- Runtime container references.

## 12.7 Editor

Represents a discovered host editor.

Fields:

- Editor ID.
- Name.
- Application identifier.
- Executable or activation reference.
- Version.
- Detection state.
- Launch capability.
- Remote capability.
- User configuration.

## 12.8 Registry

Represents a registry integration.

Fields:

- Registry ID.
- Provider.
- Name.
- Endpoint.
- Authentication reference.
- Connection status.
- Capabilities.

Secrets must be stored separately from ordinary application records.

---

# 13. Flatpak integration and security

This section is a release-critical requirement.

## 13.1 Security philosophy

Vessels should operate with the minimum permissions necessary.

It must not sacrifice Flatpak's sandboxing model merely to simplify implementation.

## 13.2 Runtime socket access

Runtime access must be explicitly configured.

The implementation must evaluate:

- Rootless Podman socket access.
- Docker rootless socket access.
- Docker system socket access.
- Remote runtime endpoints.
- Optional host integration service.

The application must never silently connect to an arbitrary privileged socket.

Connection setup should explain:

- Which runtime is being connected.
- Which endpoint is used.
- Whether the endpoint is privileged.
- What access the runtime grants.
- What permission or setup is required.

## 13.3 Host integration service

If Flatpak limitations make secure runtime or editor integration impractical, Vessels may introduce an optional host-side integration component.

This component should:

- Be separately installable.
- Have a narrow IPC interface.
- Perform only explicitly requested operations.
- Validate incoming requests.
- Avoid arbitrary shell execution.
- Expose diagnostics.
- Be versioned independently.
- Require no unnecessary elevated privileges.

It must not become an unrestricted root daemon.

The host helper is an architectural option, not a mandatory MVP dependency.

## 13.4 Credentials

Registry credentials must be protected.

Requirements:

- Use the desktop credential store where available.
- Avoid plaintext credential persistence.
- Redact secrets from logs.
- Never expose secrets in UI error messages.
- Support credential removal.
- Handle expired credentials gracefully.

## 13.5 Project access

- Use user-selected directory access.
- Preserve filesystem permission boundaries.
- Avoid blanket home-directory access.
- Explain inaccessible bind mounts.
- Avoid silently modifying project files.

## 13.6 Dangerous operations

Require explicit confirmation for:

- Privileged containers.
- Host namespace sharing.
- Destructive volume removal.
- Force container deletion.
- Pruning images.
- Removing runtime resources.
- Potentially destructive environment recreation.

Warnings should explain consequences rather than simply saying that an action is dangerous.

---

# 14. UI/UX design system

## 14.1 Design objective

Vessels should feel like an application that belongs alongside COSMIC Files, COSMIC Settings and COSMIC Terminal.

The interface should be modern without becoming visually excessive.

## 14.2 Card-driven design

Cards should be the primary browsing and selection mechanism.

Required card types:

- Container card.
- Container group card.
- Image card.
- Project card.
- Environment card.
- Service card.
- Editor card.
- Registry card.
- Resource card.
- Activity card.

## 14.3 Card hierarchy

Each card should contain:

1. Identity.
2. Primary information.
3. Status.
4. Relevant metadata.
5. Primary interaction.
6. Secondary actions.

Do not place every available setting directly inside cards.

Complex configuration belongs in detail pages or dialogs.

## 14.4 Visual density

Support two presentation modes where appropriate:

**Comfortable**

- More spacing.
- Larger cards.
- Better for smaller container counts.

**Compact**

- Smaller cards.
- More visible resources.
- Better for large collections.

## 14.5 Status system

Use restrained semantic status indicators.

Suggested states:

- Running.
- Stopped.
- Starting.
- Stopping.
- Unhealthy.
- Failed.
- Unknown.
- Disconnected.

Do not rely on colour alone. Include text and/or an icon.

## 14.6 Empty states

Every page must have a designed empty state.

Examples:

**No containers**

Explain that containers appear here after creation or import.

Offer:

- Create container.
- Explore images in Hub.

**No projects**

Offer:

- Create project.
- Import existing project.

**No registry connection**

Offer:

- Configure registry.
- Browse public registry content where possible.

**No runtime connection**

Explain the missing runtime connection and offer guided setup.

## 14.7 Loading and failure states

- Use skeletons for initial loading where appropriate.
- Use progress indicators for long operations.
- Keep unaffected UI interactive.
- Provide retry actions.
- Preserve user-entered configuration after failures.
- Display actionable error explanations.

## 14.8 Responsive design

The interface must adapt to:

- Compact windows.
- Standard desktop windows.
- Large desktop displays.
- Resized windows.
- COSMIC tiling layouts.

Cards should reflow naturally.

The application must not require a maximized window for basic functionality.

## 14.9 Accessibility

- Keyboard navigation.
- Accessible labels.
- Visible focus states.
- Screen-reader-friendly status.
- Sufficient contrast.
- Non-colour status indicators.
- Scalable interface.
- Respect system accessibility preferences where supported.

---

# 15. Performance requirements

Vessels should remain responsive when managing a large collection of containers and images.

Initial engineering targets:

| Operation | Target |
|---|---|
| Application launch | Under 3 seconds on a typical modern system, excluding runtime startup |
| Local navigation | Under 150 ms for normal cached page transitions |
| Search feedback | Under 200 ms for local indexed results |
| Container status refresh | Configurable; default approximately 3–5 seconds |
| UI interaction | No blocking during runtime operations |
| Log streaming | Incremental streaming |
| Image pulling | Asynchronous with progress |
| Environment startup | Progress reported per service |

These are initial targets, not guarantees across all hardware and runtimes.

## 15.1 Memory

- Avoid unnecessarily duplicating runtime inspection data.
- Limit retained log buffers.
- Support configurable log history.
- Avoid excessive resource polling.
- Clean up terminal sessions.
- Release resources when detail views are closed.

## 15.2 Scalability

The application should remain usable with:

- Hundreds of containers.
- Hundreds of local images.
- Dozens of projects.
- Multiple environments per project.
- Multiple services per environment.

Use pagination, lazy loading or virtualization where appropriate.

---

# 16. Error handling

Every subsystem must produce structured, actionable errors.

Examples:

**Runtime unavailable**

Display the selected runtime and connection endpoint, explain why it is unavailable, and provide a reconnect/setup action.

**Image pull failed**

Display registry, repository, tag and relevant failure reason.

**Container startup failed**

Display container name and runtime error output.

**Environment startup partially failed**

Identify which services started and which failed.

Do not mark an entire environment as healthy if one required service is unhealthy.

**Editor unavailable**

Explain whether the editor is missing, inaccessible or unsupported for remote development.

**Flatpak permission unavailable**

Explain which permission or integration is missing and how the user can resolve it.

Never recommend unrestricted host filesystem access as the default fix.

---

# 17. Development roadmap

Implementation should be divided into incremental, testable milestones.

## Phase 0 — Technical validation

Before committing to the complete application:

- Build a minimal libcosmic Flatpak.
- Validate Wayland operation.
- Validate COSMIC theme integration.
- Test rootless Podman socket connectivity.
- Test Docker connectivity.
- Validate Flatpak filesystem portal access.
- Validate host editor discovery.
- Validate terminal execution.
- Establish packaging and CI.

**Exit condition:** A working Flatpak prototype can connect to at least one supported local runtime through a documented and acceptable permission model.

## Phase 1 — Application foundation

Implement:

- Application shell.
- Sidebar.
- Global state.
- Navigation.
- COSMIC design components.
- Settings persistence.
- Runtime abstraction.
- Connection status.

## Phase 2 — Containers

Implement:

- Container listing.
- Container cards.
- Search and filtering.
- Lifecycle operations.
- Container detail.
- Logs.
- Interactive terminal.
- Manual groups.
- Bulk actions.

## Phase 3 — Hub

Implement:

- Registry provider abstraction.
- Docker Hub integration.
- Quay integration.
- Image search.
- Image details.
- Tag selection.
- Image pulling.
- Local image library.
- Container creation wizard.

## Phase 4 — Projects

Implement:

- Project registration.
- Directory selection.
- Project cards.
- Project details.
- Project configuration.
- Container association.
- Project persistence.

## Phase 5 — Environments

Implement:

- Environment model.
- Compose import.
- Compose-compatible configuration.
- Service definitions.
- Multi-container startup and shutdown.
- Service dependencies.
- Environment status.
- Environment workspace.

## Phase 6 — Integrated environment tools

Implement:

- Multi-terminal interface.
- Combined logs.
- Individual service controls.
- Networking inspection.
- Volume inspection.
- Topology view.
- Resource monitoring.

## Phase 7 — Overview

Implement:

- Runtime health.
- Container statistics.
- Environment summaries.
- Resource cards.
- Recent projects.
- Recent activity.
- Quick actions.

## Phase 8 — Editors

Implement:

- Host editor detection.
- Editor cards.
- Application activation.
- Editor settings.
- Project editor selection.
- Remote development integrations.

## Phase 9 — Release hardening

Implement:

- Flatpak packaging.
- Permissions review.
- Security testing.
- Performance optimization.
- Accessibility testing.
- Error handling.
- Documentation.
- Release metadata.
- Flathub submission preparation.

---

# 18. Testing strategy

## 18.1 Unit tests

Test:

- Configuration parsing.
- Model serialization.
- State transitions.
- Runtime capability detection.
- Registry metadata.
- Project configuration.
- Environment definitions.
- Error conversion.

## 18.2 Integration tests

Test against real Podman and Docker installations where available.

Scenarios:

- Connect to runtime.
- List containers.
- Create container.
- Start and stop container.
- Pull image.
- Inspect image.
- Stream logs.
- Open terminal.
- Create volume.
- Create network.
- Start multi-service environment.
- Shut down environment.

## 18.3 Flatpak tests

Test the actual packaged application rather than relying only on development builds.

Validate:

- Runtime socket permissions.
- Portal access.
- Persistent directory access.
- Registry authentication.
- Host editor launch.
- D-Bus permissions.
- Wayland rendering.
- COSMIC theme integration.

## 18.4 UI tests

Validate:

- Navigation.
- Empty states.
- Loading states.
- Card interaction.
- Dialog workflows.
- Responsive layouts.
- Keyboard navigation.
- Accessibility labels.

## 18.5 Failure testing

Test:

- Runtime disconnect during operation.
- Registry authentication failure.
- Image pull interruption.
- Container startup failure.
- Environment partial failure.
- Permission denial.
- Editor removal.
- Application restart during long operations.

---

# 19. Release acceptance criteria

Vessels 1.0 should not be considered complete until the following conditions are met.

### Application

- Native libcosmic interface.
- Stable Flatpak distribution.
- Correct default navigation to Containers.
- System theme integration.
- Responsive application shell.

### Containers

- List, create, inspect and remove containers.
- Lifecycle operations.
- Logs and terminals.
- Manual groups.
- Bulk operations.

### Hub

- At least Docker Hub and Quay integration.
- Image search and details.
- Image pulling.
- Local image management.
- Container creation from images.

### Projects

- Create and import projects.
- Project configuration.
- Multiple environments.
- Multiple services.
- Compose import/export.
- Environment lifecycle.

### Environment workspace

- Individual service management.
- Multiple interactive terminals.
- Combined logs.
- Service inspection.
- Networking and volume visibility.

### Editors

- Host editor detection.
- Editor launch where supported.
- Project-specific preferences.
- Explicit remote capability reporting.

### Security

- No unnecessary unrestricted filesystem access.
- No silent privileged runtime access.
- Secure credential handling.
- Clear permission diagnostics.
- Safe destructive operation handling.

---

# 20. Future possibilities

These are deliberately outside the initial 1.0 scope.

- Environment templates.
- Shareable development environment definitions.
- Remote container hosts.
- SSH-based remote development.
- Environment cloning.
- Development environment snapshots.
- Environment versioning.
- Advanced image build management.
- GPU-aware environment configuration.
- Additional OCI registries.
- Environment topology editing.
- Collaborative environment sharing.
- Optional COSMIC panel integration.
- Automated environment recovery.

These features should be evaluated individually rather than becoming dependencies of the initial release.

---

# 21. Product principles

The following principles should guide future product decisions.

**1. Containers are runtime objects.**

An individual container exists independently of how users organize it.

**2. Groups are organizational objects.**

Manual groups organize containers without introducing orchestration semantics.

**3. Projects are development workspaces.**

Projects own their configurations and associated development environments.

**4. Environments are orchestration objects.**

Environments contain services and manage their lifecycle as a cohesive unit.

**5. Services are workloads.**

A service definition describes a workload; its running container is a separate runtime object.

**6. The runtime is replaceable.**

Podman and Docker are integrations, not the application architecture itself.

**7. The UI is COSMIC-native.**

The design must follow COSMIC conventions rather than merely imitate their appearance.

**8. Flatpak is a primary platform constraint.**

Security, permissions and host integration must be designed intentionally.

**9. Simplicity is a feature.**

Advanced capabilities should not make ordinary container management complicated.

**10. Existing developer workflows should be respected.**

Vessels should work with existing directories, Compose files, images, editors and runtime configurations.

---

# 22. Final product definition

Vessels is a COSMIC-native, Flatpak-first application for managing containerized development workflows.

It provides:

- A container-first management interface.
- A global operational overview.
- A multi-registry OCI image hub.
- Project-oriented development environments.
- Multi-container orchestration.
- Integrated terminals and logs.
- Host editor integration.
- Podman and Docker compatibility.

Its core workflow is:

**Discover → Configure → Create → Organize → Develop → Manage**

The objective is to make containerized development feel like a natural part of the desktop experience, rather than a collection of disconnected command-line tools and configuration files.

**Vessels — Your development environments, all in one place.**
