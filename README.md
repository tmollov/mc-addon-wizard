# Minecraft Add-On Wizard - [STILL IN DEVELOPMENT] 🛠️✨

> A beginner-friendly, drag & drop tool to create Minecraft add-ons without hassle.

![License](https://img.shields.io/badge/license-MIT-blue)  
![Vue.js](https://img.shields.io/badge/Vue.js-4FC08D?logo=vuedotjs&logoColor=fff)
![Vite](https://img.shields.io/badge/Vite-646CFF?logo=vite&logoColor=fff)  
![Tailwind CSS](https://img.shields.io/badge/Tailwind%20CSS-%2338B2AC.svg?logo=tailwind-css&logoColor=white)  
![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri-24C8D8?logo=tauri&logoColor=fff)

## About

Minecraft Add-On Wizard is an open-source project designed for anyone who wants to create Minecraft add-ons—without diving into complex coding or third-party tools. Mainly targeting beginners and hobbyist, this app helps you drag, drop, and configure your own add-ons using an intuitive interface.

Personally i'm just a hobbyist sharing my work, and anyone is welcome to contribute, suggest features, or help improve it.

## Features

- **Drag & Drop Add-On Creation**: Build entities, items, and blocks visually.  
- **JSON-Based**: All add-ons are generated in standard Minecraft JSON format.  
- **Category Management**: Organize your components (2D, 3D, custom behaviors, etc.).  
- **Beginner-Friendly**: No prior coding knowledge required.  

## Tech-Stack

- **Responsive UI**: Built with TailwindCSS for fast, modern styling.  
- **Vue 3 + Vite**: Lightweight, fast, and easy to extend Web framework.  
- **Tauri**: Small, fast, secure framework for cross-platform desktop applications.

---

## Getting Started

### Prerequisites

In order to run the project you will need to install these:

- [Rust](https://rust-lang.org/)
- [Node.js](https://nodejs.org/) (LTS recommended)  
- [npm](https://www.npmjs.com/) or [yarn](https://yarnpkg.com/)  

### Installation

- Clone this repository:

```bash
git clone https://github.com/yourusername/minecraft-addon-wizard.git
```

- Go to that folder and open terminal in that folder
- Then run these commands:
  
```bash
npm install
npm run dev
```

It will run only the web server.  
If you want to run both web server and desktop application use this command:

```bash
npm run app
```

NOTE: Because Cargo will download and compile packages it can take some time depending on your machine.

#### Gallery

<img src="https://raw.githubusercontent.com/tmollov/mc-addon-wizard/refs/heads/main/gallery/Add-On-Init.png" width="600">
<img src="https://raw.githubusercontent.com/tmollov/mc-addon-wizard/refs/heads/main/gallery/Add-Element.png" width="600">
<img src="https://raw.githubusercontent.com/tmollov/mc-addon-wizard/refs/heads/main/gallery/Item-2D.png" width="600">

<img src="https://raw.githubusercontent.com/tmollov/mc-addon-wizard/refs/heads/main/gallery/Add-Component-All.png" width="600">
<img src="https://raw.githubusercontent.com/tmollov/mc-addon-wizard/refs/heads/main/gallery/Add-Component-Boolean.png" width="600">
<img src="https://raw.githubusercontent.com/tmollov/mc-addon-wizard/refs/heads/main/gallery/Add-Component-Number.png" width="600">
<img src="https://raw.githubusercontent.com/tmollov/mc-addon-wizard/refs/heads/main/gallery/Add-Component-String.png" width="600">
<img src="https://raw.githubusercontent.com/tmollov/mc-addon-wizard/refs/heads/main/gallery/Add-Component-Object.png" width="600">
<img src="https://raw.githubusercontent.com/tmollov/mc-addon-wizard/refs/heads/main/gallery/Add-Component-Search.png" width="600">
