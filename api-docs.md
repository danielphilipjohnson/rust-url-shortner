# URL Shortener API Documentation

## Base URL
```
http://localhost:8080
```

## Endpoints

### 1. Health Check
Check if the API is running.

```
GET /
```

**Example Request:**
```bash
curl http://localhost:8080/
```

**Example Response:**
```
✅ URL Shortener is running!
```

---

### 2. Create Short URL
Create a shortened URL from a provided original URL.

```
POST /api/shorturl
```

**Request Body:**
```json
{
    "original_url": "string"
}
```

**Response Body:**
```json
{
    "original_url": "string",
    "short_url": "string",
    "created_at": "timestamp"
}
```

**Example Request:**
```bash
curl -X POST \
  http://localhost:8080/api/shorturl \
  -H 'Content-Type: application/json' \
  -d '{
    "original_url": "https://www.example.com"
}'
```

**Example Response:**
```json
{
    "original_url": "https://www.example.com",
    "short_url": "abc123",
    "created_at": "2025-02-08T20:48:14.123Z"
}
```

---

### 3. Redirect to Original URL
Redirect to the original URL using the short URL.

```
GET /api/shorturl/{short_url}
```

**Parameters:**
- `short_url` (path parameter): The shortened URL code

**Response:**
- Redirects to the original URL
- 404 Not Found if the short URL doesn't exist

**Example Request:**
```bash
curl -L http://localhost:8080/api/shorturl/abc123
```

**Note:** The `-L` flag tells curl to follow redirects. Without it, you'll only see the redirect response.

---

## Error Responses

### Invalid URL Error
```json
{
    "error": "Invalid URL format"
}
```

### Not Found Error
```json
{
    "error": "URL not found"
}
```

### Database Error
```json
{
    "error": "Database operation failed"
}
```

---

## Testing Examples

1. Create a short URL:
```bash
curl -X POST \
  http://localhost:8080/api/shorturl \
  -H 'Content-Type: application/json' \
  -d '{
    "original_url": "https://www.rust-lang.org"
}'
```

2. Try creating with invalid URL:
```bash
curl -X POST \
  http://localhost:8080/api/shorturl \
  -H 'Content-Type: application/json' \
  -d '{
    "original_url": "not-a-valid-url"
}'
```

3. Access non-existent short URL:
```bash
curl -L http://localhost:8080/api/shorturl/nonexistent
```

---

## Response Status Codes

- 200: Successful operation
- 201: Successfully created short URL
- 301: Permanent redirect to original URL
- 400: Bad request (invalid URL format)
- 404: URL not found
- 500: Internal server error

---
