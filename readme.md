# 🚀 URL Shortener API  
> A high-performance, scalable URL shortening service built with Rust, Axum, and MongoDB.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-000000?style=for-the-badge&logo=rust&logoColor=white)
![MongoDB](https://img.shields.io/badge/MongoDB-4ea94b?style=for-the-badge&logo=mongodb&logoColor=white)

---

## 🌟 Features
✔ **Shorten long URLs into compact, shareable links**  
✔ **Redirect users to original URLs from short links**  
✔ **Retrieve all stored short URLs**  
✔ **Stores data in MongoDB for persistence**  
✔ **Built using Axum for high-performance async handling**  
✔ **Lightweight and scalable with Rust**  

---

## 🛠 Tech Stack

| Technology | Purpose |
|------------|---------|
| **Rust** 🦀 | High-performance systems programming |
| **Axum** ⚡ | Web framework for async handling |
| **MongoDB** 🎃 | NoSQL database for storing shortened URLs |
| **Serde** 🛋 | Serialization/Deserialization |
| **Tokio** 🔄 | Async runtime |

---

## 🚀 Getting Started

### **1⃣ Install Prerequisites**
First, install **Rust** using [Rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then, install **Docker & Docker Compose**:  
🔗 [Docker Installation Guide](https://docs.docker.com/get-docker/)

---

## 🛠 Running the Application

You have **two ways** to run the application:  
1. **Locally with MongoDB in Docker (Recommended for Development)**
2. **Fully in Docker using Docker Compose (For Production & Testing)**

---

### **🔹 1⃣ Running Locally with MongoDB in Docker**
For **faster local development**, run MongoDB inside Docker and run Rust locally:

1. **Start MongoDB in Docker**:
```bash
docker-compose up -d mongodb
```
2. **Update your `.env` file** (if needed):
```ini
MONGODB_URI=mongodb://admin:secret@localhost:27017/url_shortener?authSource=admin
```
3. **Run the Rust application**:
```bash
cargo run
```
Now, the server is running at **`http://localhost:8080`** 🎉

---

### **🔹 2⃣ Running Everything in Docker (Production-Ready)**
To run **both the app and MongoDB inside Docker**, use:

```bash
docker-compose up --build
```
Now, your **Rust API and MongoDB** are running inside Docker, and the API is available at:
```
http://localhost:8080
```

To stop the services:
```bash
docker-compose down
```

---

## 📡 API Endpoints

### **1⃣ Shorten a URL (`POST /api/shorturl`)**
Shortens a given long URL.

#### **Example Request**
```bash
curl -X POST http://localhost:8080/api/shorturl \
     -H "Content-Type: application/json" \
     -d '{"original_url": "https://www.rust-lang.org"}'
```
#### **Example Response**
```json
{
  "short_url": "abc123",
  "original_url": "https://www.rust-lang.org"
}
```

---

### **2⃣ Redirect (`GET /api/shorturl/{short_url}`)**
Redirects to the original URL.

#### **Example Request**
```bash
curl -L http://localhost:8080/api/shorturl/abc123
```
#### **Example Response**
Redirects the user to `https://www.rust-lang.org`.

---

### **3⃣ Get All URLs (`GET /api/shorturls`)**
Retrieves all stored short URLs.

#### **Example Request**
```bash
curl http://localhost:8080/api/shorturls
```
#### **Example Response**
```json
[
  {
    "short_url": "abc123",
    "original_url": "https://www.rust-lang.org"
  },
  {
    "short_url": "xyz789",
    "original_url": "https://github.com"
  }
]
```

---

## 🔍 Error Handling

The API returns structured error responses:

```json
{
  "type": "BadRequest",
  "message": "Invalid URL provided"
}
```

Error types:
- `BadRequest`: Invalid input.
- `NotFound`: Short URL not found.
- `InternalError`: Unexpected server-side failure.

---

## 🤮 Development Tips
- Use `cargo watch` for auto-reloading:
  ```bash
  cargo install cargo-watch
  cargo watch -x run
  ```
- Format code:
  ```bash
  cargo fmt
  ```
- Check code style:
  ```bash
  cargo clippy
  ```

---

## 👥 Contributing
1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

---

## 📝 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 📞 Contact
- Blog: [dev.to/danielphilipjohnson](https://dev.to/danielphilipjohnson)
- GitHub: [github.com/yourusername](https://github.com/yourusername)

