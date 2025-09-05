# Tech4Good Club Website

Welcome to the Tech4Good club website project! This is an open-source project where anyone can contribute to help build our community platform.

## Why Dioxus?

Tech4Good chose `Dioxus` as our club web framework because:
- **Performance**: WebAssembly (WASM) runs fast, typed Rust code in the browser
- **Familiar Syntax**: Uses RSX with React-like syntax, making it approachable for web developers
- **Type Safety**: Rust's strong type system helps catch errors at compile time
- **Innovation**: While most web development focuses on TypeScript/React, we're exploring cutting-edge tools

## Getting Started

### Prerequisites

Before you can contribute, make sure you have:
- [Rust](https://rustup.rs/) installed
- [Node.js and npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) installed
- [Dioxus CLI](https://dioxuslabs.com/learn/0.5/getting_started) installed: `cargo install dioxus-cli`

### Project Structure

```
project/
├─ assets/          # Static assets (images, stylesheets, etc.)
│  └─ tailwind.css  # Generated Tailwind CSS file
├─ src/
│  └─ main.rs       # Application entry point and components
├─ Cargo.toml       # Rust dependencies and project configuration
├─ input.css     # Tailwind input file
└─ tailwind.config.js # Tailwind configuration (if present)
```

### Setup Instructions

1. **Clone the repository**
   ```bash
   git clone [repository-url]
   cd club-website
   ```

2. **Install Tailwind CSS CLI**
   ```bash
   npm install tailwindcss @tailwindcss/cli
   ```

3. **Start Tailwind CSS compiler** (in one terminal)
   ```bash
   npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
   ```
   Keep this running while developing to automatically rebuild styles.

4. **Start the development server** (in another terminal)
   ```bash
   dx serve
   ```

### Platform-Specific Development

Dioxus supports multiple platforms. Use these commands for different targets:

```bash
# Web (default)
dx serve

# Desktop app
dx serve --platform desktop

# Mobile (requires additional setup)
dx serve --platform mobile
```

## Contributing

We welcome contributions from club members and the broader community! Here's how to get involved:

### Areas for Contribution

- **Frontend Components**: Build reusable UI components
- **Styling**: Improve the visual design and user experience
- **Features**: Add new functionality like event calendars, member profiles, etc.
- **Documentation**: Help improve this README and add code comments
- **Testing**: Write tests to ensure code quality

### Development Workflow

1. Create a new branch for your feature: `git checkout -b feature/your-feature-name`
2. Make your changes in `src/main.rs` or create new component files
3. Test your changes with `dx serve`
4. Ensure Tailwind styles compile correctly
5. Submit a pull request with a clear description of your changes

### Code Style

- Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use descriptive component and function names
- Add comments for complex logic
- Keep components small and focused on a single responsibility

### Need Help?

- Check out the [Dioxus documentation](https://dioxuslabs.com/learn/0.5/)
- Ask questions in our club Discord/Slack
- Open an issue if you find bugs or have feature requests

## Resources

- [Dioxus Book](https://dioxuslabs.com/learn/0.5/): Complete guide to Dioxus
- [Tailwind CSS Docs](https://tailwindcss.com/docs): Styling reference
- [Rust Book](https://doc.rust-lang.org/book/): Learn Rust fundamentals

---

**Happy coding! 🚀** Let's build something amazing together for Tech4Good!

