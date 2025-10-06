# API Testing Examples

## Using curl

### Signup
```bash
curl -X POST http://localhost:8000/auth/signup \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "username": "testuser",
    "password": "password123"
  }'
```

### Login
```bash
curl -X POST http://localhost:8000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }'
```

### Get Current User (Protected)
```bash
export TOKEN="your-jwt-token-here"

curl -X GET http://localhost:8000/auth/me \
  -H "Authorization: Bearer $TOKEN"
```

### Create Item (Protected)
```bash
curl -X POST http://localhost:8000/items \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My First Item",
    "description": "This is a test item"
  }'
```

### Get All Items (Protected)
```bash
curl -X GET http://localhost:8000/items \
  -H "Authorization: Bearer $TOKEN"
```

### Get Single Item (Protected)
```bash
export ITEM_ID="uuid-here"

curl -X GET http://localhost:8000/items/$ITEM_ID \
  -H "Authorization: Bearer $TOKEN"
```

### Update Item (Protected)
```bash
curl -X PUT http://localhost:8000/items/$ITEM_ID \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated Title",
    "status": "completed"
  }'
```

### Delete Item (Protected)
```bash
curl -X DELETE http://localhost:8000/items/$ITEM_ID \
  -H "Authorization: Bearer $TOKEN"
```

## Using PowerShell

### Signup
```powershell
$body = @{
    email = "test@example.com"
    username = "testuser"
    password = "password123"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:8000/auth/signup" `
  -Method Post `
  -ContentType "application/json" `
  -Body $body
```

### Login and Save Token
```powershell
$body = @{
    email = "test@example.com"
    password = "password123"
} | ConvertTo-Json

$response = Invoke-RestMethod -Uri "http://localhost:8000/auth/login" `
  -Method Post `
  -ContentType "application/json" `
  -Body $body

$token = $response.token
```

### Get Items (Protected)
```powershell
$headers = @{
    Authorization = "Bearer $token"
}

Invoke-RestMethod -Uri "http://localhost:8000/items" `
  -Method Get `
  -Headers $headers
```
