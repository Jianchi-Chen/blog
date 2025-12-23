# Copilot Instructions for myBlog

## Architecture Overview

This is a **Tauri + Vue 3 blog application** with a hybrid architecture:
- **Frontend**: Vue 3 + TypeScript + Naive UI + Vite
- **Backend**: Rust (Tauri) with SQLite database
- **Desktop**: Packaged as native app via Tauri v2
- **Development**: MSW (Mock Service Worker) for API mocking in browser mode

### Key Architecture Points
- **Dual-mode API client**: [src/api/client.ts](src/api/client.ts) uses axios in browser, Tauri invoke commands in desktop app
- **State management**: Pinia stores in [src/stores/](src/stores/) for user, articles, search
- **Database**: SQLite managed via sqlx with migrations in [src-tauri/migrations/](src-tauri/migrations/)
- **Authentication**: JWT-based auth handled by Rust backend, token stored in localStorage and auto-injected via axios interceptors

## Development Workflow

### Running the Application
```bash
# Web development (with MSW mocking)
npm run dev

# Tauri desktop app (connects to Rust backend)
npm run tauri:dev

# Production build
npm run tauri:build
```

### Key Commands
- Type checking: `npm run type-check` (uses vue-tsc)
- Vite build only: `npm run build-only`
- Database migrations auto-run on app startup (see [src-tauri/src/lib.rs](src-tauri/src/lib.rs))

## Project-Specific Conventions

### Frontend Patterns

**Component Communication**:
- Use `defineProps<T>()` and `defineEmits<T>()` with TypeScript generics (no runtime validators)
- Example: [src/components/MarkdownEditor.vue](src/components/MarkdownEditor.vue) uses `modelValue` + `update:modelValue` for v-model binding

**API Requests**:
- **Always** use `client` from [src/api/client.ts](src/api/client.ts), NOT direct axios/fetch
- Token automatically attached via request interceptor
- Route guards in [src/router/index.ts](src/router/index.ts) check `requiresAdmin` meta field

**Type Safety**:
- Use Zod schemas for data validation (see [src/types/article.ts](src/types/article.ts))
- Pattern: `const Schema = z.object({...})` → `type Type = z.infer<typeof Schema>`
- Components use type imports: `import type { Article } from "@/types/article"`

**UI Components**:
- Naive UI auto-imported via unplugin-vue-components (no manual imports needed)
- Use composition API with `<script setup lang="ts">`
- Store initialization: Call `useUserStore().initFromStorage()` in [src/main.ts](src/main.ts) to restore auth state

### Backend (Rust/Tauri) Patterns

**Command Structure**:
- All Tauri commands registered in [src-tauri/src/lib.rs](src-tauri/src/lib.rs) `invoke_handler!`
- Organized by domain: `commands/{auth, articles, comments, users, searches, http}.rs`
- Return type: `Result<T, String>` where error strings shown to frontend

**Database Operations**:
- Use sqlx with compile-time checked queries
- Repository pattern: [src-tauri/src/repositories/](src-tauri/src/repositories/)
- Migrations run automatically in [src-tauri/src/db.rs](src-tauri/src/db.rs) `run_migrations()`
- UUIDs: Use `uuid::Uuid` v7 for IDs (time-sortable)

**Authentication Flow**:
1. Frontend calls `invoke('login', {...})` or uses MSW in browser
2. Rust validates with Argon2, generates JWT
3. Token stored in localStorage, auto-attached to subsequent requests
4. See [src-tauri/AUTH_GUIDE.md](src-tauri/AUTH_GUIDE.md) for detailed auth architecture

**HTTP Proxy Command**:
- `http_request` command in [src-tauri/src/commands/http.rs](src-tauri/src/commands/http.rs) proxies frontend requests
- Automatically injects `__token` field from app state
- Used by `client.ts` when running in Tauri environment

## Critical Integration Points

### Environment Detection
```typescript
// Check if running in Tauri
import { isTauri } from '@tauri-apps/api/core'
if (isTauri()) {
  // Use invoke commands
} else {
  // Use axios (MSW intercepts in dev)
}
```

### File Paths (Tauri)
- Use `appDataDir()` for app-specific storage
- Convert Tauri paths to URLs: `convertFileSrc(path)` for images
- File operations: `@tauri-apps/plugin-fs` (readFile, writeFile)
- Dialogs: `@tauri-apps/plugin-dialog` (open file picker)

### MSW Setup
- Handlers in [src/mocks/handlers.ts](src/mocks/handlers.ts)
- Browser instance in [src/mocks/browser.ts](src/mocks/browser.ts)
- Mock data lives in JavaScript refs for persistence between requests

## Common Tasks

### Adding a New API Endpoint
1. Define type in [src/types/](src/types/) with Zod schema
2. Add Tauri command in [src-tauri/src/commands/](src-tauri/src/commands/)
3. Register command in [src-tauri/src/lib.rs](src-tauri/src/lib.rs) `invoke_handler!`
4. Create API wrapper in [src/api/](src/api/) using `client.get/post/etc`
5. Add MSW handler in [src/mocks/handlers.ts](src/mocks/handlers.ts) for browser testing

### Adding a Database Table
1. Create migration SQL in [src-tauri/migrations/XXXX_*.sql](src-tauri/migrations/)
2. Define model struct in [src-tauri/src/models/](src-tauri/src/models/)
3. Create repository in [src-tauri/src/repositories/](src-tauri/src/repositories/)
4. Expose via commands in [src-tauri/src/commands/](src-tauri/src/commands/)

### Router Guards
- Add `meta: { requiresAdmin: true }` to route definition
- Guard in [src/router/index.ts](src/router/index.ts) `beforeEach` checks `userStore.identity === 'admin'`
- Redirects to `/login` if unauthorized

## Important Notes

- **Don't** modify [components.d.ts](components.d.ts) - auto-generated by unplugin-vue-components
- **Don't** use `&&` in PowerShell commands - use `;` for command chaining
- **Vditor caching**: MarkdownEditor uses unique IDs to enable auto-save (localStorage-based)
- **Lazy loading**: Avatar images use `:lazy="true"` prop in NAvatar components
- **Date formatting**: Use `date-fns` library, imported as `import { format } from 'date-fns'`
