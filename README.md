<div align="center">

# 🌌 Nano OG 🎨

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.79%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/nano-og)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[![Open SASS Discord](https://dcbadge.limes.pink/api/server/b5JbvHW5nv)](https://discord.com/invite/b5JbvHW5nv)

| 🐧 Linux `(Recommended)` | 🪟 Windows |
| :------: | :------: |
| [**Download Executable**](https://github.com/opensass/nano-og/releases/download/v0.0.1/dist.zip) | [**Download Executable**](https://github.com/opensass/nano-og/releases/download/v0.0.1/dist.rar) |
| [**Set Environment Variables**](https://github.com/opensass/nano-og#-setting-up-env-vars) | [**Set Environment Variables**](https://github.com/opensass/nano-og#-setting-up-env-vars) |
| unzip files | unzip files |
| execute `./dist/nano-og` | execute `.\dist\nano-og.exe` |

</div>

## 📐 Architecture

![Arch](https://github.com/user-attachments/assets/9f4da22c-729e-40fd-a4c2-c1e57a8f503d)

## 📸 Demo

| 🐧 Linux `(Recommended)` | 🪟 Windows |
| :------: | :------: |
| <video src="https://github.com/user-attachments/assets/04bee24e-58a8-4845-8eff-bbdeefe3a6af"></video> | <video src="https://github.com/user-attachments/assets/f129e604-460d-4664-8dc4-921045c7e3e5"></video> |

## 🚀 About Nano OG

**Nano OG** is a blazing-fast, AI-powered tool for generating **Open Graph (OG)** images, empowering developers, marketers, and designers to create eye-catching visuals optimized for social media and SEO.

With support for real-time previews, customizable designs, and developer-friendly APIs, Nano OG takes the hassle out of crafting stunning OG images.

> [!IMPORTANT]
> 
> Nano OG leverages **Gemini Nano**, which is currently only supported on Windows:
> 
> 1. **Google Chrome Canary**: Ensure you have the latest version of [**Chrome Canary**](https://www.google.com/chrome/canary/) installed on your machine.
> 2. **Gemini Nano AI Model**: Make sure the latest Gemini AI weights are downloaded to avoid compatibility issues.
> 

## 🖥️ Pre-Built Binaries

Prefer downloading a ready-to-go binary? We've got you covered! Grab the pre-compiled binaries for your platform from the links above and get started immediately.

> **⚡ Key Features of Pre-Built Binaries:**
>
> - Ready-to-use `.exe` or Linux binary.
> - Simple setup for environment variables.
> - Optimized for speed and ease of use.

## 🤓 For the Hardcore Nerds

Prefer compiling everything from scratch? Here's what you'll need to get started.

### 🛠️ Prerequisites

1. **Install `rustup`**:

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

1. **Install `Dioxus CLI`**:

   ```sh
   cargo install dioxus-cli
   ```

1. **Clone the Repository**:

   ```sh
   git clone https://github.com/opensass/nano-og.git
   cd nano-og
   ```

## 🔑 Setting Up Env Vars

Before running Nano OG, configure the necessary environment variables for external services like **MongoDB** and **Pinata**.

### Create an `.env` File

In the project root, copy the example `.env` file:

```sh
cp .env.example .env
```

Edit the `.env` file with your credentials:

```sh
MONGODB_USR=your-mongodb-user
MONGODB_PWD=your-mongodb-password
MONGODB_CLSTR=your-cluster.mongodb.net
MONGODB_DB_NAME=nano-ogs
JWT_SECRET=your-jwt-secret
PINATA_API_KEY=your-pinata-api-key
PINATA_API_SECRET=your-pinata-api-secret
```

- **MongoDB**: Follow [this guide](./MongoDB.md) to set up and connect your database.
- **JWT Secret**: Generate a secure key using OpenSSL:

   ```sh
   openssl rand -hex 128
   ```

- **Pinata API**: Get your API keys from [Pinata's developer portal](https://pinata.cloud/).

## 🚀 Building and Running

Once the environment is set up, you're ready to build and run Nano OG.

### Build the Client

```sh
dx serve
```

Navigate to [http://localhost:3000](http://localhost:3000) to access Nano OG.

## ✅ Features

- **AI-Powered Generation**: Leverage Genini Nano AI for local and fast, high-quality OG images.
- **Customizable Designs**: Tailor images to fit your brand perfectly.
- **Real-Time Previews**: Instant updates as you tweak designs.
- **Lightning-Fast Backend**: Built with Rust for high performance.
- **Secure Storage**: Data is stored safely using MongoDB and JWT authentication.

## 🛠️ Tech Stack

- **Axum**: Rust Backend for performance and security.
- **Dioxus**: Fast, reactive rusty UI framework.
- **MongoDB**: Flexible, scalable data storage.
- **Pinata API**: Simplified media uploads to IPFS.

## 📜 License

Nano OG is licensed under the [MIT License](./LICENSE).
Feel free to use, modify, and distribute this project within the terms of the license.

## 💬 Need Help?

Join the conversation on [Discord](https://discord.com/invite/b5JbvHW5nv) or open an issue on [GitHub](https://github.com/opensass/nano-og/issues).
