# Frontend Implementation Plan

## Overview
The frontend will be a modern, responsive web application that allows users to browse and filter abilities from the game. It will use a combination of HTMX for dynamic updates, Alpine.js for client-side interactivity, and Tailwind CSS for styling.

## Technical Stack
- **Backend Framework**: Axum (Rust)
- **Templating**: Tera (server-side templating)
- **Frontend Libraries**:
  - HTMX for dynamic updates
  - Alpine.js for client-side interactivity
  - Tailwind CSS for styling

## Project Structure
```
hammer/
├── src/
│   ├── main.rs           # Application entry point
│   ├── config.rs         # Application configuration
│   ├── frontend/         # Frontend-related code
│   │   ├── mod.rs        # Frontend module definition
│   │   ├── routes.rs     # Frontend route handlers
│   │   └── templates/    # Tera templates
│   │       ├── base.html     # Base template with common elements
│   │       ├── abilities/
│   │       │   ├── list.html # Main abilities listing
│   │       │   └── filters.html # Filter component
│   │       └── components/   # Reusable components
│   └── backend/          # Backend-related code
│       ├── mod.rs        # Backend module definition
│       ├── routes.rs     # Backend API routes
│       ├── db.rs         # Database operations
│       ├── ability.rs    # Ability data model
│       ├── error.rs      # Error handling
│       ├── load_abilities.rs  # Ability loading logic
│       ├── read_abilities.rs  # Ability reading logic
│       └── index_abilities.rs # Ability indexing logic
├── static/              # Static assets (to be added)
│   ├── css/            # Tailwind CSS
│   └── js/             # Alpine.js components
```

## Features Implementation

### 1. Ability Listing
- Implement infinite scroll using HTMX
- Display abilities in a responsive grid/table layout
- Each ability card will show:
  - Name
  - Effects (truncated with "show more" option)
  - Wiki link
  - Tags (as pills/badges)

### 2. Tag Filtering System
- Create a tag selector component with the following features:
  - Autocomplete based on predefined tags from tags.md
  - Support for multiple tag selection
  - Visual representation of selected tags
  - Clear individual tags or all tags
- Implement AND/OR logic:
  - Tags within the same group use AND logic
  - Different groups are combined with OR logic
  - Visual representation of the current filter logic

### 3. API Endpoints
```
GET /api/abilities
  Query parameters:
  - page: number
  - per_page: number
  - tags: string[] (comma-separated)
  - filter_logic: "and" | "or"
  Response: JSON with abilities and pagination info

GET /api/tags
  Response: JSON array of available tags
```

### 4. In-Memory Ability Management
- Use the existing `Vec<Ability>` loaded at startup
- Leverage the existing tag-based index (`HashMap<String, Vec<usize>>`) for efficient filtering:
  - The index maps tags to sorted, deduplicated ability indices
  - Provides O(1) lookups for tag-based filtering
  - Already handles deduplication of abilities per tag
- Implement efficient filtering operations:
  - For AND operations: intersect the index vectors
  - For OR operations: union the index vectors
  - Use the sorted nature of index vectors for efficient set operations
- Implement pagination:
  - Apply filters to get the filtered indices
  - Slice the result vector for the current page
  - Map indices back to abilities
- Memory optimization considerations:
  - Keep the full ability list in memory as it's relatively small (~1.8MB)
  - Use the existing index structure which is memory efficient (only stores indices)

### 5. UI Components

#### Base Layout
- Responsive navigation
- Search/filter section
- Main content area
- Footer with links

#### Ability Card Component
- Clean, modern design
- Collapsible effects section
- Tag pills with color coding
- Wiki link button
- Hover effects for better UX

#### Filter Component
- Tag input with autocomplete
- Selected tags display
- Filter logic toggle (AND/OR)
- Clear filters button
- Mobile-responsive design

### 6. Performance Considerations
- Optimize in-memory filtering and pagination
- Lazy loading for images and content
- Optimize HTMX requests with proper debouncing

### 7. Development Phases

#### Phase 1: Basic Setup
- Set up Axum server
- Implement basic routing
- Create base templates
- Set up Tailwind CSS
- Load abilities into memory at startup

#### Phase 2: Core Features
- Implement ability listing with in-memory pagination
- Create basic tag filtering
- Set up in-memory filtering and indexing
- Add infinite scroll

#### Phase 3: Advanced Features
- Implement complex tag filtering logic
- Add advanced UI components
- Optimize performance
- Add error handling

#### Phase 4: Polish
- Add loading states
- Implement error boundaries
- Add animations and transitions
- Final styling and UX improvements

## Next Steps
1. Set up the basic project structure
2. Implement the database schema and queries
3. Create the base template with Tailwind CSS
4. Implement the ability listing component
5. Add the tag filtering system
6. Polish the UI and add final touches

## Notes
- All tag inputs will be validated against the predefined list in tags.md
- Consider implementing proper error handling and loading states
- Plan for future extensibility (e.g., adding more filter options)
